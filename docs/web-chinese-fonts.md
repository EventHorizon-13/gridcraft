# Web Chinese font build

This fork fixes Chinese tofu in the Web app without changing the workbook engine, host=parent protocol or input handling. Baseline: official v0.9.0, commit a2cc4b42cac6e2c41bae71fba4607300755a0b47.

The browser build cannot discover the visitor's installed fonts. An optional externally supplied AOSP Droid Sans fallback is appended to every font family each time GridCraft constructs its definitions, so cell content, sheet names, formulas, dialogs and language/theme changes retain Chinese glyphs. Existing Latin font priority stays intact. Chinese browser preferences are used when the embedded font is valid; other language logic is retained.

```sh
python3 packaging/web/fetch-chinese-font.py /tmp/gridcraft-craft-fonts
export CRAFT_FONTS_DIR=/tmp/gridcraft-craft-fonts
cargo xtask ci
cargo test -p gridcraft-ui-egui --features web-fonts web_fonts::tests
(cd apps/gridcraft-web && trunk build --release)
```

No font binary belongs in this repository. The fetcher pins the AOSP source commit and checks SHA-256 for both the font and original NOTICE. The Web build must ship `NOTICE-DroidSansFallback.txt` and `font-source.json` alongside the ordinary application licenses. The regression feature only makes the Web font available to native tests; normal native builds continue using installed system fonts.

The fork keeps GridCraft's original application icon. ArtCraft graphic marks are not added to the modified Web build; the About text and README identify it as based on the upstream project.
