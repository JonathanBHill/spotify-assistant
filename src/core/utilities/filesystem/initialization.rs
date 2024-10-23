use std::path::PathBuf;

use tracing::{debug, event, info, Level, span};

use crate::core::enums::fs::ProjectDirectories;

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
        info!("Initializer struct has been initialized. Home directory set to: {:?}", new_init.home_directory.path());
        new_init
    }
    pub fn init(&self) {
        let dir_vec = vec![
            self.home_directory.path(), self.config_directory.path(),
            self.data_directory.path(), self.log_directory.path(),
            self.state_directory.path(), self.cache_directory.path(),
        ];
        let span = span!(Level::INFO, "Initializer.init");
        let _enter = span.enter();
        for dir in dir_vec {
            self.create_directory(dir.clone());
        }
    }
    pub fn create_directory(&self, directory_path: PathBuf) {
        let span = span!(Level::INFO, "Initializer.create_directory", value = directory_path.clone().to_str().unwrap());
        let _enter = span.enter();
        event!(Level::INFO, "Attempting to create the following directory: {:?}", directory_path.clone().to_str().unwrap());
        if !directory_path.exists() {
            std::fs::create_dir_all(directory_path.clone()).unwrap_or_else(|_| 
                debug!("Unable to create the following directory: {:?}", directory_path.clone().to_str().unwrap())
            );
            event!(Level::INFO, "{:?} was successfully created.", directory_path.clone().to_str().unwrap());
        } else {
            event!(Level::INFO, "Skipping directory creation because it already exists.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_traits() {
        let init = ProjectFileSystem::default();
        println!("{:?}", init);
        assert_eq!(init.home_directory.path(), ProjectDirectories::Home.path());
    }
    #[test]
    fn test_get_home_directory() {
        let init = ProjectFileSystem::new();
        std::fs::remove_dir(init.config_directory.path()).expect("Unable to remove the directory");
        assert!(!init.config_directory.path().exists());
        init.init();
        assert!(init.config_directory.path().exists());
        // let home_dir = init.home_directory.path();
        // println!("{:?}", home_dir);
        // assert_eq!(home_dir.exists(), true);
    }
}
