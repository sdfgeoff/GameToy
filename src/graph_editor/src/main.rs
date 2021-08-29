mod app;
mod graph;
mod helpers;
mod metadata;
mod nodes;
mod state;
mod render_order;

fn main() {
    let ap = app::GametoyGraphEditor::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(ap), native_options);
}
