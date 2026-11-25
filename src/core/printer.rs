use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use crate::core::renderer::{
    LabelRenderOptions, render_zpl_all_to_pngs, substitute_template,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrintDestination {
    OsPrinter(String),
    Pdf,
}

impl std::fmt::Display for PrintDestination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OsPrinter(name) => write!(f, "{name}"),
            Self::Pdf => write!(f, "Salvar em PDF"),
        }
    }
}

pub struct RenderedPdfPage {
    pub width_px: u32,
    pub height_px: u32,
    pub page_width_pt: f32,
    pub page_height_pt: f32,
    pub rgb_bytes: Vec<u8>,
    pub copies: u32,
}

pub fn execute_print_job(
    template_zpl: &str,
    headers: &[String],
    rows: &[Vec<String>],
    copies: u32,
    destination: PrintDestination,
    render_opts: LabelRenderOptions,
    rotation_deg: u16,
) -> Result<String, String> {
    if template_zpl.trim().is_empty() {
        return Err("O modelo ZPL está vazio.".to_string());
    }

    let copies = copies.max(1);
    let actual_rows: Vec<Vec<String>> = if rows.is_empty() {
        vec![vec![]]
    } else {
        rows.to_vec()
    };

    match destination {
        PrintDestination::Pdf => {
            let Some(out_path) = pick_pdf_save_path() else {
                return Err("Exportação cancelada.".to_string());
            };

            let angle = rotation_deg % 360;
            let (page_w_mm, page_h_mm) = if angle == 90 || angle == 270 {
                (render_opts.height_mm, render_opts.width_mm)
            } else {
                (render_opts.width_mm, render_opts.height_mm)
            };
            let page_width_pt = page_w_mm * 72.0 / 25.4;
            let page_height_pt = page_h_mm * 72.0 / 25.4;

            let mut pages = Vec::new();
            for row in &actual_rows {
                let resolved = substitute_template(template_zpl, headers, row);
                let pngs = render_zpl_all_to_pngs(&resolved, render_opts)?;

                for png_bytes in pngs {
                    let dyn_img =
                        image::load_from_memory(&png_bytes).map_err(|e| format!("{e}"))?;
                    let rotated = match angle {
                        90 => dyn_img.rotate90(),
                        180 => dyn_img.rotate180(),
                        270 => dyn_img.rotate270(),
                        _ => dyn_img,
                    };
                    let rgb = rotated.into_rgb8();
                    pages.push(RenderedPdfPage {
                        width_px: rgb.width(),
                        height_px: rgb.height(),
                        page_width_pt,
                        page_height_pt,
                        rgb_bytes: rgb.into_raw(),
                        copies,
                    });
                }
            }

            let total_pages: usize = pages.iter().map(|p| p.copies as usize).sum();
            let pdf_bytes = build_multipage_pdf(&pages)?;
            fs::write(&out_path, pdf_bytes)
                .map_err(|e| format!("Erro ao salvar PDF: {e}"))?;

            Ok(format!(
                "PDF salvo com sucesso ({total_pages} página(s)) em {}",
                out_path.display()
            ))
        }
        PrintDestination::OsPrinter(printer_name) => {
            let mut full_zpl = String::new();
            let mut label_blocks_count = 0usize;

            for row in &actual_rows {
                let resolved = substitute_template(template_zpl, headers, row);
                let (with_pq, blocks) = inject_pq_copies(&resolved, copies);
                label_blocks_count += blocks.max(1);
                full_zpl.push_str(&with_pq);
                if !full_zpl.ends_with('\n') {
                    full_zpl.push('\n');
                }
            }

            send_raw_to_windows_printer(&printer_name, full_zpl.as_bytes())?;

            let total_labels = label_blocks_count * copies as usize;
            Ok(format!(
                "{total_labels} etiqueta(s) enviada(s) para {printer_name}!"
            ))
        }
    }
}

