use std::collections::HashSet;
use std::io::Error;
use std::path::{Path, PathBuf};

use tracing::{debug, event, info, span, Level};

use crate::enums::fs::ProjectDirectories;

/// A struct that represents the file system layout for a project, encapsulating multiple key directory paths
/// relevant to the application. The `ProjectFileSystem` struct provides a well-organized way to manage system-specific
/// or user-specific directories utilized by applications. Each directory is represented by a `ProjectDirectories` instance.
///
/// # Fields
///
/// * `home_directory` - Represents the project's home directory, typically used as the base directory for the application.
///
/// * `config_directory` - Represents the directory used for storing configuration files related to the application.
///   This is generally intended for storing settings or other configuration data.
///
/// * `data_directory` - Represents the directory where application data files are stored.
///   This can include persistent storage of information such as databases, or other data needed for the application.
///
/// * `log_directory` - Represents the directory where log files related to the application are stored.
///   Often used for application debugging, tracking events, or error reporting.
///
/// * `state_directory` - Represents the directory used for storing state information required by the application.
///   This is typically temporary data that the application may need to function during its runtime.
///
/// * `cache_directory` - Represents the directory used for storing cached data. Cached data can improve performance
///   by reducing redundant computations or repeated requests.
///
/// # Derives
///
/// * `Debug` - Allows projects of this struct to be formatted using the `{:?}` syntax for debugging purposes.
///
/// * `Default` - Provides a default implementation for initializing this struct with default values for each field.
///
/// Use this struct to abstract away the handling of major directory paths in a unified and maintainable way.
#[derive(Debug, Default)]
pub struct ProjectFileSystem {
    pub home_directory: ProjectDirectories,
    pub config_directory: ProjectDirectories,
    pub data_directory: ProjectDirectories,
    log_directory: ProjectDirectories,
    state_directory: ProjectDirectories,
    cache_directory: ProjectDirectories,
}

impl ProjectFileSystem {
    /// Creates a new instance of the `ProjectFileSystem`.
    ///
    /// This function initializes the `ProjectFileSystem` struct by setting up its directories,
    /// leveraging `ProjectDirectories` enums for standard paths such as home, config, data,
    /// log, state, and cache directories. It logs the creation and path settings using
    /// structured logging with tracing.
    ///
    /// # Returns
    ///
    /// A new instance of the `ProjectFileSystem` struct with all directories initialized.
    ///
    /// # Logging
    ///
    /// - Logs an informational message indicating the start of the initializer process.
    /// - Logs the home directory path of the initialized struct for debugging purposes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use spotify_assistant_core::utilities::filesystem::initialization::ProjectFileSystem;
    /// let project_file_system = ProjectFileSystem::new();
    /// ```
    ///
    /// # External Dependencies
    ///
    /// - This function uses the `span!` macro from the `tracing` crate to create a span
    ///   with an `INFO` level for tracing the initialization process.
    /// - Logs are emitted using the `info!` macro from the `tracing` crate.
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

