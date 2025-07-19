use std::path::PathBuf;

/// Represents different directories commonly used by applications for storing
/// files or data specific to the application.
///
/// The `ProjectDirectories` enum provides variants for handling various 
/// standard directories. These variants can be used to organize and manage 
/// application-specific files appropriately, depending on their purpose.
///
/// # Variants
///
/// * `Home` - Refers to the user's home directory. This is the default variant.
/// * `Config` - Used for application configuration files.
/// * `Data` - Represents a directory for storing application data files.
/// * `Cache` - Refers to the directory designated for storing cache files.
/// * `Log` - Represents the directory used for storing log files.
/// * `State` - Refers to a directory where application state data can be stored.
/// * `Preferences` - Represents a directory for user or application preference files.
/// * `Template` - Refers to the directory containing templates used by the application.
///
/// # Traits Implemented
///
/// * `Debug` - Enables debugging information for the enum.
/// * `Default` - Adds a default implementation, which is set to `Home`.
/// * `PartialEq` - Allows comparison of enum variants for equality.
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
    /// Returns the file system path corresponding to the specified directory type for the current operating system.
    ///
    /// This function is only available on Linux due to the use of the `#[cfg(target_os = "linux")]` attribute.
    /// It leverages the `directories` crate to retrieve base and project-specific directories.
    ///
    /// # Directory Mapping
    /// Depending on the `ProjectDirectories` variant provided, this function returns the following paths:
    /// - `ProjectDirectories::Home`: Returns the user's home directory.
    /// - `ProjectDirectories::Config`: Returns the configuration directory for the project.
    /// - `ProjectDirectories::Data`: Returns the data directory for the project.
    /// - `ProjectDirectories::Cache`: Returns the cache directory for the project.
    /// - `ProjectDirectories::Log`: Returns the data directory for logging purposes.
    /// - `ProjectDirectories::State`: Returns the state directory for the project.
    /// - `ProjectDirectories::Preferences`: Returns the preference directory for the project.
    /// - `ProjectDirectories::Template`: Returns the `Templates` directory located within the user's home directory.
    ///
    /// # Errors
    /// - If the `directories::BaseDirs::new` function fails to retrieve base directories, the function will panic with the message
    ///   `"Could not get base directories"`.
    /// - If the `directories::ProjectDirs::from` function fails to identify the project directories using the provided application
    ///   identifiers (organization name: "com", application qualifier: "spotify-assistant", application name: "spotify-assistant"), 
    ///   the function will panic with the message `"Could not find project directories"`.
    /// - If the `ProjectDirectories::State` variant is passed but the underlying call to `state_dir` returns `None`, the function will panic.
    ///
    /// # Returns
    /// A `PathBuf` representing the resolved directory path for the given `ProjectDirectories` variant.
    ///
    /// # Panics
    /// The function panics in one of the following scenarios:
    /// - If the operation to retrieve base directories fails.
    /// - If the operation to retrieve project directories with the given identifiers ("com", "spotify-assistant", "spotify-assistant") fails.
    /// - If the state directory (`state_dir`) cannot be retrieved (only in the case of `ProjectDirectories::State`).
    ///
    /// # Example
    /// ```ignore
    /// use std::path::PathBuf;
    ///
    /// // Example usage of the path function
    /// let directory_path: PathBuf = some_project_directories_enum.path();
    /// println!("Resolved Path: {:?}", directory_path);
    /// ```
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

/// An enumeration representing different types of project-related files.
///
/// `ProjectFiles` provides a categorized representation of common files
/// used within a project. This can be useful for managing or referencing
/// specific types of files programmatically.
///
/// # Variants
///
/// - `DotEnv`
///   Represents a `.env` file, which is commonly used for storing
///   environment variables in key-value pairs. This file often contains
///   sensitive configuration details such as database credentials or API keys.
///
/// - `TokenCache`
///   Represents a token cache file used for temporarily storing tokens,
///   such as authentication tokens, to avoid repeated regeneration. Useful
///   for optimizing processes that require token-based authentication.
///
/// # Example
///
/// ```rust
///
/// use spotify_assistant_core::enums::fs::ProjectFiles;
///
/// fn handle_file(file: ProjectFiles) {
///     match file {
///         ProjectFiles::DotEnv => println!("Processing .env file..."),
///         ProjectFiles::TokenCache => println!("Processing token cache file..."),
///     }
/// }
/// ```
pub enum ProjectFiles {
    DotEnv,
    TokenCache,
}

impl ProjectFiles {
    /// Returns the file path associated with a `ProjectFiles` variant.
    ///
    /// This method resolves the full file path based on the given `ProjectFiles` variant using
    /// directories provided by the `ProjectDirectories` module. It appends the appropriate
    /// file or directory name to the base path (config or cache) depending on the variant.
    ///
    /// # Variants:
    /// - `ProjectFiles::DotEnv`: Constructs the path to the `.env` file in the configuration directory.
    /// - `ProjectFiles::TokenCache`: Constructs the path to the `token_cache` file in the cache directory.
    ///
    /// # Returns:
    /// A `PathBuf` containing the resolved file path.
    ///
    /// # Examples:
    /// ```rust
    /// use std::path::PathBuf;
    /// use spotify_assistant_core::enums::fs::{ProjectDirectories, ProjectFiles};
    ///
    /// let project_file = ProjectFiles::DotEnv;
    /// let path = project_file.path();
    /// assert_eq!(path, ProjectDirectories::Config.path().join(".env"));
    /// ```
    ///
    /// # Note:
    /// Ensure that the necessary directories exist and have the required permissions, as this method only constructs the path
    /// and does not create or validate the existence of the actual file or directory.
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
