use crate::helpers::{list_edit_mut, path_widget};
use gametoy::config_file::{
    InputBufferConfig, OutputBufferConfig, OutputBufferFormat, RenderPassConfig,
};

use super::execution_mode::execution_mode_widget;
use super::output_buffer_format::output_buffer_format_selector;
use super::resolution_scaling_mode::resolution_scaling_mode_widget;
use egui::TextEdit;

pub fn edit_render_pass(ui: &mut egui::Ui, node: &mut RenderPassConfig) {
    ui.label("Name:");
    ui.text_edit_singleline(&mut node.name)
        .on_hover_text("Name of the node");
    ui.end_row();
    ui.separator();
    ui.end_row();

    ui.label("Scaling Mode:");
    resolution_scaling_mode_widget(&mut node.resolution_scaling_mode, ui);
    ui.end_row();

    ui.label("Execution Mode:");
    execution_mode_widget(ui, &mut node.execution_mode);
    ui.end_row();
    ui.separator();
    ui.end_row();

    ui.label("Shader Sources: ");
    ui.vertical(|ui| {
        list_edit_mut(
            ui,
            &mut node.fragment_shader_paths,
            |ui, _item_id, path| {
                path_widget(path, ui);
            },
            "shader_source_grid",
        );
        if ui.button("Add Source").clicked() {
            node.fragment_shader_paths.push(String::new());
        }
    });
    ui.end_row();
    ui.separator();
    ui.end_row();

    ui.label("Input Textures: ");
    ui.vertical(|ui| {
        list_edit_mut(
            ui,
            &mut node.input_texture_slots,
            |ui, item_id, input_config| {
                egui::Grid::new(format!("in_slot_grid{}", item_id))
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Name:");
                        ui.add_sized([110.0, 20.0], TextEdit::singleline(&mut input_config.name));
                        ui.end_row();
                    });
            },
            "input_texture_grid",
        );
        if ui.button("Add Input").clicked() {
            node.input_texture_slots.push(InputBufferConfig {
                name: String::new(),
            });
        }
    });
    ui.end_row();
    ui.separator();
    ui.end_row();

    ui.label("Output Textures: ");
    ui.vertical(|ui| {
        list_edit_mut(
            ui,
            &mut node.output_texture_slots,
            |ui, item_id, output_config| {
                egui::Grid::new(format!("out_slot_grid{}", item_id))
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Name:");
                        ui.add_sized([110.0, 20.0], TextEdit::singleline(&mut output_config.name));
                        ui.end_row();
                        ui.label("Format:");
                        output_buffer_format_selector(ui, &mut output_config.format, item_id);
                        ui.end_row();
                        ui.label("Mipmap:");
                        ui.checkbox(&mut output_config.generate_mipmap, "");
                        ui.end_row();
                    });
            },
            "output_texture_grid",
        );
        if ui.button("Add Output").clicked() {
            node.output_texture_slots.push(OutputBufferConfig {
                name: String::new(),
                format: OutputBufferFormat::RGBA32F,
                generate_mipmap: false,
            });
        }
    });
}
