//! Jetons visuels de la maquette (`docs/maquette/README.md`, section « Système visuel »)
//! et application de ces jetons à un `egui::Context`.

use egui::{Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId};

/// Jeton de couleur. Valeurs reprises exactement de la maquette.
pub struct Colors {
    pub bg: Color32,
    pub surface: Color32,
    pub line: Color32,
    pub text: Color32,
    pub text_secondary: Color32,
    pub label: Color32,
    pub accent: Color32,
    pub warn: Color32,
    pub danger: Color32,
    pub estop: Color32,
    pub estop_border: Color32,
}

pub const COLORS: Colors = Colors {
    bg: Color32::from_rgb(0x08, 0x0B, 0x10),
    surface: Color32::from_rgb(0x0E, 0x13, 0x1A),
    line: Color32::from_rgb(0x1F, 0x29, 0x37),
    text: Color32::from_rgb(0xE6, 0xED, 0xF3),
    text_secondary: Color32::from_rgb(0xCB, 0xD5, 0xE1),
    label: Color32::from_rgb(0x94, 0xA3, 0xB8),
    accent: Color32::from_rgb(0x22, 0xD3, 0xEE),
    warn: Color32::from_rgb(0xFB, 0xBF, 0x24),
    danger: Color32::from_rgb(0xF8, 0x71, 0x71),
    estop: Color32::from_rgb(0xC8, 0x30, 0x3A),
    estop_border: Color32::from_rgb(0xE5, 0x48, 0x4D),
};

/// Rayons repris de la maquette : panneaux, boutons.
pub const PANEL_RADIUS: u8 = 12;
pub const BUTTON_RADIUS: u8 = 8;

/// Noms des familles de polices enregistrées auprès d'egui.
pub const FONT_TEXT: &str = "barlow";
pub const FONT_TEXT_MEDIUM: &str = "barlow-medium";
pub const FONT_TEXT_SEMIBOLD: &str = "barlow-semibold";
pub const FONT_MONO: &str = "jetbrains-mono";

/// Charge Barlow (texte/libellés) et JetBrains Mono (valeurs chiffrées) comme familles
/// nommées distinctes, pour pouvoir choisir la police par usage plutôt que par rôle
/// `Proportional`/`Monospace` générique d'egui.
pub fn font_definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        FONT_TEXT.to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/Barlow-Regular.ttf")).into(),
    );
    fonts.font_data.insert(
        FONT_TEXT_MEDIUM.to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/Barlow-Medium.ttf")).into(),
    );
    fonts.font_data.insert(
        FONT_TEXT_SEMIBOLD.to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/Barlow-SemiBold.ttf")).into(),
    );
    fonts.font_data.insert(
        FONT_MONO.to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf")).into(),
    );

    fonts.families.insert(
        FontFamily::Name(FONT_TEXT.into()),
        vec![FONT_TEXT.to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(FONT_TEXT_MEDIUM.into()),
        vec![FONT_TEXT_MEDIUM.to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(FONT_TEXT_SEMIBOLD.into()),
        vec![FONT_TEXT_SEMIBOLD.to_owned()],
    );
    fonts.families.insert(
        FontFamily::Name(FONT_MONO.into()),
        vec![FONT_MONO.to_owned()],
    );

    // Barlow reste la police par défaut (`Proportional`) et JetBrains Mono le rôle
    // `Monospace` d'egui, pour que les widgets qui n'opèrent pas de choix explicite
    // (boutons, labels par défaut) utilisent déjà les bonnes polices.
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, FONT_TEXT.to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, FONT_MONO.to_owned());

    fonts
}

/// `FontId` pratiques pour les usages courants de la maquette.
pub fn font_text(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_TEXT.into()))
}

pub fn font_text_semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_TEXT_SEMIBOLD.into()))
}

pub fn font_mono(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FONT_MONO.into()))
}

/// Applique le thème (couleurs, arrondis, polices) au contexte egui.
pub fn apply(ctx: &egui::Context) {
    ctx.set_fonts(font_definitions());

    ctx.all_styles_mut(|style| {
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(COLORS.text);
        style.visuals.window_fill = COLORS.surface;
        style.visuals.panel_fill = COLORS.bg;
        style.visuals.faint_bg_color = COLORS.surface;
        style.visuals.extreme_bg_color = COLORS.bg;
        style.visuals.widgets.noninteractive.bg_fill = COLORS.surface;
        style.visuals.widgets.noninteractive.weak_bg_fill = COLORS.surface;
        style.visuals.widgets.noninteractive.bg_stroke.color = COLORS.line;
        style.visuals.widgets.inactive.bg_fill = COLORS.surface;
        style.visuals.widgets.inactive.weak_bg_fill = COLORS.surface;
        style.visuals.widgets.inactive.bg_stroke.color = COLORS.line;
        style.visuals.widgets.hovered.bg_fill = COLORS.line;
        style.visuals.widgets.active.bg_fill = COLORS.accent;
        style.visuals.selection.bg_fill = COLORS.accent;
        style.visuals.hyperlink_color = COLORS.accent;
        style.visuals.window_stroke.color = COLORS.line;

        let button_radius = CornerRadius::same(BUTTON_RADIUS);
        style.visuals.widgets.noninteractive.corner_radius = button_radius;
        style.visuals.widgets.inactive.corner_radius = button_radius;
        style.visuals.widgets.hovered.corner_radius = button_radius;
        style.visuals.widgets.active.corner_radius = button_radius;
        style.visuals.window_corner_radius = CornerRadius::same(PANEL_RADIUS);
    });
}