    /// Retrieves all files within a specified directory (and its subdirectories)
    /// that match a given filter string. The function traverses the directory
    /// structure recursively and uses the filter to determine which files to include
    /// in the result.
    ///
    /// # Parameters
    /// - `&self`: A reference to the current instance of the struct implementing this method.
    /// - `dir`: A `PathBuf` representing the root directory to search in.
    /// - `filter`: A string slice used to match files by their names. Files whose names
    ///   contain the filter string will be included in the result set.
    ///
    /// # Returns
    /// A `HashSet<PathBuf>` containing the file paths that match the given filter string.
    ///
    /// # Behavior
    /// - For each file or directory in the specified directory:
    ///     - If it is a directory, the function calls itself recursively to process
    ///       its contents.
    ///     - If it is a file, the function checks whether the file name contains the specified
    ///       filter string. Matching files are added to the result set.
    /// - If an error occurs while reading the directory, an error message is logged using
    ///   the `tracing` crate, and an empty `HashSet` is returned.
    /// - Informational logs are emitted for debugging purposes, including matched files,
    ///   processed directories, and unprocessed files.
    ///
    /// # Logging
    /// This function is instrumented with the `tracing` crate to log at the following levels:
    /// - `INFO`: Indicates when the function is entered and logs directory processing details.
    /// - `ERROR`: Logs errors if the directory cannot be read.
    ///
    /// # Panics
    /// - The function will panic if a directory entry cannot be unwrapped (e.g., due to
    ///   invalid UTF-8 or filesystem issues). This could occur when calling `entry.unwrap()`.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashSet;
    /// use std::path::PathBuf;
    ///
    /// use spotify_assistant_core::utilities::filesystem::initialization::ProjectFileSystem;
    /// let project_file_system = ProjectFileSystem::new();
    /// let dir = PathBuf::from("/path/to/search");
    /// let filter = "example";
    /// let matched_files = project_file_system.get_files(dir, filter);
    ///
    /// for file in &matched_files {
    ///     println!("Matched file: {:?}", file);
    /// }
    /// ```
    ///
    /// In this example, the function searches `/path/to/search` for files containing `"example"`
    /// in their names and prints the matched file paths.
    #[allow(clippy::only_used_in_recursion)]
    pub fn get_files(&self, dir: PathBuf, filter: &str) -> HashSet<PathBuf> {
        let span = span!(Level::INFO, "ProjectFileSystem.get_files");
        let _enter = span.enter();
        let reader = match dir.read_dir() {
            Ok(read_dir) => { read_dir }
            Err(error) => {
                event!(Level::ERROR, "Error reading directory: {:?}", error);
                return HashSet::new();
            }
        };
        let mut return_vec = HashSet::new();
        reader.for_each(|entry| {
            let path = entry.unwrap().path();
            let is_dir = path.is_dir();
            if is_dir {
                println!("Checking the {:?} directory", path.file_name().unwrap());
                let matched_files = self.get_files(path, filter);
                for file in matched_files {
                    println!("Matched file: {:?}", file.file_name().unwrap());
                    return_vec.insert(file);
                }
            } else if path.file_name().unwrap().to_str().unwrap().contains(filter) {
                println!("File match: {:?}", path.file_name().unwrap());
                return_vec.insert(path);
            } else {
                println!("Other file: {:?}", path.file_name().unwrap());
            }
        });
        println!("{return_vec:?}");
        return_vec
    }

    /// Initializes necessary directories and files for the application.
    ///
    /// This method performs the following operations:
    /// - Creates a list of directories and files that are required for the application.
    /// - Iterates over the list of directories and attempts to initialize each one by
    ///   calling the `initialize_directory` method.
    /// - Logs a debug message indicating whether each directory was successfully created
    ///   or if there was an error during its creation.
    /// - Iterates over the list of files and attempts to initialize each one by calling
    ///   the `initialize_file` method.
    /// - Logs a debug message indicating whether each file was successfully created or
    ///   if there was an error during its creation.
    ///
    /// ## Directories
    /// The following directories are initialized:
    /// - `home_directory`
    /// - `config_directory`
    /// - `data_directory`
    /// - `log_directory`
    /// - `state_directory`
    /// - `cache_directory`
    ///
    /// ## Files
    /// The following files are initialized:
    /// - `blacklist.toml` (located in the `config_directory`)
    ///
    /// ## Logging
    /// A tracing span (`INFO` level) is created for the initializer under the name
    /// `Initializer.init`. Additional debug-level messages are logged to provide
    /// detailed reports for success or failure of each operation.
    ///
    /// ## Error Handling
    /// - If a directory or file cannot be created, this is logged as a debug message
    ///   with details about the error. However, the method continues execution for
    ///   the remaining items in the list.
    ///
    /// ## Notes
    /// - The `initialize_directory` and `initialize_file` methods are expected to handle
    ///   the actual creation process for directories and files, and any errors they
    ///   produce are caught and logged in this method.
    pub fn init(&self) {
        let dir_vec = vec![
            self.home_directory.path(),
            self.config_directory.path(),
            self.data_directory.path(),
            self.log_directory.path(),
            self.state_directory.path(),
            self.cache_directory.path(),
        ];
        let file_vec = vec![
            self.config_directory.path().join("blacklist.toml"),
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
        };
        for file in file_vec {
            match self.initialize_file(&file.clone()) {
                Ok(_) => {
                    debug!(
                        "File {:?} was successfully created.",
                        file.clone().to_str().unwrap()
                    );
                }
                Err(e) => {
                    debug!(
                        "File {:?} was not created. Error: {:?}",
                        file.clone().to_str().unwrap(),
                        e
                    );
                }
            };
        };
    }

