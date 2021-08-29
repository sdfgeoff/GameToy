use crate::state::{EditorState, Reactor, StateOperation};

pub fn add_node_widget(ui: &mut egui::Ui, state: &EditorState, reactor: &mut Reactor) {
    let node_list = &state.project_data.graph.nodes;
    egui::Grid::new("add_node_grid")
        .num_columns(2)
        .show(ui, |ui| {
            if ui.button("RenderPass").clicked() {
                reactor.queue_operation(StateOperation::CreateNode(gametoy::config_file::Node::RenderPass(
                    gametoy::config_file::RenderPassConfig {
                        name: format!("Render Pass {}", node_list.len()),
                        output_texture_slots: vec![gametoy::config_file::OutputBufferConfig {
                            name: "RenderOut".to_string(),
                            format: gametoy::config_file::OutputBufferFormat::RGB8,
                        }],
                        input_texture_slots: vec![gametoy::config_file::InputBufferConfig {
                            name: "KeyboardInput".to_string(),
                        }],
                        resolution_scaling_mode:
                            gametoy::config_file::ResolutionScalingMode::ViewportScale(1.0, 1.0),
                        fragment_shader_paths: vec![],
                        execution_mode: gametoy::config_file::ExecutionMode::Always,
                    },
                )));
            }

            if ui.button("Image").clicked() {
                reactor.queue_operation(StateOperation::CreateNode(gametoy::config_file::Node::Image(
                    gametoy::config_file::ImageConfig {
                        name: format!("Image {}", node_list.len()),
                        path: String::new(),
                    },
                )));
            }
            ui.end_row();
            if ui.button("Keyboard").clicked() {
                reactor.queue_operation(StateOperation::CreateNode({
                    gametoy::config_file::Node::Keyboard(gametoy::config_file::KeyboardConfig {
                        name: format!("Keyboard {}", node_list.len()),
                    })
                }));
            }
            if ui.button("Output").clicked() {
                reactor.queue_operation(StateOperation::CreateNode(gametoy::config_file::Node::Output(
                    gametoy::config_file::OutputConfig {
                        name: format!("Output {}", node_list.len()),
                    },
                )));
            }
        });
}