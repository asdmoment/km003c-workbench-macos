use eframe::egui;
use std::sync::atomic::{AtomicU8, Ordering};

use crate::preferences::SkinId;

// KM003C's instrument palette follows one rule: containers are grayscale and
// color belongs to measured channels. Keeping the tokens here prevents local
// widgets from slowly reintroducing decorative blues, greens or oranges.
pub(crate) const BACKPLANE: egui::Color32 = egui::Color32::from_rgb(0x0D, 0x11, 0x17);
pub(crate) const PANEL: egui::Color32 = egui::Color32::from_rgb(0x16, 0x1B, 0x22);
pub(crate) const PANEL_RAISED: egui::Color32 = egui::Color32::from_rgb(0x1C, 0x21, 0x28);
pub(crate) const DIVIDER: egui::Color32 = egui::Color32::from_rgb(0x30, 0x36, 0x3D);
pub(crate) const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(0xE6, 0xED, 0xF3);
pub(crate) const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(0x91, 0x98, 0xA1);
pub(crate) const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(0x6E, 0x76, 0x81);
pub(crate) const VOLTAGE: egui::Color32 = egui::Color32::from_rgb(0x58, 0xA6, 0xFF);
pub(crate) const CURRENT: egui::Color32 = egui::Color32::from_rgb(0x3F, 0xB9, 0x50);
pub(crate) const POWER: egui::Color32 = egui::Color32::from_rgb(0xD2, 0x99, 0x22);
pub(crate) const ENERGY: egui::Color32 = egui::Color32::from_rgb(0xF7, 0x6E, 0xB6);
pub(crate) const CAPACITY: egui::Color32 = egui::Color32::from_rgb(0xA3, 0x7A, 0xF2);
pub(crate) const RECORDING: egui::Color32 = egui::Color32::from_rgb(0xF8, 0x51, 0x49);

/// Container and text colors for a complete UI skin.  Measurement colors are
/// intentionally not part of this palette: voltage/current/power keep their
/// stable semantic colors when the surrounding skin changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SkinPalette {
    pub(crate) backplane: egui::Color32,
    pub(crate) panel: egui::Color32,
    pub(crate) panel_raised: egui::Color32,
    pub(crate) divider: egui::Color32,
    pub(crate) text_primary: egui::Color32,
    pub(crate) text_secondary: egui::Color32,
    pub(crate) text_muted: egui::Color32,
    pub(crate) accent: egui::Color32,
    pub(crate) accent_soft: egui::Color32,
    pub(crate) hovered: egui::Color32,
    pub(crate) active: egui::Color32,
    /// Opacity of the optional full-window wallpaper. Industrial keeps this
    /// at zero so the default theme remains unchanged.
    pub(crate) wallpaper_alpha: u8,
    /// Dark veil drawn above the wallpaper before translucent panels are
    /// painted. This keeps measurements readable over a photographic image.
    pub(crate) wallpaper_overlay: egui::Color32,
}

impl SkinPalette {
    pub(crate) fn for_skin(skin: SkinId) -> Self {
        match skin {
            SkinId::Industrial => Self {
                backplane: BACKPLANE,
                panel: PANEL,
                panel_raised: PANEL_RAISED,
                divider: DIVIDER,
                text_primary: TEXT_PRIMARY,
                text_secondary: TEXT_SECONDARY,
                text_muted: TEXT_MUTED,
                accent: VOLTAGE,
                accent_soft: TEXT_SECONDARY,
                hovered: egui::Color32::from_rgb(0x24, 0x2A, 0x32),
                active: egui::Color32::from_rgb(0x2A, 0x30, 0x38),
                wallpaper_alpha: 0,
                wallpaper_overlay: egui::Color32::TRANSPARENT,
            },
            SkinId::CleanAnime => Self {
                // Low-saturation blue-gray surfaces keep the measurement
                // colors readable while giving the alternate skin its own
                // atmosphere.
                // The alpha lets the requested character wallpaper remain
                // visible across the whole workbench instead of appearing
                // only in a small decorative slot.
                backplane: egui::Color32::from_rgba_unmultiplied(0x0F, 0x15, 0x22, 0x90),
                panel: egui::Color32::from_rgba_unmultiplied(0x17, 0x21, 0x30, 0xB8),
                panel_raised: egui::Color32::from_rgba_unmultiplied(0x20, 0x2C, 0x3D, 0xD8),
                divider: egui::Color32::from_rgba_unmultiplied(0x3A, 0x4A, 0x60, 0xE0),
                text_primary: egui::Color32::from_rgb(0xEF, 0xF6, 0xFF),
                text_secondary: egui::Color32::from_rgb(0xAF, 0xC1, 0xD9),
                text_muted: egui::Color32::from_rgb(0x77, 0x89, 0xA3),
                accent: egui::Color32::from_rgb(0x78, 0xD9, 0xFF),
                accent_soft: egui::Color32::from_rgb(0xB7, 0xA7, 0xFF),
                hovered: egui::Color32::from_rgb(0x28, 0x3A, 0x52),
                active: egui::Color32::from_rgb(0x31, 0x46, 0x66),
                wallpaper_alpha: 225,
                wallpaper_overlay: egui::Color32::from_rgba_unmultiplied(0x07, 0x0C, 0x16, 0x58),
            },
        }
    }
}

