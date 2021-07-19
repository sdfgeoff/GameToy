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
/// Not all of these formats work in webGL. In my tests
/// RGBA8 and RGBA16F work on Chrome and Firefox.
/// For a list of supposedly working ones see:
/// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum OutputBufferFormat {
    R8,
    R8_SNORM,
    R16F,
    R32F,
    R8UI,
    R8I,
    R16UI,
    R16I,
    R32UI,
    R32I,
    RG8,
    RG8_SNORM,
    RG16F,
    RG32F,
    RG8UI,
    RG8I,
    RG16UI,
    RG16I,
    RG32UI,
    RG32I,
    RGB8,
    SRGB8,
    RGB565,
    RGB8_SNORM,
    R11F_G11F_B10F,
    RGB9_E5,
    RGB16F,
    RGB32F,
    RGB8UI,
    RGB8I,
    RGB16UI,
    RGB16I,
    RGB32UI,
    RGB32I,
    RGBA8,
    SRGB8_ALPHA8,
    RGBA8_SNORM,
    RGB5_A1,
    RGBA4,
    RGB10_A2,
    RGBA16F,
    RGBA32F,
    RGBA8UI,
    RGBA8I,
    RGB10_A2UI,
    RGBA16UI,
    RGBA16I,
    RGBA32I,
    RGBA32UI,
}
impl OutputBufferFormat {
    pub fn to_sized_internal_format(&self) -> u32 {
        match self {
            Self::R8 => glow::R8,
            Self::R8_SNORM => glow::R8_SNORM,
            Self::R16F => glow::R16F,
            Self::R32F => glow::R32F,
            Self::R8UI => glow::R8UI,
            Self::R8I => glow::R8I,
            Self::R16UI => glow::R16UI,
            Self::R16I => glow::R16I,
            Self::R32UI => glow::R32UI,
            Self::R32I => glow::R32I,
            Self::RG8 => glow::RG8,
            Self::RG8_SNORM => glow::RG8_SNORM,
            Self::RG16F => glow::RG16F,
            Self::RG32F => glow::RG32F,
            Self::RG8UI => glow::RG8UI,
            Self::RG8I => glow::RG8I,
            Self::RG16UI => glow::RG16UI,
            Self::RG16I => glow::RG16I,
            Self::RG32UI => glow::RG32UI,
            Self::RG32I => glow::RG32I,
            Self::RGB8 => glow::RGB8,
            Self::SRGB8 => glow::SRGB8,
            Self::RGB565 => glow::RGB565,
            Self::RGB8_SNORM => glow::RGB8_SNORM,
            Self::R11F_G11F_B10F => glow::R11F_G11F_B10F,
            Self::RGB9_E5 => glow::RGB9_E5,
            Self::RGB16F => glow::RGB16F,
            Self::RGB32F => glow::RGB32F,
            Self::RGB8UI => glow::RGB8UI,
            Self::RGB8I => glow::RGB8I,
            Self::RGB16UI => glow::RGB16UI,
            Self::RGB16I => glow::RGB16I,
            Self::RGB32UI => glow::RGB32UI,
            Self::RGB32I => glow::RGB32I,
            Self::RGBA8 => glow::RGBA8,
            Self::SRGB8_ALPHA8 => glow::SRGB8_ALPHA8,
            Self::RGBA8_SNORM => glow::RGBA8_SNORM,
            Self::RGB5_A1 => glow::RGB5_A1,
            Self::RGBA4 => glow::RGBA4,
            Self::RGB10_A2 => glow::RGB10_A2,
            Self::RGBA16F => glow::RGBA16F,
            Self::RGBA32F => glow::RGBA32F,
            Self::RGBA8UI => glow::RGBA8UI,
            Self::RGBA8I => glow::RGBA8I,
            Self::RGB10_A2UI => glow::RGB10_A2UI,
            Self::RGBA16UI => glow::RGBA16UI,
            Self::RGBA16I => glow::RGBA16I,
            Self::RGBA32I => glow::RGBA32I,
            Self::RGBA32UI => glow::RGBA32UI,
        }
    }

