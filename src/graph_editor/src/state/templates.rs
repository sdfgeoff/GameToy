//! Defines some projects that can be created from the "New" menu
//! These should be kept as defined in code so that we can be user they
//! Match any changes made to the config file format. If they were (for example)
//! Creates using `parse(include_bytes!()` then they could fail.
use super::{EditorState, GamePlayState, ProjectData};
use std::collections::HashMap;

// A single render pass with keyboard input
pub fn simple_project() -> EditorState {
    let config_file = gametoy::config_file::ConfigFile {
        metadata: gametoy::config_file::MetaData {
            game_name: "Your Awesome Game".to_string(),
            game_version: "0.0.0".to_string(),
            release_date: "Today".to_string(),
            website: "".to_string(),
            author_name: "You".to_string(),
            license: "CC-BY-SA-NC 3.0".to_string(),
        },
        graph: gametoy::config_file::GraphConfig {
            nodes: vec![
                gametoy::config_file::Node::Keyboard(gametoy::config_file::KeyboardConfig {
                    name: "Keyboard".to_string(),
                }),
                gametoy::config_file::Node::RenderPass(gametoy::config_file::RenderPassConfig {
                    name: "Render Pass 1".to_string(),
                    output_texture_slots: vec![gametoy::config_file::OutputBufferConfig {
                        name: "RenderOut".to_string(),
                        format: gametoy::config_file::OutputBufferFormat::RGB8,
                    }],
                    input_texture_slots: vec![gametoy::config_file::InputBufferConfig {
                        name: "KeyboardInput".to_string(),
                    }],
                    resolution_scaling_mode:
                        gametoy::config_file::ResolutionScalingMode::ViewportScale(1.0, 1.0),
                    fragment_shader_paths: vec!["render.frag".to_string()],
                    execution_mode: gametoy::config_file::ExecutionMode::Always,
                }),
                gametoy::config_file::Node::Output(gametoy::config_file::OutputConfig {
                    name: "Output".to_string(),
                }),
            ],
            links: vec![
                gametoy::config_file::Link {
                    start_node: "Keyboard".to_string(),
                    start_output_slot: "tex".to_string(),
                    end_node: "Render Pass 1".to_string(),
                    end_input_slot: "KeyboardInput".to_string(),
                },
                gametoy::config_file::Link {
                    start_node: "Render Pass 1".to_string(),
                    start_output_slot: "RenderOut".to_string(),
                    end_node: "Output".to_string(),
                    end_input_slot: "col".to_string(),
                },
            ],
        },
    };
    let mut files = HashMap::new();
    files.insert("render.frag".to_string(), br#"Put your shader code in here"#.to_vec());


    EditorState {
        project_file: None,
        project_data: ProjectData{
            config_file,
            files,
        },
        ui_state: Default::default(),
        game_play_state: GamePlayState {
            render_size: [640, 480],
        }
    }
}
