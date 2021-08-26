mod app;
mod graph;
mod metadata;

mod helpers;

fn main() {
    let ap = app::GametoyGraphEditor::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(ap), native_options);
}
