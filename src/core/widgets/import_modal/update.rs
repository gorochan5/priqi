use std::path::PathBuf;
use iced::Task;
use iced::widget::text_editor;

use super::models::{ImportConfig, ImportKind, Message, State};

const MAX_FILE_SIZE_BYTES: u64 = 25 * 1024 * 1024;

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Open => {
            state.is_open = true;
            state.file_error = None;
            state.is_hovered = false;
            Task::none()
        }
        Message::Close => {
            state.reset();
            Task::none()
        }
        Message::SelectTab(tab) => {
            state.active_tab = tab;
            state.file_error = None;
            Task::none()
        }
        Message::EditorAction(action) => {
            if action.is_edit() {
                state.selected_file = None;
                state.selected_file_size = None;
            }
            state.content.perform(action);
            state.sync_line_numbers();
            state.file_error = None;
            Task::none()
        }
        Message::OpenFilePicker(kind) => {
            state.kind = Some(kind);
            if state.is_picking_file {
                return Task::none();
            }

            #[cfg(not(target_os = "windows"))]
            {
                state.file_error = Some(
                    "Seleção nativa de arquivos disponível apenas no Windows. Arraste o arquivo ou cole o conteúdo."
                        .to_string(),
                );
                return Task::none();
            }

            #[cfg(target_os = "windows")]
            {
                state.is_picking_file = true;
                state.file_error = None;

                Task::perform(
                    async move {
                        match tokio::task::spawn_blocking(move || pick_file_native(kind)).await {
                            Ok(res) => res,
                            Err(e) => Err(format!("Falha ao abrir seletor de arquivos: {e}")),
                        }
                    },
                    Message::FileSelected,
                )
            }
        }
        Message::FileSelected(result) => {
            state.is_picking_file = false;
            state.is_hovered = false;
            if !state.is_open {
                return Task::none();
            }
            match result {
                Ok(Some(path)) => load_file_into_state(state, path),
                Ok(None) => {}
                Err(err) => {
                    state.file_error = Some(err);
                }
            }
            Task::none()
        }
        Message::FileHovered(hovered) => {
            state.is_hovered = hovered;
            Task::none()
        }
        Message::FileDropped(path) => {
            state.is_hovered = false;
            state.active_tab = super::models::ImportTab::File;
            load_file_into_state(state, path);
            Task::none()
        }
        Message::ImportPressed => Task::none(),
    }
}

fn load_file_into_state(state: &mut State, path: PathBuf) {
    if let Some(kind) = state.kind {
        let allowed = ImportConfig::for_kind(kind).extensions;
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase());
        if !ext.as_deref().is_some_and(|e| allowed.contains(&e)) {
            state.file_error = Some(format!(
                "Formato de arquivo não suportado. Use: {}",
                allowed.join(", ")
            ));
            return;
        }
    }

    let file_size = match std::fs::metadata(&path) {
        Ok(meta) if meta.len() > MAX_FILE_SIZE_BYTES => {
            state.file_error = Some("Arquivo muito grande (máximo 25 MB)".to_string());
            return;
        }
        Err(e) => {
            state.file_error = Some(format!("Erro ao acessar arquivo: {e}"));
            return;
        }
        Ok(meta) => meta.len(),
    };

    if state.kind == Some(ImportKind::Datasets) {
        state.selected_file = Some(path);
        state.selected_file_size = Some(file_size);
        state.content = text_editor::Content::new();
        state.sync_line_numbers();
        state.file_error = None;
        return;
    }

    match std::fs::read(&path) {
        Ok(bytes) => {
            if bytes.len() as u64 > MAX_FILE_SIZE_BYTES {
                state.file_error = Some("Arquivo muito grande (máximo 25 MB)".to_string());
                return;
            }
            let text = String::from_utf8_lossy(&bytes);
            // Remove BOM UTF-8 se presente
            let clean_text = text.strip_prefix('\u{feff}').unwrap_or(&text);
            state.content = text_editor::Content::with_text(clean_text);
            state.sync_line_numbers();
            state.selected_file = Some(path);
            state.selected_file_size = Some(bytes.len() as u64);
            state.file_error = None;
        }
        Err(e) => {
            state.file_error = Some(format!("Erro ao ler arquivo: {e}"));
        }
    }
}

#[cfg(target_os = "windows")]
fn pick_file_native(kind: ImportKind) -> Result<Option<PathBuf>, String> {
    #[repr(C)]
    #[allow(clippy::upper_case_acronyms)]
    struct OPENFILENAMEW {
        l_struct_size: u32,
        hwnd_owner: usize,
        h_instance: usize,
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
        fn GetOpenFileNameW(lpofn: *mut OPENFILENAMEW) -> i32;
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

    const OFN_PATHMUSTEXIST: u32 = 0x00000800;
    const OFN_FILEMUSTEXIST: u32 = 0x00001000;
    const OFN_NOCHANGEDIR: u32 = 0x00000008;
    const OFN_EXPLORER: u32 = 0x00080000;

    let filter_str = match kind {
        ImportKind::Templates => {
            "Arquivos de Etiqueta (*.zpl;*.prn;*.txt)\0*.zpl;*.prn;*.txt\0Todos os Arquivos (*.*)\0*.*\0\0"
        }
        ImportKind::Datasets => {
            "Arquivos CSV (*.csv;*.txt)\0*.csv;*.txt\0Todos os Arquivos (*.*)\0*.*\0\0"
        }
    };
    let title_str = match kind {
        ImportKind::Templates => "Selecionar Arquivo de Etiqueta ZPL\0",
        ImportKind::Datasets => "Selecionar Base de Dados CSV\0",
    };

    let filter_utf16: Vec<u16> = filter_str.encode_utf16().collect();
    let title_utf16: Vec<u16> = title_str.encode_utf16().collect();
    let mut file_buf = vec![0u16; 4096];

    let mut ofn = OPENFILENAMEW {
        l_struct_size: std::mem::size_of::<OPENFILENAMEW>() as u32,
        hwnd_owner: unsafe { GetForegroundWindow() as usize },
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
        flags: OFN_PATHMUSTEXIST | OFN_FILEMUSTEXIST | OFN_NOCHANGEDIR | OFN_EXPLORER,
        n_file_offset: 0,
        n_file_extension: 0,
        lpstr_def_ext: std::ptr::null(),
        l_cust_data: 0,
        lpfn_hook: 0,
        lp_template_name: std::ptr::null(),
        pv_reserved: 0,
        dw_reserved: 0,
        flags_ex: 0,
    };

    let ok = unsafe { GetOpenFileNameW(&mut ofn) };
    if ok != 0 {
        let end = file_buf
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(file_buf.len());
        let path_str = String::from_utf16_lossy(&file_buf[..end]);
        if !path_str.is_empty() {
            return Ok(Some(PathBuf::from(path_str)));
        }
        return Ok(None);
    }

    let err = unsafe { CommDlgExtendedError() };
    if err != 0 {
        tracing::error!("Win32 file dialog error: 0x{err:08x}");
        return Err(format!("Erro ao abrir seletor de arquivos (0x{err:08x})"));
    }
    Ok(None)
}

#[cfg(not(target_os = "windows"))]
fn pick_file_native(_kind: ImportKind) -> Result<Option<PathBuf>, String> {
    Ok(None)
}


