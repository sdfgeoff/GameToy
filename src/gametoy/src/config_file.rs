use serde::{Serialize, Deserialize};


/// The game is configured through a config file. In the future this
/// may be generated using some sort of tool.
/// This file contains everything needed to construct the game except
/// for some "big" data such as textures and shaders
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    /// Data about the game, author etc.
    metadata: MetaData,

    /// Configures the rendergraph - the thing that actually "runs" the game
    graph: GraphConfig,
}

/// Contains information about the game/author etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    /// The name of the game. This is displayed while loading, on the window title etc.
    game_name: String,

    /// What version of the game this is. Probably use semantic versioning or a git hash
    game_version: String,
    
    /// What date this version of the game was released on
    release_date: String,
    
    /// Website for this game
    website: String,

    /// The name of the author who created this game
    author_name: String,
    
    /// Any licensing information for this game.
    license: String,
}


/// Configures the rendergraph - the thing that actually runs the game.
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphConfig {
    /// A node is a "process" that takes some inputs and generates some output
    /// Note that the order is important - the order in here defines
    /// the order of execution
    nodes: Vec<Node>,

    /// A link connects the nodes
    links: Vec<Link>
}

/// Connects two nodes in the rendergraph
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    /// The name of the node to start on
    start_node: String,
    /// Which output of the `start_node` to connect to
    start_output_slot: String,

    /// The name of the node to end on
    end_node: String,
    /// Which input of the `end_node` to connect to
    end_input_slot: String,
}

/// A node in the rendergraph. A rendergraph node takes a bunch of
/// texture inputs and processes them to create some outputs
#[derive(Serialize, Deserialize, Debug)]
pub enum Node{
    /// A static texture as defined in a file
    Texture(TextureConfig),

    /// A renderpass - runs a GLSL shader on it's inputs to create
    /// the outputs.
    RenderPass(RenderPassConfig)
}

/// A node containing a static texture
#[derive(Serialize, Deserialize, Debug)]
pub struct TextureConfig {
    /// the path to read the texture from
    name: String,
    path: String
}

/// A node that runs a GLSL shader on it's inputs.
#[derive(Serialize, Deserialize, Debug)]
pub struct RenderPassConfig {
    /// the name of the renderpass
    name: String,
    /// Configuration of the renderpasses outputs
    output_texture_slots: Vec<OutputBufferConfig>,

    /// Configuration of the renderpasses inputs
    input_texture_slots: Vec<InputBufferConfig>,

    /// How the output textures pick what resolution to run at
    resolution_scaling_mode: ResolutionScalingMode,

    /// Path to the shader to execute. Often this is just a single
    /// string, but it can be multiple so that you can share functions
    /// between shaders.
    fragment_shader_paths: Vec<String>,

    /// When the renderpass should execute
    execution_mode: ExecutionMode,
}

/// An output channel from a `RenderPass
#[derive(Serialize, Deserialize, Debug)]
pub struct OutputBufferConfig {
    /// The name of the output channel
    name: String,

    /// The texture format for the output buffer.
    format: OutputBufferFormat,

    /// Should the texture behind this output double buffer?
    /// Double buffering uses more VRAM, but allows a renderpass
    /// to sample it's own output on subsequent frames.
    double_buffer: bool
}

/// An input channel for a `RenderPass`
#[derive(Serialize, Deserialize, Debug)]
struct InputBufferConfig {
    /// the name of the input channel
    name: String,
}

/// How the resolution of a `RenderPass` is configured
#[derive(Serialize, Deserialize, Debug)]
pub enum ResolutionScalingMode {
    /// The resolution is a fixed size and does not change
    Fixed(u32, u32),

    /// The resolution is a mutiplier of the viewport resolution
    ViewportScale(f32, f32),
}


/// When should the renderpass execute
#[derive(Serialize, Deserialize, Debug)]
pub enum ExecutionMode {

    /// Draw on every frame
    Always,

    /// Draw when the framebuffer is created or resized. This is useful
    /// for generated sprite sheets and textures.
    CreationOrResized,

    /// Run only when the inputs change. This is useful
    /// if you need to post-process generated textures etc.
    InputsChanged
}

/// The precision and number of channels used for a buffer
#[derive(Serialize, Deserialize, Debug)]
pub enum OutputBufferFormat {
    R8,
    R8_SNORM,	 	 	 	 
    R16,
    R16_SNORM,
    RG8,	
    RG8_SNORM,
    RG16,
    RG16_SNORM,
    R3_G3_B2,
    RGB4,
    RGB5,
    RGB8,
    RGB8_SNORM,
    RGB10,
    RGB12,
    RGB16_SNORM,
    RGBA2,
    RGBA4,
    RGB5_A1,
    RGBA8,
    RGBA8_SNORM,
    RGB10_A2,
    RGB10_A2UI,
    RGBA12,
    RGBA16,
    SRGB8,
    SRGB8_ALPHA8,
    R16F,
    RG16F,
    RGB16F,
    RGBA16F,
    R32F,
    RG32F,
    RGB32F,
    RGBA32F,
    R11F_G11F_B10F,
    RGB9_E5,
    R8I,
    R8UI,
    R16I,
    R16UI,
    R32I,
    R32UI,
    RG8I,
    RG8UI,
    RG16I,
    RG16UI,
    RG32I,
    RG32UI,
    RGB8I,
    RGB8UI,
    RGB16I,
    RGB16UI,
    RGB32I,
    RGB32UI,
    RGBA8I,
    RGBA8UI,
    RGBA16I,
    RGBA16UI,
    RGBA32I,
    RGBA32UI,
}
