use serde_json;
use std::collections::HashMap;
use std::io::Read;

use super::config_file;

#[derive(Debug)]
pub enum GameDataError {
    TarError(std::io::Error),
    ConfigFileParseError(serde_json::Error),
    MissingConfigFile,
}

pub struct GameData {
    pub config_file: config_file::ConfigFile,
    pub textures: HashMap<String, Vec<u8>>,
    pub shader_sources: HashMap<String, String>,
}

impl GameData {
    pub fn from_tar<R>(mut data: tar::Archive<R>) -> Result<Self, GameDataError>
    where
        R: Read,
    {
        let mut config_file: Option<config_file::ConfigFile> = None;
        let mut textures = HashMap::new();
        let mut shader_sources = HashMap::new();

        let entries = data.entries().map_err(GameDataError::TarError)?;
        for file in entries {
            // Make sure there wasn't an I/O error
            let mut file = file.map_err(GameDataError::TarError)?;

            // Inspect metadata about the file

            let filename_string = {
                let filename = file.header().path().map_err(GameDataError::TarError)?;
                filename.to_str().unwrap().to_string()
            };
            println!("Loading {:?}", filename_string);

            if filename_string == "data.json" {
                //config_file = ConfigFile::
                // files implement the Read trait
                //let mut s = String::new();
                //file.read_to_string(&mut s).unwrap();

                config_file = Some(
                    serde_json::from_reader(file).map_err(GameDataError::ConfigFileParseError)?,
                );
            } else if filename_string.ends_with(".frag") {
                let file_string = {
                    let mut s = String::new();
                    file.read_to_string(&mut s).unwrap();
                    s
                };
                shader_sources.insert(filename_string, file_string);
            } else if filename_string.ends_with(".png") {
                let file_data = {
                    let mut d = Vec::new();
                    file.read_to_end(&mut d).unwrap();
                    d
                };
                textures.insert(filename_string, file_data);
            }
        }

        let config_file = config_file.ok_or(GameDataError::MissingConfigFile)?;

        Ok(Self {
            config_file,
            textures,
            shader_sources,
        })
    }
}
