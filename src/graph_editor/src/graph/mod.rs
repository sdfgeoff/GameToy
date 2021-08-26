mod add_node_grid;
mod renderpass;

use super::helpers::path_widget;

pub use add_node_grid::add_node_widget;

use gametoy::config_file::{Node, ResolutionScalingMode};
use std::cell::RefCell;
use std::rc::Rc;

pub fn draw_node_properties(node_data: &mut Node, ui: &mut egui::Ui) {
    egui::Grid::new("metadata_grid")
        .num_columns(2)
        .show(ui, |ui| match node_data {
            Node::Image(node) => {
                ui.label("Name:");
                ui.text_edit_singleline(&mut node.name)
                    .on_hover_text("Name of the node");
                ui.end_row();

                ui.label("Path:");
                path_widget(&mut node.path, ui);
                ui.end_row();
            }
            Node::RenderPass(node) => {
                renderpass::edit_render_pass(node, ui);
            }
            Node::Output(node) => {
                ui.label("Name:");
                ui.text_edit_singleline(&mut node.name)
                    .on_hover_text("Name of the node");
                ui.end_row();
            }
            Node::Keyboard(node) => {
                ui.label("Name:");
                ui.text_edit_singleline(&mut node.name)
                    .on_hover_text("Name of the node");
                ui.end_row();
            }
        });
}

pub fn get_node_name(node_data: &Node) -> &str {
    match node_data {
        Node::Image(img_data) => &img_data.name,
        Node::RenderPass(pass_data) => &pass_data.name,
        Node::Output(output_data) => &output_data.name,
        Node::Keyboard(keyboard_data) => &keyboard_data.name,
    }
}

pub fn get_node_type_name(node_data: &Node) -> &str {
    match node_data {
        Node::Image(_) => "Image",
        Node::RenderPass(_) => "RenderPass",
        Node::Output(_) => "Output",
        Node::Keyboard(_) => "Keyboard",
    }
}
