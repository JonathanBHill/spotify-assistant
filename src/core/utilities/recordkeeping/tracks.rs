use std::fs;
use std::path::PathBuf;

use rspotify::model::FullTrack;
use serde::{Deserialize, Serialize};

use crate::core::utilities::filesystem::initialization::ProjectFileSystem;

#[derive(Serialize, Deserialize)]
enum ETrackIO {
    TopTracks
}
impl ETrackIO {
    pub fn file_location(&self) -> PathBuf {
        let proj_fs = ProjectFileSystem::new();
        let data_dir = proj_fs.data_directory.path();
        let location = match self {
            ETrackIO::TopTracks => {
                let tm = chrono::Local::now(); //.format("%M-%D-%Y");
                let formatted = format!("{}", tm.format("%m-%d-%y"));
                let join_path = format!("TopTracks/{formatted}.yaml");
                data_dir.join(join_path)
            }
            _ => {
                panic!("Not yet implemented")
            }
        };
        location

    }
}

#[derive(Serialize, Deserialize)]
pub struct TracksIO {
    // tracks: Vec<FullTrack>,
    write_type: ETrackIO
}
impl TracksIO {
    pub fn new(io_type: String) -> Self {
        let write_type = match io_type.as_str() {
            "toptracks" => ETrackIO::TopTracks,
            _ => ETrackIO::TopTracks
        };
        TracksIO {
            // tracks,
            write_type
        }
    }
    pub fn serialize(&self, tracks: &Vec<FullTrack>) {
        let yaml_string = match serde_yaml::to_string(tracks) {
            Ok(stringed) => stringed,
            Err(error) => panic!("Could not serialize: {error}")
        };
        println!("File location: {:?}", self.write_type.file_location());
        fs::write(self.write_type.file_location(), yaml_string).expect("Could not serialize");
    }
    pub fn deserialize(&self) -> Vec<FullTrack> {
        let file_location = self.write_type.file_location();
        let fs_str = fs::read_to_string(file_location).expect("Couldn't read");
        
        let des: Vec<FullTrack> = serde_yaml::from_str(&fs_str).expect("Couldn't convert to full track");
        des
    }
}
