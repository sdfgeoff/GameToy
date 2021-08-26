use gametoy::config_file::ResolutionScalingMode;
use crate::helpers::{path_widget, list_edit};

pub fn edit_render_pass(node: &mut gametoy::config_file::RenderPassConfig, ui: &mut egui::Ui) {
ui.label("Name:");
                ui.text_edit_singleline(&mut node.name)
                    .on_hover_text("Name of the node");
                ui.end_row();
                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label("Scaling Mode:");
                resolution_scaling_mode_widget(&mut node.resolution_scaling_mode, ui);
                ui.end_row();

                ui.label("Execution Mode:");
                execution_mode_widget(ui, &mut node.execution_mode);
                ui.end_row();
                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label("Shader Sources: ");
                ui.vertical(|ui| {

                    list_edit(ui, &mut node.fragment_shader_paths, |ui, _item_id, path| {
                        path_widget(path, ui);
                    });
                    if ui.button("Add Source").clicked() {
                        node.fragment_shader_paths.push(String::new());
                    }
                });
                ui.end_row();
                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label("Input Textures: ");
                ui.vertical(|ui| {
                    list_edit(ui, &mut node.input_texture_slots, |ui, _item_id, input_config| {
                        egui::Grid::new("input_texture_slot_grid").num_columns(2).show(ui, |ui| {
                            ui.label("Name:");
                            path_widget(&mut input_config.name, ui);
                            ui.end_row();
                        });
                        
                    });
                    if ui.button("Add Input").clicked() {
                        node.input_texture_slots.push(gametoy::config_file::InputBufferConfig {
                            name: String::new()
                        });
                    }
                });
                ui.end_row();
                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label("Output Textures: ");
                ui.vertical(|ui| {
                    list_edit(ui, &mut node.output_texture_slots, |ui, _item_id, output_config| {
                        egui::Grid::new("output_texture_slot_grid").num_columns(2).show(ui, |ui| {
                            ui.label("Name:");
                            path_widget(&mut output_config.name, ui);
                            ui.end_row();
                            ui.label("Pixel Format:");
                            egui::ComboBox::from_id_source("pixel_format")
                            .selected_text("Pixel Format")
                            .show_ui(ui, |ui| {
                                ui.label("Coming Soon")
                            });
                            ui.end_row();
                        });
                        
                    });
                    if ui.button("Add Output").clicked() {
                        node.input_texture_slots.push(gametoy::config_file::InputBufferConfig {
                            name: String::new()
                        });
                    }
                });
            }


#[derive(PartialEq)]
enum ResScalingModeUi {
    Fixed,
    ViewportScale
}
impl ResScalingModeUi {
    pub fn from_resolution_scaling_mode(mode: &ResolutionScalingMode) -> Self {
        match mode {
            ResolutionScalingMode::Fixed(_, _) => Self::Fixed,
            ResolutionScalingMode::ViewportScale(_, _) => Self::ViewportScale,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Fixed => "Fixed",
            Self::ViewportScale => "Viewport Scale"
        }
    }
    pub fn to_default(&self) -> ResolutionScalingMode {
        match self {
            Self::Fixed => ResolutionScalingMode::Fixed(512, 512),
            Self::ViewportScale => ResolutionScalingMode::ViewportScale(1.0, 1.0)
        }
    }
}



pub fn resolution_scaling_mode_widget(scaling_mode: &mut ResolutionScalingMode, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let mut current_scaling_mode = ResScalingModeUi::from_resolution_scaling_mode(&scaling_mode);
        egui::ComboBox::from_id_source(12345)
            .selected_text(current_scaling_mode.to_str())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut current_scaling_mode, ResScalingModeUi::Fixed, ResScalingModeUi::Fixed.to_str());
                ui.selectable_value(&mut current_scaling_mode, ResScalingModeUi::ViewportScale, ResScalingModeUi::ViewportScale.to_str());
            });

        if current_scaling_mode != ResScalingModeUi::from_resolution_scaling_mode(&scaling_mode) {
            *scaling_mode = current_scaling_mode.to_default()
        }
        match scaling_mode {
            ResolutionScalingMode::Fixed(mut x, mut y) => {
                ui.add(egui::widgets::DragValue::new(&mut x).suffix("px"));
                ui.add(egui::widgets::DragValue::new(&mut y).suffix("px"));
                x = x.max(1);
                y = y.max(1);
                *scaling_mode = ResolutionScalingMode::Fixed(x, y);
            },
            ResolutionScalingMode::ViewportScale(mut x, mut y) => {
                x = x * 100.0;
                y = y * 100.0;
                ui.add(egui::widgets::DragValue::new(&mut x).suffix("%"));
                ui.add(egui::widgets::DragValue::new(&mut y).suffix("%"));
                x = x / 100.0;
                y = y / 100.0;
                x = x.max(0.0);
                y = y.max(0.0);
                *scaling_mode = ResolutionScalingMode::ViewportScale(x, y);
            }
        }
    });
}

pub fn execution_mode_widget(ui: &mut egui::Ui, execution_mode: &mut gametoy::config_file::ExecutionMode) {
    egui::ComboBox::from_id_source("Execution Mode")
    .selected_text(execution_mode_to_str(execution_mode))
    .show_ui(ui, |ui| {
        ui.selectable_value(execution_mode, gametoy::config_file::ExecutionMode::Always, execution_mode_to_str(&gametoy::config_file::ExecutionMode::Always));
        ui.selectable_value(execution_mode, gametoy::config_file::ExecutionMode::CreationOrResized, execution_mode_to_str(&gametoy::config_file::ExecutionMode::CreationOrResized));
        ui.selectable_value(execution_mode, gametoy::config_file::ExecutionMode::InputsChanged, execution_mode_to_str(&gametoy::config_file::ExecutionMode::InputsChanged));
});
}

pub fn execution_mode_to_str(this_mode: &gametoy::config_file::ExecutionMode) -> &str {
    match this_mode {
        gametoy::config_file::ExecutionMode::Always => "Always",
        gametoy::config_file::ExecutionMode::CreationOrResized => "Creation Or Resized",
        gametoy::config_file::ExecutionMode::InputsChanged => "Inputs Changed",
    }

}