static ACTIVE_SKIN: AtomicU8 = AtomicU8::new(SkinId::Industrial as u8);

pub(crate) fn set_active_skin(skin: SkinId) {
    ACTIVE_SKIN.store(skin as u8, Ordering::Relaxed);
}

pub(crate) fn active_skin() -> SkinId {
    match ACTIVE_SKIN.load(Ordering::Relaxed) {
        value if value == SkinId::CleanAnime as u8 => SkinId::CleanAnime,
        _ => SkinId::Industrial,
    }
}

pub(crate) fn palette() -> SkinPalette {
    SkinPalette::for_skin(active_skin())
}

pub(crate) fn backplane() -> egui::Color32 {
    palette().backplane
}

pub(crate) fn panel() -> egui::Color32 {
    palette().panel
}

pub(crate) fn panel_raised() -> egui::Color32 {
    palette().panel_raised
}

pub(crate) fn divider() -> egui::Color32 {
    palette().divider
}

pub(crate) fn text_primary() -> egui::Color32 {
    palette().text_primary
}

pub(crate) fn text_secondary() -> egui::Color32 {
    palette().text_secondary
}

pub(crate) fn text_muted() -> egui::Color32 {
    palette().text_muted
}

pub(crate) fn muted_text() -> egui::Color32 {
    palette().text_secondary
}

pub(crate) fn accent() -> egui::Color32 {
    palette().accent
}

pub(crate) fn accent_soft() -> egui::Color32 {
    palette().accent_soft
}

pub(crate) fn apply(ctx: &egui::Context, skin: SkinId) {
    set_active_skin(skin);
    let palette = SkinPalette::for_skin(skin);
    let mut visuals = egui::Visuals::dark();
    visuals.dark_mode = true;
    visuals.window_fill = palette.panel;
    visuals.panel_fill = palette.panel;
    visuals.extreme_bg_color = palette.backplane;
    visuals.faint_bg_color = palette.backplane;
    visuals.code_bg_color = palette.backplane;
    visuals.window_stroke = egui::Stroke::new(1.0, palette.divider);
    visuals.widgets.noninteractive.bg_fill = palette.panel;
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, palette.text_primary);
    visuals.widgets.inactive.bg_fill = palette.panel_raised;
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, palette.text_primary);
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, palette.divider);
    visuals.widgets.hovered.bg_fill = palette.hovered;
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, palette.text_muted);
    visuals.widgets.active.bg_fill = palette.active;
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, palette.text_secondary);
    visuals.widgets.open.bg_fill = palette.panel_raised;
    visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0, palette.text_muted);
    visuals.selection.bg_fill = palette.text_secondary.gamma_multiply(0.22);
    visuals.selection.stroke = egui::Stroke::new(1.0, palette.text_secondary);
    visuals.hyperlink_color = palette.text_primary;
    ctx.set_visuals(visuals);

    ctx.set_theme(egui::Theme::Dark);
    ctx.global_style_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(10.0, 6.0);
        style.visuals.window_corner_radius = egui::CornerRadius::same(8);
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);
        style.visuals.widgets.open.corner_radius = egui::CornerRadius::same(6);
        style
            .text_styles
            .insert(egui::TextStyle::Heading, egui::FontId::proportional(16.0));
        style
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(13.0));
        style
            .text_styles
            .insert(egui::TextStyle::Button, egui::FontId::proportional(13.0));
        style
            .text_styles
            .insert(egui::TextStyle::Monospace, egui::FontId::monospace(13.0));
        style
            .text_styles
            .insert(egui::TextStyle::Small, egui::FontId::proportional(11.0));
    });
}

/// Installs system fonts for the UI languages, Simplified Chinese first and
/// Latin as fallback, without bundling a font or changing licensing.
///
/// Fonts never depend on the skin, so this runs once at startup instead of
/// on every skin change: each installation maps or reads tens of megabytes
/// of font files, and egui compares replaced font definitions byte by byte.
pub(crate) fn install_fonts(ctx: &egui::Context) {
    #[cfg(target_os = "macos")]
    {
        egui_system_fonts::set_with_presets(
            ctx,
            [egui_system_fonts::FontPreset::Latin],
            egui_system_fonts::FontStyle::Sans,
        );
        match macos_fonts::simplified_chinese() {
            Some(font) => ctx.add_font(font),
            None => tracing::warn!("No Simplified Chinese system font found; Chinese labels cannot be drawn"),
        }
    }
    #[cfg(not(target_os = "macos"))]
    let _ = ctx;
}

