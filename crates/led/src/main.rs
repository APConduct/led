use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "LED Editor",
        options,
        Box::new(|cc| Ok(Box::new(led::txt::edtr::App::new(cc)))),
    )
}

pub use led::lua;