    /// Initializes a file at the specified file path.
    ///
    /// This function attempts to create a file at the provided `file_path`. If the file does not
    /// already exist, it attempts to create it using the `create_file` method. If the file
    /// already exists, the function logs an informational message and skips file creation.
    ///
    /// # Parameters
    /// - `file_path`: A reference to a `PathBuf` that specifies the location of the file to initialize.
    ///
    /// # Returns
    /// - `Ok(true)`: If the file was successfully created.
    /// - `Ok(false)`: If the file already exists and no creation occurred.
    /// - `Err(Error)`: If an error occurred during the file creation process.
    ///
    /// # Logging
    /// - Emits an INFO-level tracing event indicating the attempt to initialize the file.
    /// - Logs whether the file creation is attempted or skipped.
    ///
    /// # Errors
    /// Returns an error if the `create_file` method fails to create the file.
    ///
    /// # Example
    /// ```
    /// use std::path::PathBuf;
    /// use spotify_assistant_core::utilities::filesystem::initialization::ProjectFileSystem;
    /// let initializer = ProjectFileSystem::new();
    /// let file_path = PathBuf::from("example.txt");
    /// match initializer.initialize_file(&file_path) {
    ///     Ok(true) => println!("File created successfully."),
    ///     Ok(false) => println!("File already exists."),
    ///     Err(e) => eprintln!("Failed to create the file: {:?}", e),
    /// }
    /// ```
    pub fn initialize_file(&self, file_path: &Path) -> Result<bool, Error> {
        let span = span!(
            Level::INFO,
            "Initializer.initialize_file",
            value = file_path.to_str().unwrap()
        );
        let _enter = span.enter();
        event!(
            Level::INFO,
            "Attempting to create the following file: {:?}",
            file_path.to_str().unwrap()
        );
        if !file_path.exists() {
            match self.create_file(PathBuf::from(file_path)) {
                Ok(_) => Ok(true),
                Err(e) => Err(e)
            }
        } else {
            event!(
                Level::INFO,
                "Skipping file creation because it already exists."
            );
            Ok(false)
        }
    }