/// Locates the macOS Simplified Chinese system font.
///
/// egui-system-fonts asks fontdb for "PingFang SC", but PingFang has moved
/// twice: out of /System/Library/Fonts into a downloadable MobileAsset
/// (macOS 10.15–15), then into FontServices' reserved directory (macOS 26+),
/// which no generic font scanner searches. On those releases the Chinese UI
/// had no glyphs at all. CoreText still resolves "PingFang SC" to that file,
/// and Hiragino Sans GB and Heiti SC remain in /System/Library/Fonts.
#[cfg(target_os = "macos")]
mod macos_fonts {
    use std::path::{Path, PathBuf};

    use eframe::egui::{FontData, FontFamily};
    use eframe::epaint::text::{FontInsert, FontPriority, InsertFontFamily};

    /// Families in order of preference: the system face first, then faces
    /// that have shipped in /System/Library/Fonts for many releases.
    pub(super) const FAMILIES: [&str; 3] = ["PingFang SC", "Hiragino Sans GB", "Heiti SC"];

    const RESERVED_PINGFANG: &str =
        "/System/Library/PrivateFrameworks/FontServices.framework/Resources/Reserved/PingFangUI.ttc";
    const SYSTEM_FONTS: [&str; 4] = [
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        "/System/Library/Fonts/STHeiti Light.ttc",
        "/System/Library/Fonts/STHeiti Medium.ttc",
    ];
    const ASSETS: &str = "/System/Library/AssetsV2";

    /// Existing files that may contain one of [`FAMILIES`]. Listing a few
    /// known locations is far cheaper than scanning every installed font.
    pub(super) fn candidate_files() -> Vec<PathBuf> {
        let mut files = vec![PathBuf::from(RESERVED_PINGFANG)];
        files.extend(downloadable_pingfang());
        files.extend(SYSTEM_FONTS.iter().map(PathBuf::from));
        files.retain(|path| path.is_file());
        files
    }

