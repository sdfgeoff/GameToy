use gametoy::config_file::ExecutionMode;

pub fn execution_mode_widget(ui: &mut egui::Ui, execution_mode: &mut ExecutionMode) {
    egui::ComboBox::from_id_source("Execution Mode")
        .selected_text(execution_mode_to_str(execution_mode))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                execution_mode,
                ExecutionMode::Always,
                execution_mode_to_str(&ExecutionMode::Always),
            );
            ui.selectable_value(
                execution_mode,
                ExecutionMode::CreationOrResized,
                execution_mode_to_str(&ExecutionMode::CreationOrResized),
            );
            ui.selectable_value(
                execution_mode,
                ExecutionMode::InputsChanged,
                execution_mode_to_str(&ExecutionMode::InputsChanged),
            );
        });
}

pub fn execution_mode_to_str(this_mode: &ExecutionMode) -> &str {
    match this_mode {
        ExecutionMode::Always => "Always",
        ExecutionMode::CreationOrResized => "Creation Or Resized",
        ExecutionMode::InputsChanged => "Inputs Changed",
    }
}
