use std::io::Error;
use std::path::PathBuf;

use tracing::{debug, event, info, Level, span};

use crate::enums::fs::ProjectDirectories;

#[derive(Debug, Default)]
pub struct ProjectFileSystem {
    pub home_directory: ProjectDirectories,
    config_directory: ProjectDirectories,
    pub data_directory: ProjectDirectories,
    log_directory: ProjectDirectories,
    state_directory: ProjectDirectories,
    cache_directory: ProjectDirectories,
}
impl ProjectFileSystem {
    pub fn new() -> Self {
        let span = span!(Level::INFO, "Initializer.new");
        let _enter = span.enter();
        info!("Initializing the Initializer struct");
        let new_init = ProjectFileSystem {
            home_directory: ProjectDirectories::Home,
            config_directory: ProjectDirectories::Config,
            data_directory: ProjectDirectories::Data,
            log_directory: ProjectDirectories::Log,
            state_directory: ProjectDirectories::State,
            cache_directory: ProjectDirectories::Cache,
        };
        info!(
            "Initializer struct has been initialized. Home directory set to: {:?}",
            new_init.home_directory.path()
        );
        new_init
    }
    pub fn init(&self) {
        let dir_vec = vec![
            self.home_directory.path(),
            self.config_directory.path(),
            self.data_directory.path(),
            self.log_directory.path(),
            self.state_directory.path(),
            self.cache_directory.path(),
        ];
        let span = span!(Level::INFO, "Initializer.init");
        let _enter = span.enter();
        for dir in dir_vec {
            match self.initialize_directory(&dir.clone()) {
                Ok(_) => {
                    debug!(
                        "Directory {:?} was successfully created.",
                        dir.clone().to_str().unwrap()
                    );
                }
                Err(e) => {
                    debug!(
                        "Directory {:?} was not created. Error: {:?}",
                        dir.clone().to_str().unwrap(),
                        e
                    );
                }
            };
        }
    }
    pub fn initialize_directory(&self, directory_path: &PathBuf) -> Result<bool, Error> {
        let span = span!(
            Level::INFO,
            "Initializer.create_directory",
            value = directory_path.clone().to_str().unwrap()
        );
        let _enter = span.enter();
        event!(
            Level::INFO,
            "Attempting to create the following directory: {:?}",
            directory_path.clone().to_str().unwrap()
        );
        if !directory_path.exists() {
            match self.create_directory(directory_path.clone()) {
                Ok(_) => Ok(true),
                Err(e) => Err(e)
            }
        } else {
            event!(
                Level::INFO,
                "Skipping directory creation because it already exists."
            );
            Ok(false)
        }
    }
    fn create_directory(&self, directory_path: PathBuf) -> Result<(), Error> {
        let span = span!(
            Level::INFO,
            "Initializer.create_directory",
            value = directory_path.clone().to_str().unwrap() //tarpaulin ignore
        );
        let _enter = span.enter();
        return match std::fs::create_dir(directory_path.clone()) {
            Ok(_) => {
                event!(Level::INFO,
                    "{:?} was successfully created.",
                    directory_path.clone().to_str().unwrap()
                );
                Ok(())
            }
            Err(e) => {
                event!(Level::DEBUG,
                    "Unable to create the following directory: {:?}",
                    directory_path.clone().to_str().unwrap()
                );
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::ErrorKind;

    use super::*;

    #[test]
    fn test_default() {
        let init = ProjectFileSystem::default();
        println!("{:?}", init);
        assert_eq!(init.home_directory.path(), ProjectDirectories::Home.path());
    }

    #[test]
    fn test_new() {
        let init = ProjectFileSystem::new();
        assert_eq!(init.home_directory.path(), ProjectDirectories::Home.path());
    }

    #[test]
    fn test_initialize_directory() {
        let init = ProjectFileSystem::default();
        let dir = init.home_directory.path();
        let test_dir = dir.clone().join("test_dir");
        match init.initialize_directory(&test_dir.clone()) {
            Ok(was_created) => {
                assert!(was_created);
            }
            Err(e) => {
                assert_eq!(e.kind(), ErrorKind::AlreadyExists);
            }
        };
        assert!(test_dir.exists());
        match init.initialize_directory(&test_dir.clone()) {
            Ok(was_created) => {
                assert!(!was_created);
            }
            Err(e) => {
                assert_eq!(e.kind(), ErrorKind::AlreadyExists);
            }
        };
        fs::remove_dir(test_dir).unwrap();
    }
}
