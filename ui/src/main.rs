// Ce socle expose le contrat de données et la logique pure avant que la vraie interface
// (PR suivante) ne les consomme entièrement ; les API non encore appelées restent donc
// présentes mais inutilisées depuis `main`.
#![allow(dead_code)]

mod app;
mod contract;
mod demo;
mod logic;
mod theme;

use app::App;
use contract::TelemetrySource;
use demo::DemoSource;

fn main() -> eframe::Result<()> {
    let demo_mode = std::env::args().any(|arg| arg == "--demo");

    let source: Option<Box<dyn TelemetrySource>> = if demo_mode {
        Some(Box::new(DemoSource::spawn()))
    } else {
        None
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "explorer-1 — interface opérateur",
        options,
        Box::new(move |cc| Ok(Box::new(App::new(source, &cc.egui_ctx)))),
    )
}
