//! Disposition générale de l'interface (grille 1280×800, voir `docs/maquette/README.md`).
//! Les quatre zones sont posées ici vides ; leur contenu arrive dans les PR suivantes
//! (vidéo, mini-carte lidar, panneau de commande).

use egui::{Color32, Frame, Panel, Stroke, UiBuilder};

use crate::contract::TelemetrySource;
use crate::theme;

/// Hauteur de la barre d'état, en pixels logiques (maquette : 64 px).
pub const STATUS_BAR_HEIGHT: f32 = 64.0;
/// Largeur du panneau de commande, en pixels logiques (maquette : 300 px).
pub const COMMAND_PANEL_WIDTH: f32 = 300.0;
/// Taille (carrée) de la mini-carte lidar, en bas à droite de la zone vidéo.
// [TODO] Dimension non spécifiée précisément dans la maquette texte ; valeur choisie en
// cohérence avec les captures (`docs/maquette/etat-mouvement.png`), à ajuster si besoin
// lors de l'implémentation du contenu réel (PR suivante).
pub const LIDAR_MINIMAP_SIZE: f32 = 220.0;

pub struct App {
    source: Option<Box<dyn TelemetrySource>>,
}

impl App {
    pub fn new(source: Option<Box<dyn TelemetrySource>>, ctx: &egui::Context) -> Self {
        theme::apply(ctx);
        Self { source }
    }

    fn zone_frame(fill: Color32) -> Frame {
        Frame::new()
            .fill(fill)
            .stroke(Stroke::new(1.0, theme::COLORS.line))
            .corner_radius(theme::PANEL_RADIUS)
            .inner_margin(12.0)
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(source) = &mut self.source {
            // Vide le canal sans bloquer : le thread UI ne doit jamais attendre sur une
            // source de données (voir docs/troubleshooting.md, section Interface).
            while source.try_recv_telemetry().is_some() {}
            while source.try_recv_video_frame().is_some() {}
        }

        Panel::top("status_bar")
            .resizable(false)
            .exact_size(STATUS_BAR_HEIGHT)
            .frame(Self::zone_frame(theme::COLORS.surface))
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Barre d'état");
                });
            });

        Panel::right("command_panel")
            .resizable(false)
            .exact_size(COMMAND_PANEL_WIDTH)
            .frame(Self::zone_frame(theme::COLORS.surface))
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("Panneau de commande");
                });
            });

        egui::CentralPanel::default()
            .frame(Self::zone_frame(theme::COLORS.bg))
            .show(ui, |ui| {
                let video_rect = ui.max_rect();
                let minimap_size = egui::Vec2::splat(
                    LIDAR_MINIMAP_SIZE.min(video_rect.width().min(video_rect.height())),
                );
                let minimap_rect =
                    egui::Rect::from_min_size(video_rect.max - minimap_size, minimap_size);

                ui.scope_builder(UiBuilder::new().max_rect(video_rect), |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Vidéo");
                    });
                });

                let minimap_frame = Self::zone_frame(theme::COLORS.surface);
                ui.scope_builder(UiBuilder::new().max_rect(minimap_rect), |ui| {
                    minimap_frame.show(ui, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label("Mini-carte lidar");
                        });
                    });
                });
            });

        // La démo pousse des données en continu : on redemande un rendu pour rester à jour
        // sans dépendre d'une interaction utilisateur.
        if self.source.is_some() {
            ui.ctx().request_repaint();
        }
    }
}
