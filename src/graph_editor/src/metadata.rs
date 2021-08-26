use gametoy::config_file::ConfigFile;

pub fn draw_metadata(new_proj: &mut ConfigFile, ui: &mut egui::Ui) {
    egui::Grid::new("metadata_grid")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Game Name:");
            ui.text_edit_singleline(&mut new_proj.metadata.game_name)
                .on_hover_text("Name of the game");
            ui.end_row();

            ui.label("Game Version:");
            ui.text_edit_singleline(&mut new_proj.metadata.game_version)
                .on_hover_text("I suggest using semantic versioning: {major}.{minor}.{patch}");
            ui.end_row();

            ui.label("Release Date:");
            ui.text_edit_singleline(&mut new_proj.metadata.release_date)
                .on_hover_text("Date of release of this game version");
            ui.end_row();

            ui.label("Game Website:");
            ui.text_edit_singleline(&mut new_proj.metadata.website)
                .on_hover_text("Website to find out more about this game");
            ui.end_row();

            ui.label("Author:");
            ui.text_edit_singleline(&mut new_proj.metadata.author_name)
                .on_hover_text("Who made this game");
            ui.end_row();

            ui.label("License:");
            ui.text_edit_singleline(&mut new_proj.metadata.license)
                .on_hover_text("What license do you release this game under?");
            ui.end_row();
        });
}
