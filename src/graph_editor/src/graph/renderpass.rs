use crate::helpers::{list_edit, path_widget};
use gametoy::config_file::{RenderPassConfig, OutputBufferConfig, InputBufferConfig, OutputBufferFormat};

use super::execution_mode::execution_mode_widget;
use super::resolution_scaling_mode::resolution_scaling_mode_widget;

pub fn edit_render_pass(node: &mut RenderPassConfig, ui: &mut egui::Ui) {
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
        list_edit(
            ui,
            &mut node.input_texture_slots,
            |ui, item_id, input_config| {
                egui::Grid::new(format!("in_slot_grid{}",item_id))
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Name:");
                        path_widget(&mut input_config.name, ui);
                        ui.end_row();
                    });
            },
        );
        if ui.button("Add Input").clicked() {
            node.input_texture_slots
                .push(InputBufferConfig {
                    name: String::new(),
                });
        }
    });
    ui.end_row();
    ui.separator();
    ui.separator();
    ui.end_row();

    ui.label("Output Textures: ");
    ui.vertical(|ui| {
        list_edit(
            ui,
            &mut node.output_texture_slots,
            |ui, item_id, output_config| {
                egui::Grid::new(format!("out_slot_grid{}",item_id))
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Name:");
                        path_widget(&mut output_config.name, ui);
                        ui.end_row();
                        ui.label("Pixel Format:");
                        egui::ComboBox::from_id_source(format!("out_format_pixel{}", item_id))
                            .selected_text("Pixel Format")
                            .show_ui(ui, |ui| ui.label("Coming Soon"));
                        ui.end_row();
                    });
            },
        );
        if ui.button("Add Output").clicked() {
            node.output_texture_slots
                .push(OutputBufferConfig {
                    name: String::new(),
                    format: OutputBufferFormat::RGBA32F
                });
        }
    });
}
