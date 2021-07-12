use serde::{Deserialize, Serialize};

/// The game is configured through a config file. In the future this
/// may be generated using some sort of tool.
/// This file contains everything needed to construct the game except
/// for some "big" data such as textures and shaders
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    /// Data about the game, author etc.
    pub metadata: MetaData,

    /// Configures the rendergraph - the thing that actually "runs" the game
    pub graph: GraphConfig,
}

/// Contains information about the game/author etc.
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    /// The name of the game. This is displayed while loading, on the window title etc.
    pub game_name: String,

    /// What version of the game this is. Probably use semantic versioning or a git hash
    pub game_version: String,

    /// What date this version of the game was released on
    pub release_date: String,

    /// Website for this game
    pub website: String,

    /// The name of the author who created this game
    pub author_name: String,

    /// Any licensing information for this game.
    pub license: String,
}

/// Configures the rendergraph - the thing that actually runs the game.
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphConfig {
    /// A node is a "process" that takes some inputs and generates some output
    /// Note that the order is important - the order in here defines
    /// the order of execution
    pub nodes: Vec<Node>,

    /// A link connects the nodes
    pub links: Vec<Link>,
}

/// Connects two nodes in the rendergraph
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    /// The name of the node to start on
    pub start_node: String,
    /// Which output of the `start_node` to connect to
    pub start_output_slot: String,

    /// The name of the node to end on
    pub end_node: String,
    /// Which input of the `end_node` to connect to
    pub end_input_slot: String,
}

/// A node in the rendergraph. A rendergraph node takes a bunch of
/// texture inputs and processes them to create some outputs
#[derive(Serialize, Deserialize, Debug)]
pub enum Node {
    /// A static texture as defined in a file
    Texture(TextureConfig),

    /// A renderpass - runs a GLSL shader on it's inputs to create
    /// the outputs.
    RenderPass(RenderPassConfig),

    Output(OutputConfig),
}

/// A node containing a static texture
#[derive(Serialize, Deserialize, Debug)]
pub struct TextureConfig {
    /// the path to read the texture from
    pub name: String,
    pub path: String,
}

/// The node that actually writes to the screen
#[derive(Serialize, Deserialize, Debug)]
pub struct OutputConfig {
    pub name: String,
}

/// A node that runs a GLSL shader on it's inputs.
#[derive(Serialize, Deserialize, Debug)]
pub struct RenderPassConfig {
    /// the name of the renderpass
    pub name: String,
    /// Configuration of the renderpasses outputs
    pub output_texture_slots: Vec<OutputBufferConfig>,

    /// Configuration of the renderpasses inputs
    pub input_texture_slots: Vec<InputBufferConfig>,

    /// How the output textures pick what resolution to run at
    pub resolution_scaling_mode: ResolutionScalingMode,

    /// Path to the shader to execute. Often this is just a single
    /// string, but it can be multiple so that you can share functions
    /// between shaders.
    pub fragment_shader_paths: Vec<String>,

    /// When the renderpass should execute
    pub execution_mode: ExecutionMode,
}

/// An output channel from a `RenderPass
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputBufferConfig {
    /// The name of the output channel
    pub name: String,

    /// The texture format for the output buffer.
    pub format: OutputBufferFormat,

    /// Should the texture behind this output double buffer?
    /// Double buffering uses more VRAM, but allows a renderpass
    /// to sample it's own output on subsequent frames.
    pub double_buffer: bool,
}

/// An input channel for a `RenderPass`
#[derive(Serialize, Deserialize, Debug)]
pub struct InputBufferConfig {
    /// the name of the input channel
    pub name: String,
}

/// How the resolution of a `RenderPass` is configured
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResolutionScalingMode {
    /// The resolution is a fixed size and does not change
    /// These are i32's because for some reason it's the way openGL wants it
    Fixed(i32, i32),

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
    InputsChanged,
}

/// The precision and number of channels used for a buffer
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
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
impl OutputBufferFormat {
    pub fn to_gl_const(&self) -> u32 {
        match self {
            Self::R8 => glow::R8,
            Self::R8_SNORM => glow::R8_SNORM,
            Self::R16 => glow::R16,
            Self::R16_SNORM => glow::R16_SNORM,
            Self::RG8 => glow::RG8,
            Self::RG8_SNORM => glow::RG8_SNORM,
            Self::RG16 => glow::RG16,
            Self::RG16_SNORM => glow::RG16_SNORM,
            Self::R3_G3_B2 => glow::R3_G3_B2,
            Self::RGB4 => glow::RGB4,
            Self::RGB5 => glow::RGB5,
            Self::RGB8 => glow::RGB8,
            Self::RGB8_SNORM => glow::RGB8_SNORM,
            Self::RGB10 => glow::RGB10,
            Self::RGB12 => glow::RGB12,
            Self::RGB16_SNORM => glow::RGB16_SNORM,
            Self::RGBA2 => glow::RGBA2,
            Self::RGBA4 => glow::RGBA4,
            Self::RGB5_A1 => glow::RGB5_A1,
            Self::RGBA8 => glow::RGBA8,
            Self::RGBA8_SNORM => glow::RGBA8_SNORM,
            Self::RGB10_A2 => glow::RGB10_A2,
            Self::RGB10_A2UI => glow::RGB10_A2UI,
            Self::RGBA12 => glow::RGBA12,
            Self::RGBA16 => glow::RGBA16,
            Self::SRGB8 => glow::SRGB8,
            Self::SRGB8_ALPHA8 => glow::SRGB8_ALPHA8,
            Self::R16F => glow::R16F,
            Self::RG16F => glow::RG16F,
            Self::RGB16F => glow::RGB16F,
            Self::RGBA16F => glow::RGBA16F,
            Self::R32F => glow::R32F,
            Self::RG32F => glow::RG32F,
            Self::RGB32F => glow::RGB32F,
            Self::RGBA32F => glow::RGBA32F,
            Self::R11F_G11F_B10F => glow::R11F_G11F_B10F,
            Self::RGB9_E5 => glow::RGB9_E5,
            Self::R8I => glow::R8I,
            Self::R8UI => glow::R8UI,
            Self::R16I => glow::R16I,
            Self::R16UI => glow::R16UI,
            Self::R32I => glow::R32I,
            Self::R32UI => glow::R32UI,
            Self::RG8I => glow::RG8I,
            Self::RG8UI => glow::RG8UI,
            Self::RG16I => glow::RG16I,
            Self::RG16UI => glow::RG16UI,
            Self::RG32I => glow::RG32I,
            Self::RG32UI => glow::RG32UI,
            Self::RGB8I => glow::RGB8I,
            Self::RGB8UI => glow::RGB8UI,
            Self::RGB16I => glow::RGB16I,
            Self::RGB16UI => glow::RGB16UI,
            Self::RGB32I => glow::RGB32I,
            Self::RGB32UI => glow::RGB32UI,
            Self::RGBA8I => glow::RGBA8I,
            Self::RGBA8UI => glow::RGBA8UI,
            Self::RGBA16I => glow::RGBA16I,
            Self::RGBA16UI => glow::RGBA16UI,
            Self::RGBA32I => glow::RGBA32I,
            Self::RGBA32UI => glow::RGBA32UI,

        }
    }
}