/// Injeta `^PQ{copies}` antes de cada `^XZ` (case-insensitive) caso o bloco ainda
/// não contenha uma instrução `^PQ`. Retorna o ZPL resultante e a contagem de blocos `^XZ`.
pub(crate) fn inject_pq_copies(zpl: &str, copies: u32) -> (String, usize) {
    let bytes = zpl.as_bytes();
    let mut out = String::with_capacity(zpl.len() + 32);
    let mut block_start = 0usize;
    let mut blocks = 0usize;
    let mut i = 0usize;

    while i + 3 <= bytes.len() {
        if bytes[i] == b'^'
            && bytes[i + 1].eq_ignore_ascii_case(&b'x')
            && bytes[i + 2].eq_ignore_ascii_case(&b'z')
        {
            let block = &zpl[block_start..i];
            out.push_str(block);
            if !contains_ascii_ignore_case(block, "^PQ") {
                if !block.ends_with('\n') && !block.is_empty() {
                    out.push('\n');
                }
                let _ = writeln!(out, "^PQ{copies}");
            }
            out.push_str(&zpl[i..i + 3]);
            blocks += 1;
            i += 3;
            block_start = i;
        } else {
            let ch = zpl[i..].chars().next().unwrap();
            i += ch.len_utf8();
        }
    }

    if block_start < zpl.len() {
        out.push_str(&zpl[block_start..]);
    }

    (out, blocks)
}

fn contains_ascii_ignore_case(haystack: &str, needle: &str) -> bool {
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    if n.is_empty() || h.len() < n.len() {
        return false;
    }
    h.windows(n.len()).any(|w| w.eq_ignore_ascii_case(n))
}

pub fn build_multipage_pdf(pages: &[RenderedPdfPage]) -> Result<Vec<u8>, String> {
    if pages.is_empty() {
        return Err("Nenhuma página para exportar em PDF.".to_string());
    }

    let mut pdf = Vec::<u8>::new();
    pdf.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

    // Objeto 1: Catalog, Objeto 2: Pages
    let mut offsets: Vec<usize> = vec![0; 3]; // índice 0 não usado; 1 e 2 preenchidos ao montar
    let mut next_obj_id = 3usize;

    // Pré-calcula IDs: para cada RenderedPdfPage, 1 XObject de imagem; para cada cópia, 1 Content + 1 Page
    struct PageEntry {
        page_obj_id: usize,
        content_obj_id: usize,
        img_obj_id: usize,
        page_width_pt: f32,
        page_height_pt: f32,
    }

    let mut img_obj_ids = Vec::with_capacity(pages.len());
    let mut page_entries = Vec::new();

    for page in pages {
        let img_obj_id = next_obj_id;
        next_obj_id += 1;
        img_obj_ids.push(img_obj_id);

        for _ in 0..page.copies.max(1) {
            let content_obj_id = next_obj_id;
            let page_obj_id = next_obj_id + 1;
            next_obj_id += 2;
            page_entries.push(PageEntry {
                page_obj_id,
                content_obj_id,
                img_obj_id,
                page_width_pt: page.page_width_pt,
                page_height_pt: page.page_height_pt,
            });
        }
    }

    offsets.resize(next_obj_id, 0);

    // Objeto 1: Catalog
    offsets[1] = pdf.len();
    pdf.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

    // Objeto 2: Pages
    offsets[2] = pdf.len();
    let mut kids = String::new();
    for (idx, entry) in page_entries.iter().enumerate() {
        if idx > 0 {
            kids.push(' ');
        }
        let _ = write!(kids, "{} 0 R", entry.page_obj_id);
    }
    let pages_obj = format!(
        "2 0 obj\n<< /Type /Pages /Kids [ {kids} ] /Count {} >>\nendobj\n",
        page_entries.len()
    );
    pdf.extend_from_slice(pages_obj.as_bytes());

    // Objetos de Imagem XObject
    for (page, &img_obj_id) in pages.iter().zip(img_obj_ids.iter()) {
        offsets[img_obj_id] = pdf.len();
        let header = format!(
            "{img_obj_id} 0 obj\n<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Length {} >>\nstream\n",
            page.width_px,
            page.height_px,
            page.rgb_bytes.len()
        );
        pdf.extend_from_slice(header.as_bytes());
        pdf.extend_from_slice(&page.rgb_bytes);
        pdf.extend_from_slice(b"\nendstream\nendobj\n");
    }

    // Objetos de Conteúdo e Página
    for entry in &page_entries {
        let stream_body = format!(
            "q\n{:.2} 0 0 {:.2} 0 0 cm\n/Im0 Do\nQ\n",
            entry.page_width_pt, entry.page_height_pt
        );
        offsets[entry.content_obj_id] = pdf.len();
        let content_obj = format!(
            "{} 0 obj\n<< /Length {} >>\nstream\n{stream_body}endstream\nendobj\n",
            entry.content_obj_id,
            stream_body.len()
        );
        pdf.extend_from_slice(content_obj.as_bytes());

        offsets[entry.page_obj_id] = pdf.len();
        let page_obj = format!(
            "{} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {:.2} {:.2}] /Resources << /XObject << /Im0 {} 0 R >> >> /Contents {} 0 R >>\nendobj\n",
            entry.page_obj_id,
            entry.page_width_pt,
            entry.page_height_pt,
            entry.img_obj_id,
            entry.content_obj_id
        );
        pdf.extend_from_slice(page_obj.as_bytes());
    }

    let xref_offset = pdf.len();
    let xref_header = format!("xref\n0 {next_obj_id}\n0000000000 65535 f \n");
    pdf.extend_from_slice(xref_header.as_bytes());

    for &offset in &offsets[1..next_obj_id] {
        let line = format!("{offset:010} 00000 n \n");
        pdf.extend_from_slice(line.as_bytes());
    }

    let trailer = format!(
        "trailer\n<< /Size {next_obj_id} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n"
    );
    pdf.extend_from_slice(trailer.as_bytes());

    Ok(pdf)
}

