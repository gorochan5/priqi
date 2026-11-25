use std::collections::HashMap;
use std::io::Cursor;

use iced::widget::image::Handle;
use labelize::{DrawerOptions, Renderer, ZplParser};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LabelRenderOptions {
    pub width_mm: f32,
    pub height_mm: f32,
    pub dpi: u32,
}

impl Default for LabelRenderOptions {
    fn default() -> Self {
        Self {
            width_mm: 100.0,
            height_mm: 150.0,
            dpi: 203,
        }
    }
}

impl LabelRenderOptions {
    pub fn to_drawer_options(self) -> DrawerOptions {
        let width = if self.width_mm > 0.0 {
            self.width_mm as f64
        } else {
            100.0
        };
        let height = if self.height_mm > 0.0 {
            self.height_mm as f64
        } else {
            150.0
        };
        let dpmm = match self.dpi {
            203 => 8,
            300 => 12,
            600 => 24,
            0 => 8,
            other => ((other as f32 / 25.4).round() as i32).max(1),
        };

        DrawerOptions {
            label_width_mm: width,
            label_height_mm: height,
            dpmm,
            ..DrawerOptions::default()
        }
    }
}

pub fn render_zpl_to_png(zpl: &str) -> Result<Vec<u8>, String> {
    render_zpl_to_png_with_options(zpl, LabelRenderOptions::default())
}

pub fn render_zpl_to_png_with_options(
    zpl: &str,
    opts: LabelRenderOptions,
) -> Result<Vec<u8>, String> {
    let trimmed = zpl.trim();
    if trimmed.is_empty() {
        return Err("Código ZPL vazio".to_string());
    }

    let mut parser = ZplParser::new();
    let labels = parser
        .parse(trimmed.as_bytes())
        .map_err(|e| format!("{e:?}"))?;
    if labels.is_empty() {
        return Err("Nenhuma etiqueta encontrada no ZPL".to_string());
    }

    let renderer = Renderer::new();
    let drawer_opts = opts.to_drawer_options();
    let mut buf = Cursor::new(Vec::new());
    renderer
        .draw_label_as_png(&labels[0], &mut buf, drawer_opts)
        .map_err(|e| format!("{e:?}"))?;

    Ok(buf.into_inner())
}

pub fn render_zpl_all_to_pngs(
    zpl: &str,
    opts: LabelRenderOptions,
) -> Result<Vec<Vec<u8>>, String> {
    let trimmed = zpl.trim();
    if trimmed.is_empty() {
        return Err("Código ZPL vazio".to_string());
    }

    let mut parser = ZplParser::new();
    let labels = parser
        .parse(trimmed.as_bytes())
        .map_err(|e| format!("{e:?}"))?;
    if labels.is_empty() {
        return Err("Nenhuma etiqueta encontrada no ZPL".to_string());
    }

    let renderer = Renderer::new();
    let drawer_opts = opts.to_drawer_options();
    let mut pngs = Vec::with_capacity(labels.len());

    for label in &labels {
        let mut buf = Cursor::new(Vec::new());
        renderer
            .draw_label_as_png(label, &mut buf, drawer_opts.clone())
            .map_err(|e| format!("{e:?}"))?;
        pngs.push(buf.into_inner());
    }

    Ok(pngs)
}

pub fn render_zpl_all_to_handles(
    zpl: &str,
    opts: LabelRenderOptions,
) -> Result<Vec<Handle>, String> {
    let pngs = render_zpl_all_to_pngs(zpl, opts)?;
    Ok(pngs.into_iter().map(Handle::from_bytes).collect())
}

