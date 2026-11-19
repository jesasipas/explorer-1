//! Capture de référence de la grille 1280×800 (thème + 4 zones vides), à comparer
//! visuellement avec `docs/maquette/etat-mouvement.png` (disposition, couleurs de fond).
//!
//! [TODO] Rendu logiciel (wgpu) : fonctionne en local, mais nécessite probablement un
//! pilote Vulkan/GL logiciel (ex: Mesa/lavapipe) sur un runner CI sans GPU. Décision CI
//! (installer un rendu logiciel, exclure ce test du job CI, ou ajuster les tolérances)
//! laissée au lead — voir le rapport de PR.

// Ce test recompile les modules du binaire comme crate séparée ; les API non exercées par
// ce seul test (reste du contrat, variantes de couleurs/polices non encore utilisées par la
// mise en page vide) resteraient sinon signalées comme mortes.
#![allow(dead_code)]

#[path = "../src/app.rs"]
mod app;
#[path = "../src/contract.rs"]
mod contract;
#[path = "../src/theme.rs"]
mod theme;

use app::App;
use egui_kittest::Harness;

#[test]
fn layout_empty_matches_snapshot() {
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::new(1280.0, 800.0))
        .build_eframe(|cc| App::new(None, &cc.egui_ctx));

    harness.run();
    harness.snapshot("layout_empty");
}
