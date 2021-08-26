mod renderpass;

use gametoy::config_file::{Node, ResolutionScalingMode};
use std::cell::RefCell;
use std::rc::Rc;


pub fn path_widget(path: &mut String, ui: &mut egui::Ui) {
    ui.text_edit_singleline(path);
}

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
                ui.label("Name:");
                ui.text_edit_singleline(&mut node.name)
                    .on_hover_text("Name of the node");
                ui.end_row();

                ui.label("Shader Sources: ");
                ui.vertical(|ui| {

                    list_edit(ui, &mut node.fragment_shader_paths, |ui, item_id, path| {
                        path_widget(path, ui);
                    });

                });
                ui.end_row();

                ui.label("Scaling Mode:");
                renderpass::resolution_scaling_mode_widget(&mut node.resolution_scaling_mode, ui)

                
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

/// Displays buttons for moving items within a list
pub fn list_edit_buttons<T>(ui: &mut egui::Ui, item_list: &mut Vec<T>, item_id: usize) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.spacing_mut().button_padding.x = 0.0;

        if ui.add(egui::Button::new('❌').small()).clicked() {
            item_list.remove(item_id);
        };
        if ui.add(egui::Button::new('⬇').small().enabled(item_id<item_list.len() - 1)).clicked() {
            item_list.swap(item_id, item_id+1);
        };
        if ui.add(egui::Button::new('⬆').small().enabled(item_id > 0)).clicked() {
            item_list.swap(item_id, item_id-1);
        };
    });
}

/// Allows editing of items within a list
pub fn list_edit<T: Clone>(ui: &mut egui::Ui, item_list: &mut Vec<T>, draw_item_function: fn(&mut egui::Ui, usize, &mut T) ) {
    let reflist: Vec<Rc<RefCell<T>>> = item_list.iter_mut().map(|x| {Rc::new(RefCell::new(x.clone()))}).collect();
    let mut reflistout = reflist.clone();

    egui::Grid::new("render_pass_grid")
        .num_columns(2)
        .show(ui, |ui| {
            for (node_id, node) in reflist.iter().enumerate() {
                list_edit_buttons(ui, &mut reflistout, node_id);

                draw_item_function(ui, node_id, &mut (node.borrow_mut()));
                ui.end_row();
            }
        });
    
        *item_list = reflistout.iter().map(|x| x.borrow().clone()).collect();

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
