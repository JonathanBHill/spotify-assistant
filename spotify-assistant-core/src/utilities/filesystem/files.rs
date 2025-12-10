use std::fs;
use std::path::{Path, PathBuf};

use tracing::{span, Level};

use crate::utilities::filesystem::initialization::ProjectFileSystem;

/// A struct representing the directories associated with a project's file system.
///
/// The `ProjectFiles` struct provides a way to store and manage
/// paths to the data and configuration directories of a project.
///
/// # Fields
///
/// * `data_directory` - A `PathBuf` representing the directory
///   where project-specific data files are stored.
///
/// * `config_directory` - A `PathBuf` representing the directory
///   where configuration files for the project are stored.
///
pub struct ProjectFiles {
    data_directory: PathBuf,
    config_directory: PathBuf,
}

impl Default for ProjectFiles {
    fn default() -> Self {
        let pfs = ProjectFileSystem::new();
        ProjectFiles {
            data_directory: pfs.data_directory.path(),
            config_directory: pfs.config_directory.path(),
        }
    }
}

impl ProjectFiles {
    /// Returns a reference to the data directory path.
    ///
    /// This method provides access to the path representing the data directory
    /// associated with the current instance.
    ///
    /// # Returns
    /// A reference to a `Path` that represents the data directory.
    ///
    /// # Example
    /// ```
    /// use spotify_assistant_core::utilities::filesystem::files::ProjectFiles;
    /// let instance = ProjectFiles::default();
    /// let data_dir = instance.data_directory();
    /// println!("Data directory: {:?}", data_dir);
    /// ```
    pub fn data_directory(&self) -> &Path {
        &self.data_directory
    }

    /// Returns a reference to the path of the configuration directory.
    ///
    /// # Example
    ///
    /// ```
    /// use spotify_assistant_core::utilities::filesystem::files::ProjectFiles;
    /// let instance = ProjectFiles::default();
    /// let config_dir = instance.config_directory();
    /// println!("Configuration directory: {:?}", config_dir);
    /// ```
    ///
    /// # Returns
    /// A reference to a `Path` representing the configuration directory.
    pub fn config_directory(&self) -> &Path {
        &self.config_directory
    }

    /// Processes historical Spotify data stored in the specified data directory by renaming
    /// all files to replace spaces in their names with underscores. Returns a vector of
    /// `PathBuf` objects representing the new file paths.
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    /// - `Ok(Vec<PathBuf>)`: A vector of `PathBuf` objects pointing to renamed files.
    /// - `Err(anyhow::Error)`: An error if reading the directory or renaming the files fails.
    ///
    /// # Errors
    ///
    /// - Returns an error if the directory at `self.data_directory` cannot be read.
    /// - **Panics** if encountering a file that cannot be processed (e.g., invalid/unreadable names).
    /// - **Panics** if file renaming fails due to any reason (e.g., permission issues).
    ///
    /// # Example
    ///
    /// Assume `data_directory` contains the files `example 1.mp3` and `example 2.mp3`.
    /// Calling this function will rename these files to `example_1.mp3` and `example_2.mp3`.
    /// The function returns a vector of paths to the renamed files.
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use spotify_assistant_core::utilities::filesystem::files::ProjectFiles;
    ///
    /// let instance = ProjectFiles::default();
    /// let paths = instance.spotify_historical_data().unwrap();
    /// for path in paths {
    ///     println!("{:?}", path);
    /// }
    /// ```
    ///
    /// This would yield outputs similar to:
    /// ```text
    /// PathBuf("path/to/data_directory/example_1.mp3")
    /// PathBuf("path/to/data_directory/example_2.mp3")
    /// ```
    ///
    /// # Notes
    ///
    /// - The function logs information using a telemetry span, labeled `FileRename.spotify_historical_data`.
    /// - Any invalid file names or paths will trigger an error or panic.
    /// - Spaces in file names are replaced by underscores (`_`).
    ///
    /// # Dependencies
    ///
    /// - The function depends on the `fs` module for file system operations.
    /// - The `tracing` crate is used for telemetry.
    pub fn spotify_historical_data(&self) -> anyhow::Result<Vec<PathBuf>> {
        let span = span!(Level::INFO, "FileRename.spotify_historical_data");
        let _enter = span.enter();
        let files = fs::read_dir(&self.data_directory)?;
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
            let old_path = Path::new(&self.data_directory).join(&file);
            let new_path = Path::new(&self.data_directory).join(&new_file);
            fs::rename(old_path, new_path.clone()).expect("Failed to edit file name");
            new_path
        }).collect::<Vec<PathBuf>>();
        Ok(file_paths)
    }
}