#[cfg(target_os = "windows")]
pub fn enumerate_windows_printers() -> Vec<String> {
    use std::ptr;

    #[repr(C)]
    #[allow(dead_code, clippy::upper_case_acronyms)]
    struct PRINTER_INFO_2W {
        p_server_name: *mut u16,
        p_printer_name: *mut u16,
        p_share_name: *mut u16,
        p_port_name: *mut u16,
        p_driver_name: *mut u16,
        p_comment: *mut u16,
        p_location: *mut u16,
        p_dev_mode: *mut core::ffi::c_void,
        p_sep_file: *mut u16,
        p_print_processor: *mut u16,
        p_datatype: *mut u16,
        p_parameters: *mut u16,
        p_security_descriptor: *mut core::ffi::c_void,
        attributes: u32,
        priority: u32,
        default_priority: u32,
        start_time: u32,
        until_time: u32,
        status: u32,
        c_jobs: u32,
        average_ppm: u32,
    }

    #[repr(C)]
    #[allow(dead_code, clippy::upper_case_acronyms)]
    struct PRINTER_INFO_4W {
        p_printer_name: *mut u16,
        p_server_name: *mut u16,
        attributes: u32,
    }

    #[link(name = "winspool")]
    unsafe extern "system" {
        fn EnumPrintersW(
            flags: u32,
            name: *const u16,
            level: u32,
            p_printer_enum: *mut u8,
            cb_buf: u32,
            pcb_needed: *mut u32,
            pc_returned: *mut u32,
        ) -> i32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLastError() -> u32;
    }

    unsafe fn utf16_ptr_to_string(ptr: *const u16) -> Option<String> {
        if ptr.is_null() {
            return None;
        }
        let mut len = 0usize;
        while unsafe { *ptr.add(len) } != 0 {
            len += 1;
        }
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        let name = String::from_utf16_lossy(slice);
        if name.trim().is_empty() {
            None
        } else {
            Some(name)
        }
    }

    fn try_enum_printers(flags: u32, level: u32) -> Vec<String> {
        let mut needed: u32 = 0;
        let mut returned: u32 = 0;

        let sizing_ok = unsafe {
            EnumPrintersW(
                flags,
                ptr::null(),
                level,
                ptr::null_mut(),
                0,
                &mut needed,
                &mut returned,
            )
        };

        if needed == 0 {
            if sizing_ok == 0 {
                let err = unsafe { GetLastError() };
                if err != 0 {
                    tracing::warn!(
                        flags,
                        level,
                        error = err,
                        "EnumPrintersW sizing call failed"
                    );
                }
            }
            return Vec::new();
        }

        let mut buf = vec![0u64; (needed as usize).div_ceil(8)];
        let cb_buf = (buf.len() * std::mem::size_of::<u64>()) as u32;
        let ok = unsafe {
            EnumPrintersW(
                flags,
                ptr::null(),
                level,
                buf.as_mut_ptr() as *mut u8,
                cb_buf,
                &mut needed,
                &mut returned,
            )
        };

        if ok == 0 {
            let err = unsafe { GetLastError() };
            tracing::warn!(flags, level, error = err, "EnumPrintersW failed");
            return Vec::new();
        }

        if returned == 0 {
            return Vec::new();
        }

        let count = returned as usize;
        let mut printers = Vec::with_capacity(count);
        match level {
            2 => {
                let infos = unsafe {
                    std::slice::from_raw_parts(buf.as_ptr() as *const PRINTER_INFO_2W, count)
                };
                for info in infos {
                    if let Some(name) = unsafe { utf16_ptr_to_string(info.p_printer_name) }
                        && !printers.contains(&name)
                    {
                        printers.push(name);
                    }
                }
            }
            4 => {
                let infos = unsafe {
                    std::slice::from_raw_parts(buf.as_ptr() as *const PRINTER_INFO_4W, count)
                };
                for info in infos {
                    if let Some(name) = unsafe { utf16_ptr_to_string(info.p_printer_name) }
                        && !printers.contains(&name)
                    {
                        printers.push(name);
                    }
                }
            }
            _ => {}
        }

        printers
    }

    const PRINTER_ENUM_LOCAL: u32 = 0x0000_0002;
    const PRINTER_ENUM_CONNECTIONS: u32 = 0x0000_0004;

    let attempts = [
        (PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS, 2),
        (PRINTER_ENUM_LOCAL | PRINTER_ENUM_CONNECTIONS, 4),
        (PRINTER_ENUM_LOCAL, 2),
        (PRINTER_ENUM_LOCAL, 4),
    ];

    for (flags, level) in attempts {
        let printers = try_enum_printers(flags, level);
        if !printers.is_empty() {
            return printers;
        }
    }

    Vec::new()
}

