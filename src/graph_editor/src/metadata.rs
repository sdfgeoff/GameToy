use super::state::{Reactor, StateOperation};
use gametoy::config_file::MetaData;

pub fn draw_metadata(ui: &mut egui::Ui, metadata: &MetaData, reactor: &mut Reactor) {
    let mut new_metadata = metadata.clone();
    egui::Grid::new("metadata_grid")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Game Name:");
            ui.text_edit_singleline(&mut new_metadata.game_name)
                .on_hover_text("Name of the game");
            ui.end_row();

            ui.label("Game Version:");
            ui.text_edit_singleline(&mut new_metadata.game_version)
                .on_hover_text("I suggest using semantic versioning: {major}.{minor}.{patch}");
            ui.end_row();

            ui.label("Release Date:");
            ui.text_edit_singleline(&mut new_metadata.release_date)
                .on_hover_text("Date of release of this game version");
            ui.end_row();

            ui.label("Game Website:");
            ui.text_edit_singleline(&mut new_metadata.website)
                .on_hover_text("Website to find out more about this game");
            ui.end_row();

            ui.label("Author:");
            ui.text_edit_singleline(&mut new_metadata.author_name)
                .on_hover_text("Who made this game");
            ui.end_row();

            ui.label("License:");
            ui.text_edit_singleline(&mut new_metadata.license)
                .on_hover_text("What license do you release this game under?");
            ui.end_row();
        });
    if &new_metadata != metadata {
        reactor.queue_operation(StateOperation::SetMetadata(new_metadata));
    }
}
