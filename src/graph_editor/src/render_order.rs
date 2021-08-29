use crate::nodes::{get_node_name, get_node_type_name, add_node_widget};
use gametoy::config_file::Node;
use crate::state::{Reactor, StateOperation};
use crate::helpers::{list_edit, ListEditResponse};


pub fn render_order_widget(ui: &mut egui::Ui, reactor: &mut Reactor, nodes: &Vec<Node>) {
    let draw_node = |ui: &mut egui::Ui,
            node_id: usize,
            node: &gametoy::config_file::Node| {
            let area_name = &format!(
                "{} ({})",
                get_node_name(&node),
                get_node_type_name(&node)
            );

            let available_space = ui.available_size();
            if ui
                .add_sized(available_space, egui::Button::new(area_name))
                .clicked()
            {
                reactor
                    .queue_operation(StateOperation::SelectNode(Some(node_id)));
            };
        };

    match list_edit(ui, nodes, draw_node, "render_order_grid") {
        ListEditResponse::None => {}
        ListEditResponse::Remove(node_id) => reactor
            .queue_operation(StateOperation::DeleteNode(node_id)),
        ListEditResponse::Swap(node_id_1, node_id_2) => reactor
            .queue_operation(StateOperation::SwapNodes(node_id_1, node_id_2)),
    };

    ui.separator();
    ui.label("Add Node");

    add_node_widget(ui, nodes, reactor);
}                        