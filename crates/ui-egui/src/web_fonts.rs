//! Optional Web Chinese fallback, supplied through the external craft-fonts build input.
//! Append it to every UI/cell role so Latin typography and workbook formatting keep priority.

use egui::{FontData, FontDefinitions};
use std::sync::Arc;

include!(concat!(env!("OUT_DIR"), "/web_font.rs"));

const KEY: &str = "gridcraft-web-chinese";

/// True only when this build contains a face egui can parse.
pub fn available() -> bool {
    CHINESE_FONT.is_some_and(|bytes| crate::theme::font_parses(bytes, 0))
}

pub(crate) fn install(fonts: &mut FontDefinitions) {
    let Some(bytes) = CHINESE_FONT.filter(|bytes| crate::theme::font_parses(bytes, 0)) else {
        return;
    };
    fonts.font_data.insert(KEY.into(), Arc::new(FontData::from_static(bytes)));
    for chain in fonts.families.values_mut() {
        if !chain.iter().any(|key| key == KEY) {
            chain.push(KEY.into());
        }
    }
}

#[cfg(all(test, feature = "web-fonts"))]
mod tests {
    use super::*;
    use crate::{i18n::Language, theme};
    use egui::{Color32, FontFamily, FontId};

    #[test]
    fn external_chinese_face_is_present_and_valid() {
        assert!(available(), "run packaging/web/fetch-chinese-font.py and set CRAFT_FONTS_DIR");
    }

    #[test]
    fn chinese_glyphs_survive_language_and_theme_font_rebuilds() {
        let ctx = egui::Context::default();
        let sample = "中文电子表格销售金额预算工作表臺灣繁體測試";
        // Rebuild as the app does on startup, workbook font binding and language changes.
        for language in [Language::En, Language::Zh, Language::En] {
            let definitions = theme::font_definitions_with(language, &["Calibri".into()]);
            assert!(definitions.families.values().all(|chain| chain.iter().any(|key| key == KEY)));
            ctx.set_fonts(definitions);
            let mut out = ctx.run_ui(egui::RawInput::default(), |_ui| {});
            out.textures_delta.clear();
            for family in [
                FontFamily::Proportional,
                FontFamily::Monospace,
                FontFamily::Name(theme::UI.into()),
                FontFamily::Name(theme::UI_BOLD.into()),
                FontFamily::Name(theme::CELL.into()),
                FontFamily::Name(theme::CELL_BOLD.into()),
                FontFamily::Name(theme::CELL_ITALIC.into()),
                FontFamily::Name(theme::CELL_BOLD_ITALIC.into()),
                FontFamily::Name(theme::SERIF.into()),
                FontFamily::Name(theme::MONO.into()),
            ] {
                let font = FontId::new(14.0, family.clone());
                let glyphs = |text: &str| {
                    let galley = ctx.fonts_mut(|fonts| fonts.layout_no_wrap(text.into(), font.clone(), Color32::BLACK));
                    galley.rows.iter().flat_map(|row| row.glyphs.iter().map(|glyph| glyph.uv_rect)).collect::<Vec<_>>()
                };
                let missing = glyphs("\u{10ffff}");
                let actual = glyphs(sample);
                assert_eq!(actual.len(), sample.chars().count());
                assert!(actual.iter().all(|glyph| !missing.contains(glyph)), "tofu in {family:?}, {language:?}");
            }
        }
    }
}
