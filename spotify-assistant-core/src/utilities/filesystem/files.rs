use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use tracing::{Level, span};

use crate::utilities::filesystem::initialization::ProjectFileSystem;

pub struct ProjectFiles {
    pub directory: PathBuf,
}

impl ProjectFiles {
    pub fn new() -> Self {
        let pfs = ProjectFileSystem::new();
        ProjectFiles {
            directory: pfs.data_directory.path(),
        }
    }
    pub fn spotify_historical_data(&self) -> anyhow::Result<Vec<PathBuf>> {
        let span = span!(Level::INFO, "FileRename.spotify_historical_data");
        let _enter = span.enter();
        let mut files = fs::read_dir(&self.directory)?;
        let file_paths = files.into_iter().map(|file| {
            let directory = match file {
                Ok(dir) => { dir },
                _ => { panic!("Error") }
            };
            let file_name = directory.file_name().clone();

            let file = match file_name.into_string() {
                Ok(string) => { string }
                Err(osstring) => {
                    let inter = osstring.to_str().ok_or("Invalid OsString").unwrap();
                    inter.to_string()
                }
            };
            let new_file = file.replace(" ", "_");
            let old_path = Path::new(&self.directory).join(&file);
            let new_path = Path::new(&self.directory).join(&new_file);
            fs::rename(old_path, new_path.clone()).expect("Failed to edit file name");
            new_path
        }).collect::<Vec<PathBuf>>();
        Ok(file_paths)
    }
}
