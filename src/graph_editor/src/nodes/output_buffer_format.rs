use gametoy::config_file::OutputBufferFormat;

pub fn output_buffer_format_selector(
    ui: &mut egui::Ui,
    pixel_format: &mut OutputBufferFormat,
    slot_id: usize,
) {
    egui::ComboBox::from_id_source(format!("out_format_pixel{}", slot_id))
        .selected_text(pixel_format_to_str(pixel_format))
        .show_ui(ui, |ui| {
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R8);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R8_SNORM);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R16F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R32F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R8UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R8I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R16UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R16I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R32UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R32I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG8);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG8_SNORM);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG16F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG32F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG8UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG8I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG16UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG16I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG32UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RG32I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB8);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::SRGB8);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB565);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB8_SNORM);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::R11F_G11F_B10F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB9_E5);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB16F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB32F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB8UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB8I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB16UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB16I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB32UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB32I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA8);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::SRGB8_ALPHA8);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA8_SNORM);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB5_A1);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA4);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB10_A2);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA16F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA32F);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA8UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA8I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGB10_A2UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA16UI);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA16I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA32I);
            pixel_format_entry(ui, pixel_format, OutputBufferFormat::RGBA32UI);
        });
}

fn pixel_format_entry(
    ui: &mut egui::Ui,
    current_format: &mut OutputBufferFormat,
    this_format: OutputBufferFormat,
) {
    let name = pixel_format_to_str(&this_format);
    ui.selectable_value(current_format, this_format, name);
}

fn pixel_format_to_str(this_format: &OutputBufferFormat) -> &'static str {
    match this_format {
        OutputBufferFormat::R8 => "R8",
        OutputBufferFormat::R8_SNORM => "R8_SNORM",
        OutputBufferFormat::R16F => "R16F",
        OutputBufferFormat::R32F => "R32F",
        OutputBufferFormat::R8UI => "R8UI",
        OutputBufferFormat::R8I => "R8I",
        OutputBufferFormat::R16UI => "R16UI",
        OutputBufferFormat::R16I => "R16I",
        OutputBufferFormat::R32UI => "R32UI",
        OutputBufferFormat::R32I => "R32I",
        OutputBufferFormat::RG8 => "RG8",
        OutputBufferFormat::RG8_SNORM => "RG8_SNORM",
        OutputBufferFormat::RG16F => "RG16F",
        OutputBufferFormat::RG32F => "RG32F",
        OutputBufferFormat::RG8UI => "RG8UI",
        OutputBufferFormat::RG8I => "RG8I",
        OutputBufferFormat::RG16UI => "RG16UI",
        OutputBufferFormat::RG16I => "RG16I",
        OutputBufferFormat::RG32UI => "RG32UI",
        OutputBufferFormat::RG32I => "RG32I",
        OutputBufferFormat::RGB8 => "RGB8",
        OutputBufferFormat::SRGB8 => "SRGB8",
        OutputBufferFormat::RGB565 => "RGB565",
        OutputBufferFormat::RGB8_SNORM => "RGB8_SNORM",
        OutputBufferFormat::R11F_G11F_B10F => "R11F_G11F_B10F",
        OutputBufferFormat::RGB9_E5 => "RGB9_E5",
        OutputBufferFormat::RGB16F => "RGB16F",
        OutputBufferFormat::RGB32F => "RGB32F",
        OutputBufferFormat::RGB8UI => "RGB8UI",
        OutputBufferFormat::RGB8I => "RGB8I",
        OutputBufferFormat::RGB16UI => "RGB16UI",
        OutputBufferFormat::RGB16I => "RGB16I",
        OutputBufferFormat::RGB32UI => "RGB32UI",
        OutputBufferFormat::RGB32I => "RGB32I",
        OutputBufferFormat::RGBA8 => "RGBA8",
        OutputBufferFormat::SRGB8_ALPHA8 => "SRGB8_ALPHA8",
        OutputBufferFormat::RGBA8_SNORM => "RGBA8_SNORM",
        OutputBufferFormat::RGB5_A1 => "RGB5_A1",
        OutputBufferFormat::RGBA4 => "RGBA4",
        OutputBufferFormat::RGB10_A2 => "RGB10_A2",
        OutputBufferFormat::RGBA16F => "RGBA16F",
        OutputBufferFormat::RGBA32F => "RGBA32F",
        OutputBufferFormat::RGBA8UI => "RGBA8UI",
        OutputBufferFormat::RGBA8I => "RGBA8I",
        OutputBufferFormat::RGB10_A2UI => "RGB10_A2UI",
        OutputBufferFormat::RGBA16UI => "RGBA16UI",
        OutputBufferFormat::RGBA16I => "RGBA16I",
        OutputBufferFormat::RGBA32I => "RGBA32I",
        OutputBufferFormat::RGBA32UI => "RGBA32UI",
    }
}
