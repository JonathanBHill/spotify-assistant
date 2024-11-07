use std::path::PathBuf;

#[derive(Debug, Default, PartialEq)]
pub enum ProjectDirectories {
    #[default]
    Home,
    Config,
    Data,
    Cache,
    Log,
    State,
    Preferences,
    Template,
}

impl ProjectDirectories {
    #[cfg(target_os = "linux")]
    pub fn path(&self) -> PathBuf {
        let dir = directories::BaseDirs::new().expect("Could not get base directories");
        let pdir = directories::ProjectDirs::from("com", "spotify-assistant", "spotify-assistant")
            .expect("Could not find project directories");
        let directory_path = match self {
            ProjectDirectories::Home => dir.home_dir(),
            ProjectDirectories::Config => pdir.config_dir(),
            ProjectDirectories::Data => pdir.data_dir(),
            ProjectDirectories::Cache => pdir.cache_dir(),
            ProjectDirectories::Log => pdir.data_dir(),
            ProjectDirectories::State => pdir.state_dir().unwrap(),
            ProjectDirectories::Preferences => pdir.preference_dir(),
            ProjectDirectories::Template => &*dir.home_dir().join("Templates"),
        };
        directory_path.to_path_buf()
    }
}

pub enum ProjectFiles {
    DotEnv,
    TokenCache,
}

impl ProjectFiles {
    pub fn path(&self) -> PathBuf {
        match self {
            ProjectFiles::DotEnv => ProjectDirectories::Config.path().join(".env"),
            ProjectFiles::TokenCache => ProjectDirectories::Cache.path().join("token_cache"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// use crate::enums::fs::ProjectDirectories;
    // use crate::enums::pl::PlaylistType;

    #[test]
    fn test_path() {
        let home = ProjectDirectories::Home.path();
        let config = ProjectDirectories::Config.path();
        let data = ProjectDirectories::Data.path();
        let cache = ProjectDirectories::Cache.path();
        let log = ProjectDirectories::Log.path();
        let state = ProjectDirectories::State.path();
        let preferences = ProjectDirectories::Preferences.path();
        let template = ProjectDirectories::Template.path();
        if !config.exists() {
            assert!(data.exists());
            assert!(cache.exists());
            assert!(log.exists());
            assert!(state.exists());
            assert!(preferences.exists());
            assert!(template.exists());
        } else {
            println!("Project directories have not yet been created. Assert statements for the existence of other directories will be skipped");
        }
        assert!(home.exists());
    }
    #[test]
    fn test_default() {
        let default = ProjectDirectories::default();
        assert_eq!(default, ProjectDirectories::Home);
    }

    #[test]
    fn test_project_files_types() {
        let dot_env = ProjectFiles::DotEnv.path();
        let token_cache = ProjectFiles::TokenCache.path();
        assert_eq!(dot_env.parent().unwrap(), ProjectDirectories::Config.path());
        assert_eq!(token_cache.parent().unwrap(), ProjectDirectories::Cache.path());
    }
}
