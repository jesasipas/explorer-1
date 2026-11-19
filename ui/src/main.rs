// Ce socle expose le contrat de données et la logique pure avant que la vraie interface
// (PR suivante) ne les consomme entièrement ; les API non encore appelées restent donc
// présentes mais inutilisées depuis `main`.
#![allow(dead_code)]

mod contract;
mod demo;
mod logic;

use contract::TelemetrySource;
use demo::DemoSource;

struct App {
    source: Option<Box<dyn TelemetrySource>>,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(source) = &mut self.source {
            // Vide le canal sans bloquer : le thread UI ne doit jamais attendre sur une
            // source de données (voir docs/troubleshooting.md, section Interface).
            while source.try_recv_telemetry().is_some() {}
            while source.try_recv_video_frame().is_some() {}
        }

        ui.heading("explorer-1 — interface opérateur");
        ui.label("Socle en place ; l'interface visuelle arrive dans une PR suivante.");

        // La démo pousse des données en continu : on redemande un rendu pour rester à jour
        // sans dépendre d'une interaction utilisateur.
        if self.source.is_some() {
            ui.ctx().request_repaint();
        }
    }
}

fn main() -> eframe::Result<()> {
    let demo_mode = std::env::args().any(|arg| arg == "--demo");

    let source: Option<Box<dyn TelemetrySource>> = if demo_mode {
        Some(Box::new(DemoSource::spawn()))
    } else {
        None
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "explorer-1 — interface opérateur",
        options,
        Box::new(move |_cc| Ok(Box::new(App { source }))),
    )
}