    /// PingFang delivered as a MobileAsset:
    /// `AssetsV2/com_apple_MobileAsset_Font*/<asset>/AssetData/PingFang*.ttc`.
    fn downloadable_pingfang() -> Vec<PathBuf> {
        let children = |dir: &Path| {
            std::fs::read_dir(dir)
                .into_iter()
                .flatten()
                .flatten()
                .map(|entry| entry.path())
                .collect::<Vec<_>>()
        };
        let name_starts_with = |path: &Path, prefix: &str| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with(prefix))
        };
        children(Path::new(ASSETS))
            .into_iter()
            .filter(|kind| name_starts_with(kind, "com_apple_MobileAsset_Font"))
            .flat_map(|kind| children(&kind))
            .flat_map(|asset| children(&asset.join("AssetData")))
            .filter(|font| name_starts_with(font, "PingFang"))
            .collect()
    }

    /// The preferred available face as an egui font that takes precedence
    /// over the Latin fallback, like PingFang SC does in native macOS UI.
    pub(crate) fn simplified_chinese() -> Option<FontInsert> {
        let mut database = fontdb::Database::new();
        for file in candidate_files() {
            if let Err(error) = database.load_font_file(&file) {
                tracing::debug!("Skipping font {}: {error}", file.display());
            }
        }
        FAMILIES.iter().find_map(|&family| {
            let id = database.query(&fontdb::Query {
                families: &[fontdb::Family::Name(family)],
                ..fontdb::Query::default()
            })?;
            let (source, index) = database.face_source(id)?;
            let path = match source {
                fontdb::Source::File(path) | fontdb::Source::SharedFile(path, _) => path,
                fontdb::Source::Binary(_) => return None,
            };
            let mut data = FontData::from_static(map_font_file(&path)?);
            data.index = index;
            tracing::info!(family, path = %path.display(), index, "Using Chinese system font");
            Some(FontInsert::new(
                &format!("macos-system:{family}"),
                data,
                vec![
                    InsertFontFamily {
                        family: FontFamily::Proportional,
                        priority: FontPriority::Highest,
                    },
                    InsertFontFamily {
                        family: FontFamily::Monospace,
                        priority: FontPriority::Highest,
                    },
                ],
            ))
        })
    }

    /// Maps a font collection instead of copying it to the heap. PingFang is
    /// a ~60 MB collection; egui only touches the tables and glyphs it draws,
    /// so clean, file-backed pages replace that much resident memory. The
    /// mapping is leaked on purpose: installed fonts live as long as the app.
    fn map_font_file(path: &Path) -> Option<&'static [u8]> {
        let file = std::fs::File::open(path).ok()?;
        // SAFETY: these are system font files on the sealed, read-only system
        // volume, or MobileAsset files that updates replace rather than
        // rewrite. Their contents cannot change while mapped.
        let map = unsafe { memmap2::Mmap::map(&file) }
            .inspect_err(|error| tracing::debug!("Could not map font {}: {error}", path.display()))
            .ok()?;
        Some(&Box::leak(Box::new(map))[..])
    }
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;

    /// Labels the workbench shows in its default Simplified Chinese UI.
    const CHINESE_SAMPLE: &str = "工作台电压电流功率录制设置";

    /// Whether the `font_id` stack can draw every glyph in `text`.
    fn renders(ctx: &egui::Context, font_id: &egui::FontId, text: &str) -> bool {
        // Pending font changes are applied at the start of the next pass.
        let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
        ctx.fonts_mut(|fonts| fonts.has_glyphs(font_id, text))
    }

    /// "A font was found" is not enough: on macOS 26+ the resolver found only
    /// Latin faces and every Chinese label rendered without glyphs.
    #[test]
    fn installed_fonts_draw_the_chinese_and_latin_ui() {
        let ctx = egui::Context::default();
        install_fonts(&ctx);
        apply(&ctx, SkinId::Industrial);
        for font_id in [egui::FontId::proportional(13.0), egui::FontId::monospace(13.0)] {
            assert!(
                renders(&ctx, &font_id, CHINESE_SAMPLE),
                "no installed {font_id:?} font covers the default Chinese UI labels"
            );
            assert!(renders(&ctx, &font_id, "Voltage 12.345 V · 1.234 A"));
        }
    }

    #[test]
    fn skin_changes_keep_the_installed_fonts() {
        let ctx = egui::Context::default();
        install_fonts(&ctx);
        apply(&ctx, SkinId::Industrial);
        let font_id = egui::FontId::proportional(13.0);
        assert!(renders(&ctx, &font_id, CHINESE_SAMPLE));
        apply(&ctx, SkinId::CleanAnime);
        assert!(renders(&ctx, &font_id, CHINESE_SAMPLE));
    }

    /// PingFang is the native face whenever its file exists, including the
    /// reserved FontServices location used since macOS 26.
    #[test]
    fn pingfang_is_preferred_when_installed() {
        let pingfang_installed = macos_fonts::candidate_files().iter().any(|path| {
            path.file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with("PingFang"))
        });
        let font = macos_fonts::simplified_chinese().expect("a Simplified Chinese system font");
        if pingfang_installed {
            assert_eq!(font.name, "macos-system:PingFang SC");
        } else {
            assert!(macos_fonts::FAMILIES.iter().any(|family| font.name.ends_with(family)));
        }
    }

    #[test]
    fn instrument_design_tokens_do_not_drift() {
        assert_eq!(BACKPLANE, egui::Color32::from_rgb(0x0D, 0x11, 0x17));
        assert_eq!(PANEL, egui::Color32::from_rgb(0x16, 0x1B, 0x22));
        assert_eq!(DIVIDER, egui::Color32::from_rgb(0x30, 0x36, 0x3D));
        assert_eq!(VOLTAGE, egui::Color32::from_rgb(0x58, 0xA6, 0xFF));
        assert_eq!(CURRENT, egui::Color32::from_rgb(0x3F, 0xB9, 0x50));
        assert_eq!(POWER, egui::Color32::from_rgb(0xD2, 0x99, 0x22));
        assert_eq!(ENERGY, egui::Color32::from_rgb(0xF7, 0x6E, 0xB6));
        assert_eq!(CAPACITY, egui::Color32::from_rgb(0xA3, 0x7A, 0xF2));
        assert_eq!(RECORDING, egui::Color32::from_rgb(0xF8, 0x51, 0x49));
    }

    #[test]
    fn skins_only_change_containers_not_measurement_semantics() {
        let industrial = SkinPalette::for_skin(SkinId::Industrial);
        let clean = SkinPalette::for_skin(SkinId::CleanAnime);
        assert_eq!(industrial.backplane, BACKPLANE);
        assert_eq!(industrial.panel, PANEL);
        assert_eq!(industrial.divider, DIVIDER);
        assert_ne!(clean.backplane, industrial.backplane);
        assert_ne!(clean.panel_raised, industrial.panel_raised);
        assert_eq!(industrial.wallpaper_alpha, 0);
        assert!(clean.wallpaper_alpha > 0);
        assert_eq!(VOLTAGE, egui::Color32::from_rgb(0x58, 0xA6, 0xFF));
        assert_eq!(CURRENT, egui::Color32::from_rgb(0x3F, 0xB9, 0x50));
        assert_eq!(POWER, egui::Color32::from_rgb(0xD2, 0x99, 0x22));
    }
}