pub fn render_zpl_to_handle(zpl: &str, rotation_deg: u16) -> Result<Handle, String> {
    let png_bytes = render_zpl_to_png(zpl)?;
    let angle = rotation_deg % 360;
    if angle == 0 {
        return Ok(Handle::from_bytes(png_bytes));
    }

    let dyn_img = image::load_from_memory(&png_bytes).map_err(|e| format!("{e}"))?;
    let rotated = match angle {
        90 => dyn_img.rotate90(),
        180 => dyn_img.rotate180(),
        270 => dyn_img.rotate270(),
        _ => dyn_img,
    };

    let rgba = rotated.into_rgba8();
    Ok(Handle::from_rgba(
        rgba.width(),
        rgba.height(),
        rgba.into_raw(),
    ))
}

fn push_sanitized_zpl_value(out: &mut String, val: &str) {
    for ch in val.chars() {
        if ch != '^' && ch != '~' {
            out.push(ch);
        }
    }
}

/// Substitui tags `[NOME_COLUNA]` pelos valores da linha correspondente do dataset
/// em uma única passagem, evitando re-substituição cruzada entre colunas e
/// removendo caracteres de controle ZPL (`^`, `~`) dos valores inseridos.
pub fn substitute_template(zpl: &str, headers: &[String], row: &[String]) -> String {
    let mut map: HashMap<&str, &str> = HashMap::with_capacity(headers.len());
    for (idx, header) in headers.iter().enumerate() {
        if header.trim().is_empty() {
            continue;
        }
        let val = row.get(idx).map(|s| s.as_str()).unwrap_or("");
        map.entry(header.as_str()).or_insert(val);
    }

    if map.is_empty() {
        return zpl.to_string();
    }

    let mut out = String::with_capacity(zpl.len());
    let bytes = zpl.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'[' {
            let mut j = i + 1;
            let mut closed = false;
            while j < bytes.len() {
                if bytes[j] == b']' {
                    closed = true;
                    break;
                }
                if bytes[j] == b'[' {
                    break;
                }
                j += 1;
            }

            if closed {
                let key = &zpl[i + 1..j];
                if let Some(val) = map.get(key) {
                    push_sanitized_zpl_value(&mut out, val);
                    i = j + 1;
                    continue;
                }
            }

            out.push('[');
            i += 1;
        } else {
            let ch = zpl[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::substitute_template;

    #[test]
    fn cross_column_tags_are_not_resubstituted() {
        let headers = vec!["A".to_string(), "B".to_string()];
        let row = vec!["[B]".to_string(), "FINAL".to_string()];
        let zpl = "^XA^FO10,10^FD[A] / [B]^FS^XZ";
        assert_eq!(
            substitute_template(zpl, &headers, &row),
            "^XA^FO10,10^FD[B] / FINAL^FS^XZ"
        );
    }

    #[test]
    fn missing_row_values_substitute_empty() {
        let headers = vec!["NOME".to_string(), "SKU".to_string()];
        let row = vec!["Produto".to_string()];
        let zpl = "^FD[NOME]-[SKU]^FS";
        assert_eq!(substitute_template(zpl, &headers, &row), "^FDProduto-^FS");
    }

    #[test]
    fn empty_headers_do_not_replace_literal_brackets() {
        let headers = vec!["".to_string(), "   ".to_string(), "COD".to_string()];
        let row = vec!["X".to_string(), "Y".to_string(), "123".to_string()];
        let zpl = "^FD[] [   ] [COD]^FS";
        assert_eq!(
            substitute_template(zpl, &headers, &row),
            "^FD[] [   ] 123^FS"
        );
    }

    #[test]
    fn unclosed_bracket_does_not_break_following_tags() {
        let headers = vec!["COD".to_string()];
        let row = vec!["999".to_string()];
        let zpl = "^FD[ABERTO [COD]^FS";
        assert_eq!(
            substitute_template(zpl, &headers, &row),
            "^FD[ABERTO 999^FS"
        );
    }

    #[test]
    fn zpl_control_chars_are_sanitized_in_values() {
        let headers = vec!["OBS".to_string()];
        let row = vec!["OK^XZ~DG".to_string()];
        let zpl = "^FD[OBS]^FS";
        assert_eq!(substitute_template(zpl, &headers, &row), "^FDOKXZDG^FS");
    }
}
