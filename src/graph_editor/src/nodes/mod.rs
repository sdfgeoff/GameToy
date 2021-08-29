mod add_node_grid;
mod execution_mode;
mod output_buffer_format;
mod renderpass;
mod resolution_scaling_mode;

use super::helpers::path_widget;

pub use add_node_grid::add_node_widget;

use crate::state::{Reactor, StateOperation};
use gametoy::config_file::Node;

pub fn draw_node_properties(
    ui: &mut egui::Ui,
    reactor: &mut Reactor,
    node_data: &Node,
    node_id: usize,
) {
    let mut new_node_data = node_data.clone();

    egui::Grid::new("metadata_grid")
        .num_columns(2)
        .show(ui, |ui| match &mut new_node_data {
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
                renderpass::edit_render_pass(ui, node);
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

    if &new_node_data != node_data {
        reactor.queue_operation(StateOperation::UpdateNode(node_id, new_node_data));
    }
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

pub fn get_input_slots(node: &gametoy::config_file::Node) -> Vec<String> {
    match node {
        gametoy::config_file::Node::Image(_image_data) => vec![],
        gametoy::config_file::Node::Keyboard(_keyboard_data) => vec![],
        gametoy::config_file::Node::Output(_output_data) => {
            vec![gametoy::nodes::Output::INPUT_BUFFER_NAME.to_string()]
        }
        gametoy::config_file::Node::RenderPass(renderpass_data) => renderpass_data
            .input_texture_slots
            .iter()
            .map(|x| x.name.clone())
            .collect(),
    }
}

pub fn get_output_slots(node: &gametoy::config_file::Node) -> Vec<String> {
    match node {
        gametoy::config_file::Node::Image(_image_data) => {
            vec![gametoy::nodes::Image::OUTPUT_BUFFER_NAME.to_string()]
        }
        gametoy::config_file::Node::Keyboard(_keyboard_data) => {
            vec![gametoy::nodes::Keyboard::OUTPUT_BUFFER_NAME.to_string()]
        }
        gametoy::config_file::Node::Output(_output_data) => vec![],
        gametoy::config_file::Node::RenderPass(renderpass_data) => renderpass_data
            .output_texture_slots
            .iter()
            .map(|x| x.name.clone())
            .collect(),
    }
}
