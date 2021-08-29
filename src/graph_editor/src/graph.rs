use crate::state::{Reactor, StateOperation};
use egui_nodes::{Context, LinkArgs, NodeConstructor};
use gametoy::config_file::ConfigFile;
use std::collections::HashMap;

use crate::nodes::{get_input_slots, get_node_name, get_output_slots};

pub fn draw_rendergraph_editor(
    ui: &mut egui::Ui,
    reactor: &mut Reactor,
    node_context: &mut Context,
    new_proj: &ConfigFile,
) {
    let graph_nodes = &new_proj.graph.nodes;
    let graph_links = &new_proj.graph.links;

    let nodes: Vec<NodeConstructor> = graph_nodes
        .iter()
        .enumerate()
        .map(|(node_id, node)| {
            let title = format!("({}) {}", node_id, get_node_name(&node));

            let mut node_constructor = NodeConstructor::new(node_id, Default::default())
                .with_title(move |ui| ui.label(title));

            let input_slots = get_input_slots(&node);
            for (slot_id, input_name) in input_slots.iter().enumerate() {
                let slot_name = input_name.to_string();
                let pin_id = pairing_function(node_id, slot_id);
                node_constructor =
                    node_constructor.with_input_attribute(pin_id, Default::default(), move |ui| {
                        ui.label(slot_name)
                    });
            }
            for (slot_id, output_name) in get_output_slots(&node).iter().enumerate() {
                let slot_name = output_name.clone();
                let pin_id = pairing_function(node_id, slot_id + input_slots.len());
                node_constructor =
                    node_constructor.with_output_attribute(pin_id, Default::default(), move |ui| {
                        ui.label(slot_name)
                    });
            }
            node_constructor
        })
        .collect();

    // With a change in file format a lot of this complexity could be removed
    // Here we convert from names of nodes + names of slots into ID's
    // (and in a short distance we convert back)

    let links = &mut vec![];
    let mut node_name_to_id: HashMap<String, usize> = HashMap::new();
    for (node_id, node) in graph_nodes.iter().enumerate() {
        let node_name = get_node_name(node);
        node_name_to_id.insert(node_name.to_string(), node_id);
    }

    for link in graph_links {
        if let Some(start_node_id) = node_name_to_id.get(&link.start_node) {
            if let Some(end_node_id) = node_name_to_id.get(&link.end_node) {
                if let Some(start_slot_id) = get_output_slots(&graph_nodes[*start_node_id])
                    .iter()
                    .position(|x| *x == link.start_output_slot)
                {
                    if let Some(end_slot_id) = get_input_slots(&graph_nodes[*end_node_id])
                        .iter()
                        .position(|x| *x == link.end_input_slot)
                    {
                        let input_slots = get_input_slots(&graph_nodes[*start_node_id]);
                        let start_pin =
                            pairing_function(*start_node_id, start_slot_id + input_slots.len());
                        let end_pin = pairing_function(*end_node_id, end_slot_id);
                        links.push((start_pin, end_pin));
                    }
                }
            }
        }
    }

    // add them to the ui
    node_context.show(
        nodes,
        links
            .iter()
            .enumerate()
            .map(|(i, (start, end))| (i, *start, *end, LinkArgs::default())),
        ui,
    );

    // remove destroyed links
    if let Some(idx) = node_context.link_destroyed() {
        println!("del: {}", idx);
        links.remove(idx);
    }

    // add created links
    if let Some((start, end, _)) = node_context.link_created() {
        let (start_node_id, start_slot_id_and_len) = unpairing_function(start);
        let (end_node_id, end_slot_id) = unpairing_function(end);
        let start_node = &graph_nodes[start_node_id];
        let end_node = &graph_nodes[end_node_id];
        let start_slot_id = start_slot_id_and_len - get_input_slots(&start_node).len();

        let start_node_name = get_node_name(&start_node).to_string();
        let end_node_name = get_node_name(&end_node).to_string();
        let start_output_slot_name = get_output_slots(&start_node)[start_slot_id].clone();
        let end_input_slot_name = get_input_slots(&end_node)[end_slot_id].clone();

        let link_to_create = gametoy::config_file::Link {
            start_node: start_node_name,
            start_output_slot: start_output_slot_name,
            end_node: end_node_name,
            end_input_slot: end_input_slot_name,
        };
        // Remove old links that link to the same place:
        for (existing_link_id, existing_link) in new_proj.graph.links.iter().enumerate() {
            if existing_link.end_node == link_to_create.end_node
                && existing_link.end_input_slot == link_to_create.end_input_slot
            {
                reactor.queue_operation(StateOperation::DeleteLink(existing_link_id));
            }
        }
        reactor.queue_operation(StateOperation::CreateLink(link_to_create));
    }

    if let Some(selected_node) = node_context.get_selected_nodes().pop() {
        reactor.queue_operation(StateOperation::SelectNode(Some(selected_node)));
    }
}

/// An Elegant Pairing Function by Matthew Szudzik @ Wolfram Research, Inc.
fn pairing_function(x: usize, y: usize) -> usize {
    if x >= y {
        x * x + x + y
    } else {
        y * y + x
    }
}
fn unpairing_function(z: usize) -> (usize, usize) {
    let sqrtz = (f64::sqrt(z as f64)).floor() as usize;
    let sqz = sqrtz * sqrtz;
    if (z - sqz) >= sqrtz {
        (sqrtz, z - sqz - sqrtz)
    } else {
        (z - sqz, sqrtz)
    }
}