    /// Initializes a directory at the specified path.
    ///
    /// If the directory does not exist, it attempts to create it. If the directory already exists, no
    /// action is taken, and the function returns `Ok(false)`.
    ///
    /// # Arguments
    ///
    /// * `directory_path` - A reference to a `PathBuf` which represents the path of the directory to be initialized.
    ///
    /// # Returns
    ///
    /// This method returns a `Result<bool, Error>`:
    /// - `Ok(true)` if the directory was successfully created.
    /// - `Ok(false)` if the directory already exists, and no creation was necessary.
    /// - `Err(Error)` if an error occurred while attempting to create the directory.
    ///
    /// # Logging
    ///
    /// This function makes use of structured logging:
    /// - Logs an `INFO` level span named `"Initializer.initialize_directory"` with the target directory path.
    /// - Logs an `INFO` level event when attempting to create a directory.
    /// - Logs an `INFO` level event if the directory already exists and the creation is skipped.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// - If the directory does not exist and there is a problem with creating it, such as insufficient permissions,
    ///   invalid path, or filesystem-related issues.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use spotify_assistant_core::utilities::filesystem::initialization::ProjectFileSystem;
    ///
    /// let initializer = ProjectFileSystem::new();
    /// let directory_path = PathBuf::from("/path/to/directory");
    ///
    /// match initializer.initialize_directory(&directory_path) {
    ///     Ok(true) => println!("Directory created successfully."),
    ///     Ok(false) => println!("Directory already exists. No action taken."),
    ///     Err(e) => eprintln!("Failed to initialize directory: {}", e),
    /// }
    /// ```
    pub fn initialize_directory(&self, directory_path: &Path) -> Result<bool, Error> {
        let span = span!(
            Level::INFO,
            "Initializer.initialize_directory",
            value = directory_path.to_str().unwrap()
        );
        let _enter = span.enter();
        event!(
            Level::INFO,
            "Attempting to create the following directory: {:?}",
            directory_path.to_str().unwrap()
        );
        if !directory_path.exists() {
            match self.create_directory(PathBuf::from(directory_path)) {
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

    /// Creates a new file at the specified file path.
    ///
    /// This function attempts to create a file at the given `file_path`.
    /// It utilizes structured logging to log events, such as the success or failure
    /// of the file creation process, at different levels of severity. Specifically:
    /// - Logs an `INFO` level message if the file is successfully created.
    /// - Logs a `DEBUG` level message if there is an error creating the file.
    ///
    /// # Arguments
    /// - `file_path`: A `PathBuf` representing the path of the file to be created.
    ///
    /// # Returns
    /// - `Ok(())` if the file is successfully created.
    /// - `Err(Error)` if there is an error during the file creation process.
    ///
    /// # Logging
    /// - Uses the `tracing` crate for structured logging.
    /// - A span named `Initializer.create_file` is created for the duration of the function,
    ///   with a logged value of the file path as a string.
    /// - Logs an informational message when the file is successfully created.
    /// - Logs a debug message when there is a failure.
    ///
    /// # Errors
    /// - Returns an `Err(Error)` if the file system fails to create the file, such as
    ///   due to permission issues, invalid paths, or insufficient disk space.
    fn create_file(&self, file_path: PathBuf) -> Result<(), Error> {
        let span = span!(
            Level::INFO,
            "Initializer.create_file",
            value = file_path.clone().to_str().unwrap()
        );
        let _enter = span.enter();
        match std::fs::File::create(file_path.clone()) {
            Ok(_) => {
                event!(
                    Level::INFO,
                    "{:?} was successfully created.",
                    file_path.clone().to_str().unwrap()
                );
                Ok(())
            }
            Err(e) => {
                event!(
                    Level::DEBUG,
                    "Unable to create the following file: {:?}",
                    file_path.clone().to_str().unwrap()
                );
                Err(e)
            }
        }
    }

    /// Attempts to create a new directory at the specified path.
    ///
    /// This function creates a new directory at the provided `directory_path` and logs the process
    /// using tracing spans and events for better observability. If the operation is successful,
    /// it logs an informational message. Otherwise, it logs a debug message and returns an error.
    ///
    /// # Parameters
    /// - `directory_path` (`PathBuf`): The path where the directory should be created.
    ///
    /// # Returns
    /// - `Ok(())` if the directory was successfully created.
    /// - `Err(Error)` if there was an error while attempting to create the directory.
    ///
    /// # Logging
    /// - Logs an informational message when a directory is successfully created.
    /// - Logs a debug message if the creation fails, including the error context.
    ///
    /// # Errors
    /// Returns an `Error` when the directory creation fails, including cases like:
    /// - The directory already exists.
    /// - Insufficient permissions to create the directory.
    /// - Invalid or inaccessible path.
    ///
    /// # Note
    /// The function uses tracing's `span!` and `event!` macro for structured logging and telemetry.
    /// Ensure that tracing instrumentation is initialized in your application for effective logging.
    fn create_directory(&self, directory_path: PathBuf) -> Result<(), Error> {
        let span = span!(
            Level::INFO,
            "Initializer.create_directory",
            value = directory_path.clone().to_str().unwrap() //tarpaulin ignore
        );
        let _enter = span.enter();
        match std::fs::create_dir(directory_path.clone()) {
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
        println!("{init:?}");
        assert_eq!(init.home_directory.path(), ProjectDirectories::Home.path());
    }

    #[test]
    fn test_get_files() {
        let init = ProjectFileSystem::default();
        // init.show_items(ProjectDirectories::Data.path(), 0);
        let mut x = init.get_files(ProjectDirectories::Data.path(), "Audio");
        x.retain(|path| path.extension().unwrap() == "json");
        x.clone().into_iter().enumerate().for_each(|(index, path)| {
            println!("{index}: {path:?}");
        });
        assert_eq!(x.len(), 8);
    }

    #[test]
    fn test_new() {
        let init = ProjectFileSystem::new();
        assert_eq!(init.home_directory.path(), ProjectDirectories::Home.path());
    }
    #[test]
    fn test_create_file() {
        let init = ProjectFileSystem::default();
        let file = init.home_directory.path().join("test_file");
        match init.create_file(file.clone()) {
            Ok(_) => {
                assert!(file.exists());
            }
            Err(e) => {
                assert_eq!(e.kind(), ErrorKind::AlreadyExists);
            }
        };
        fs::remove_file(file).unwrap();
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
