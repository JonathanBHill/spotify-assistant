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

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::test_support::{TestEnvironment, ENV_MUTEX};


    #[test]
    fn project_directories_resolve_within_temporary_environment() {
        let _guard = ENV_MUTEX.lock().unwrap_or_else(|err| err.into_inner());
        let env = unsafe { TestEnvironment::new() };

        assert_eq!(ProjectDirectories::Home.path(), env.home_dir());
        assert_eq!(ProjectDirectories::Config.path(), env.config_dir());
        assert_eq!(ProjectDirectories::Data.path(), env.data_dir());
        assert_eq!(ProjectDirectories::Cache.path(), env.cache_dir());
        assert_eq!(ProjectDirectories::Log.path(), env.data_dir());
        assert_eq!(ProjectDirectories::State.path(), env.state_dir());
        let preferences_path = ProjectDirectories::Preferences.path();
        assert!(
            preferences_path == env.preferences_dir() || preferences_path == env.config_dir(),
            "preferences directory should resolve to either the explicit preference path or configuration path"
        );
        assert_eq!(ProjectDirectories::Template.path(), env.template_dir());
    }

    #[test]
    fn project_directories_default_is_home() {
        assert_eq!(ProjectDirectories::default(), ProjectDirectories::Home);
    }

    #[test]
    fn project_files_use_expected_roots() {
        let _guard = ENV_MUTEX.lock().unwrap_or_else(|err| err.into_inner());
        let env = unsafe { TestEnvironment::new() };

        let dot_env = ProjectFiles::DotEnv.path();
        let token_cache = ProjectFiles::TokenCache.path();

        assert_eq!(dot_env, env.config_dir().join(".env"));
        assert_eq!(token_cache, env.cache_file("token_cache"));
    }
}