    pub fn to_format(&self) -> u32 {
        match self {
            Self::R8 => glow::RED,
            Self::R8_SNORM => glow::RED,
            Self::R16F => glow::RED,
            Self::R32F => glow::RED,
            Self::R8UI => glow::RED_INTEGER,
            Self::R8I => glow::RED_INTEGER,
            Self::R16UI => glow::RED_INTEGER,
            Self::R16I => glow::RED_INTEGER,
            Self::R32UI => glow::RED_INTEGER,
            Self::R32I => glow::RED_INTEGER,
            Self::RG8 => glow::RG,
            Self::RG8_SNORM => glow::RG,
            Self::RG16F => glow::RG,
            Self::RG32F => glow::RG,
            Self::RG8UI => glow::RG_INTEGER,
            Self::RG8I => glow::RG_INTEGER,
            Self::RG16UI => glow::RG_INTEGER,
            Self::RG16I => glow::RG_INTEGER,
            Self::RG32UI => glow::RG_INTEGER,
            Self::RG32I => glow::RG_INTEGER,
            Self::RGB8 => glow::RGB,
            Self::SRGB8 => glow::RGB,
            Self::RGB565 => glow::RGB,
            Self::RGB8_SNORM => glow::RGB,
            Self::R11F_G11F_B10F => glow::RGB,
            Self::RGB9_E5 => glow::RGB,
            Self::RGB16F => glow::RGB,
            Self::RGB32F => glow::RGB,
            Self::RGB8UI => glow::RGB_INTEGER,
            Self::RGB8I => glow::RGB_INTEGER,
            Self::RGB16UI => glow::RGB_INTEGER,
            Self::RGB16I => glow::RGB_INTEGER,
            Self::RGB32UI => glow::RGB_INTEGER,
            Self::RGB32I => glow::RGB_INTEGER,
            Self::RGBA8 => glow::RGBA,
            Self::SRGB8_ALPHA8 => glow::RGBA,
            Self::RGBA8_SNORM => glow::RGBA,
            Self::RGB5_A1 => glow::RGBA,
            Self::RGBA4 => glow::RGBA,
            Self::RGB10_A2 => glow::RGBA,
            Self::RGBA16F => glow::RGBA,
            Self::RGBA32F => glow::RGBA,
            Self::RGBA8UI => glow::RGBA_INTEGER,
            Self::RGBA8I => glow::RGBA_INTEGER,
            Self::RGB10_A2UI => glow::RGBA_INTEGER,
            Self::RGBA16UI => glow::RGBA_INTEGER,
            Self::RGBA16I => glow::RGBA_INTEGER,
            Self::RGBA32I => glow::RGBA_INTEGER,
            Self::RGBA32UI => glow::RGBA_INTEGER,
        }
    }

    pub fn to_type(&self) -> u32 {
        match self {
            Self::R8 => glow::UNSIGNED_BYTE,
            Self::R8_SNORM => glow::BYTE,
            Self::R16F => glow::HALF_FLOAT,
            Self::R32F => glow::FLOAT,
            Self::R8UI => glow::UNSIGNED_BYTE,
            Self::R8I => glow::BYTE,
            Self::R16UI => glow::UNSIGNED_SHORT,
            Self::R16I => glow::SHORT,
            Self::R32UI => glow::UNSIGNED_INT,
            Self::R32I => glow::INT,
            Self::RG8 => glow::UNSIGNED_BYTE,
            Self::RG8_SNORM => glow::BYTE,
            Self::RG16F => glow::FLOAT,
            Self::RG32F => glow::FLOAT,
            Self::RG8UI => glow::UNSIGNED_BYTE,
            Self::RG8I => glow::BYTE,
            Self::RG16UI => glow::UNSIGNED_SHORT,
            Self::RG16I => glow::SHORT,
            Self::RG32UI => glow::UNSIGNED_INT,
            Self::RG32I => glow::INT,
            Self::RGB8 => glow::UNSIGNED_BYTE,
            Self::SRGB8 => glow::UNSIGNED_BYTE,
            Self::RGB565 => glow::UNSIGNED_SHORT_5_6_5,
            Self::RGB8_SNORM => glow::BYTE,
            Self::R11F_G11F_B10F => glow::UNSIGNED_INT_10F_11F_11F_REV,
            Self::RGB9_E5 => glow::UNSIGNED_INT_5_9_9_9_REV,
            Self::RGB16F => glow::HALF_FLOAT,
            Self::RGB32F => glow::FLOAT,
            Self::RGB8UI => glow::UNSIGNED_BYTE,
            Self::RGB8I => glow::BYTE,
            Self::RGB16UI => glow::UNSIGNED_SHORT,
            Self::RGB16I => glow::SHORT,
            Self::RGB32UI => glow::UNSIGNED_INT,
            Self::RGB32I => glow::INT,
            Self::RGBA8 => glow::UNSIGNED_BYTE,
            Self::SRGB8_ALPHA8 => glow::UNSIGNED_BYTE,
            Self::RGBA8_SNORM => glow::BYTE,
            Self::RGB5_A1 => glow::UNSIGNED_INT_2_10_10_10_REV,
            Self::RGBA4 => glow::UNSIGNED_SHORT_4_4_4_4,
            Self::RGB10_A2 => glow::UNSIGNED_INT_2_10_10_10_REV,
            Self::RGBA16F => glow::HALF_FLOAT,
            Self::RGBA32F => glow::FLOAT,
            Self::RGBA8UI => glow::UNSIGNED_BYTE,
            Self::RGBA8I => glow::BYTE,
            Self::RGB10_A2UI => glow::UNSIGNED_INT_2_10_10_10_REV,
            Self::RGBA16UI => glow::UNSIGNED_SHORT,
            Self::RGBA16I => glow::SHORT,
            Self::RGBA32I => glow::INT,
            Self::RGBA32UI => glow::UNSIGNED_INT,
        }
    }
}