#[cfg(not(target_os = "windows"))]
pub fn enumerate_windows_printers() -> Vec<String> {
    Vec::new()
}

#[cfg(target_os = "windows")]
fn send_raw_to_windows_printer(printer_name: &str, data: &[u8]) -> Result<(), String> {
    use std::ptr;

    #[repr(C)]
    struct DOC_INFO_1W {
        p_doc_name: *mut u16,
        p_output_file: *mut u16,
        p_datatype: *mut u16,
    }

    #[link(name = "winspool")]
    unsafe extern "system" {
        fn OpenPrinterW(
            p_printer_name: *const u16,
            ph_printer: *mut isize,
            p_default: *const u8,
        ) -> i32;
        fn StartDocPrinterW(
            h_printer: isize,
            level: u32,
            p_doc_info: *const DOC_INFO_1W,
        ) -> u32;
        fn StartPagePrinter(h_printer: isize) -> i32;
        fn WritePrinter(
            h_printer: isize,
            p_buf: *const u8,
            cb_buf: u32,
            pc_written: *mut u32,
        ) -> i32;
        fn EndPagePrinter(h_printer: isize) -> i32;
        fn EndDocPrinter(h_printer: isize) -> i32;
        fn ClosePrinter(h_printer: isize) -> i32;
    }

    let printer_w: Vec<u16> = printer_name.encode_utf16().chain(Some(0)).collect();
    let mut h_printer: isize = 0;

    let opened = unsafe { OpenPrinterW(printer_w.as_ptr(), &mut h_printer, ptr::null()) };
    if opened == 0 || h_printer == 0 {
        return Err(format!(
            "Não foi possível abrir a impressora '{printer_name}' (código {}).",
            std::io::Error::last_os_error()
        ));
    }

    let mut doc_name_w: Vec<u16> = "Priqi ZPL Print Job".encode_utf16().chain(Some(0)).collect();
    let mut datatype_w: Vec<u16> = "RAW".encode_utf16().chain(Some(0)).collect();
    let doc_info = DOC_INFO_1W {
        p_doc_name: doc_name_w.as_mut_ptr(),
        p_output_file: ptr::null_mut(),
        p_datatype: datatype_w.as_mut_ptr(),
    };

    let job_id = unsafe { StartDocPrinterW(h_printer, 1, &doc_info) };
    if job_id == 0 {
        let err = std::io::Error::last_os_error();
        unsafe { ClosePrinter(h_printer) };
        return Err(format!("Falha em StartDocPrinterW: {err}"));
    }

    let page_started = unsafe { StartPagePrinter(h_printer) };
    if page_started == 0 {
        let err = std::io::Error::last_os_error();
        unsafe {
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
        }
        return Err(format!("Falha em StartPagePrinter: {err}"));
    }

    let mut total_written = 0usize;
    while total_written < data.len() {
        let chunk = &data[total_written..];
        let to_write = chunk.len().min(u32::MAX as usize) as u32;
        let mut written: u32 = 0;
        let ok = unsafe { WritePrinter(h_printer, chunk.as_ptr(), to_write, &mut written) };
        if ok == 0 || written == 0 {
            let err = std::io::Error::last_os_error();
            unsafe {
                EndPagePrinter(h_printer);
                EndDocPrinter(h_printer);
                ClosePrinter(h_printer);
            }
            return Err(format!("Falha em WritePrinter: {err}"));
        }
        total_written += written as usize;
    }

    unsafe {
        EndPagePrinter(h_printer);
        EndDocPrinter(h_printer);
        ClosePrinter(h_printer);
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn send_raw_to_windows_printer(_printer_name: &str, _data: &[u8]) -> Result<(), String> {
    Err("Impressão RAW via spooler é suportada apenas no Windows.".to_string())
}

#[cfg(target_os = "windows")]
fn pick_pdf_save_path() -> Option<PathBuf> {
    #[repr(C)]
    #[allow(clippy::upper_case_acronyms)]
    struct OPENFILENAMEW {
        l_struct_size: u32,
        hwnd_owner: isize,
        h_instance: isize,
        lpstr_filter: *const u16,
        lpstr_custom_filter: *mut u16,
        n_max_cust_filter: u32,
        n_filter_index: u32,
        lpstr_file: *mut u16,
        n_max_file: u32,
        lpstr_file_title: *mut u16,
        n_max_file_title: u32,
        lpstr_initial_dir: *const u16,
        lpstr_title: *const u16,
        flags: u32,
        n_file_offset: u16,
        n_file_extension: u16,
        lpstr_def_ext: *const u16,
        l_cust_data: usize,
        lpfn_hook: usize,
        lp_template_name: *const u16,
        pv_reserved: usize,
        dw_reserved: u32,
        flags_ex: u32,
    }

    #[link(name = "comdlg32")]
    unsafe extern "system" {
        fn GetSaveFileNameW(lpofn: *mut OPENFILENAMEW) -> i32;
        fn CommDlgExtendedError() -> u32;
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        fn GetForegroundWindow() -> isize;
    }

    #[link(name = "ole32")]
    unsafe extern "system" {
        fn CoInitializeEx(pv_reserved: *mut core::ffi::c_void, dw_co_init: u32) -> i32;
        fn CoUninitialize();
    }

    struct ComGuard {
        initialized: bool,
    }

    impl Drop for ComGuard {
        fn drop(&mut self) {
            if self.initialized {
                unsafe { CoUninitialize() };
            }
        }
    }

    const COINIT_APARTMENTTHREADED: u32 = 0x2;
    const COINIT_DISABLE_OLE1DDE: u32 = 0x4;
    let hr = unsafe {
        CoInitializeEx(
            std::ptr::null_mut(),
            COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE,
        )
    };
    let _com_guard = ComGuard {
        initialized: hr >= 0,
    };

    const OFN_OVERWRITEPROMPT: u32 = 0x0000_0002;
    const OFN_NOCHANGEDIR: u32 = 0x0000_0008;
    const OFN_PATHMUSTEXIST: u32 = 0x0000_0800;
    const OFN_EXPLORER: u32 = 0x0008_0000;

    let filter_utf16: Vec<u16> = "Arquivos PDF (*.pdf)\0*.pdf\0\0".encode_utf16().collect();
    let title_utf16: Vec<u16> = "Salvar Etiquetas em PDF\0".encode_utf16().collect();
    let def_ext_utf16: Vec<u16> = "pdf\0".encode_utf16().collect();

    let mut file_buf = vec![0u16; 4096];
    let default_name: Vec<u16> = "etiquetas.pdf".encode_utf16().collect();
    file_buf[..default_name.len()].copy_from_slice(&default_name);

    let hwnd_owner = unsafe { GetForegroundWindow() };

    let mut ofn = OPENFILENAMEW {
        l_struct_size: std::mem::size_of::<OPENFILENAMEW>() as u32,
        hwnd_owner,
        h_instance: 0,
        lpstr_filter: filter_utf16.as_ptr(),
        lpstr_custom_filter: std::ptr::null_mut(),
        n_max_cust_filter: 0,
        n_filter_index: 1,
        lpstr_file: file_buf.as_mut_ptr(),
        n_max_file: file_buf.len() as u32,
        lpstr_file_title: std::ptr::null_mut(),
        n_max_file_title: 0,
        lpstr_initial_dir: std::ptr::null(),
        lpstr_title: title_utf16.as_ptr(),
        flags: OFN_OVERWRITEPROMPT | OFN_PATHMUSTEXIST | OFN_NOCHANGEDIR | OFN_EXPLORER,
        n_file_offset: 0,
        n_file_extension: 0,
        lpstr_def_ext: def_ext_utf16.as_ptr(),
        l_cust_data: 0,
        lpfn_hook: 0,
        lp_template_name: std::ptr::null(),
        pv_reserved: 0,
        dw_reserved: 0,
        flags_ex: 0,
    };

    let ok = unsafe { GetSaveFileNameW(&mut ofn) };
    if ok != 0 {
        let end = file_buf
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(file_buf.len());
        let path_str = String::from_utf16_lossy(&file_buf[..end]);
        if !path_str.is_empty() {
            return Some(PathBuf::from(path_str));
        }
        return None;
    }

    let err = unsafe { CommDlgExtendedError() };
    if err != 0 {
        tracing::error!("Win32 file dialog error: 0x{err:08x}");
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn pick_pdf_save_path() -> Option<PathBuf> {
    Some(PathBuf::from("etiquetas.pdf"))
}

#[cfg(test)]
mod tests {
    use super::{RenderedPdfPage, build_multipage_pdf, inject_pq_copies};

    #[test]
    fn inject_pq_handles_lowercase_xz_and_skips_existing_pq() {
        let zpl = "^xa^fo10,10^fdA^fs^xz\n^XA^FO10,10^FDB^FS^PQ5^XZ";
        let (out, blocks) = inject_pq_copies(zpl, 3);
        assert_eq!(blocks, 2);
        assert!(out.contains("^PQ3\n^xz"));
        assert_eq!(out.matches("^PQ").count(), 2);
        assert!(out.contains("^PQ5^XZ"));
    }

    #[test]
    fn multipage_pdf_has_valid_header_pages_and_eof() {
        let pages = vec![
            RenderedPdfPage {
                width_px: 2,
                height_px: 2,
                page_width_pt: 283.46,
                page_height_pt: 425.20,
                rgb_bytes: vec![255; 12],
                copies: 2,
            },
            RenderedPdfPage {
                width_px: 2,
                height_px: 2,
                page_width_pt: 283.46,
                page_height_pt: 425.20,
                rgb_bytes: vec![0; 12],
                copies: 1,
            },
        ];

        let pdf = build_multipage_pdf(&pages).unwrap();
        let text = String::from_utf8_lossy(&pdf);
        assert!(text.starts_with("%PDF-1.4\n"));
        assert!(text.contains("/Count 3 >>"));
        assert!(text.ends_with("%%EOF\n"));
    }
}
