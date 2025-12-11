use rspotify::model::{Id, Market};
use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodeSpotify, Config, Credentials, OAuth};
use std::collections::HashSet;
use std::env;
use tracing::{Level, error, event, span, trace, trace_span};

use crate::enums::fs::{ProjectDirectories, ProjectFiles};

/// The `Api` trait provides an interface for setting up and interacting with an API client.
/// It includes methods for selecting scopes, configuring the client, defining a market,
/// and removing duplicate items from a vector.
///
/// # Required Imports
/// Ensure that your project includes the following dependencies:
/// - `dotenv`
/// - `rspotify`
/// - `tracing`
///
/// # Methods
///
/// ## `fn select_scopes() -> HashSet<String>`
///
/// This method should be implemented to define and return the set of authorization scopes
/// required for the API.
///
/// # Returns
/// - A `HashSet` containing strings that represent the required scopes.
///
/// ## `fn set_up_client(is_test: bool, scopes: Option<HashSet<String>>) -> impl Future<Output = AuthCodeSpotify> + Send`
///
/// Asynchronously sets up the API client using given credentials and configuration. The setup
/// involves loading credentials from a `.env` file located in a configuration directory,
/// establishing an OAuth flow, and initializing the `AuthCodeSpotify` client.
///
/// ### Parameters
/// - `is_test`:
///     - `true` if setting up the client for test purposes without triggering the full OAuth flow.
///     - `false` to complete the OAuth flow interactively.
/// - `scopes`: An optional set of authorization scopes for the client.
///
/// ### Behavior
/// - Loads environment variables from a `.env` file.
/// - Searches for `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET` in the `.env` file to configure credentials.
/// - If credentials are not found, logs appropriate errors and halts execution with a `panic!`.
/// - Configures token caching and refreshing using `Config`.
/// - If `is_test` is `true`, the OAuth flow is skipped, and an unauthenticated client is returned.
/// - Otherwise, it initiates the OAuth authorization flow to obtain an access token interactively.
///
/// ### Returns
/// - An asynchronous future that resolves to an `AuthCodeSpotify` object, representing the configured client.
///
/// ### Errors
/// - Logs detailed error messages if credentials are missing or improperly set in the `.env` file.
/// - Panics if credentials cannot be loaded.
///
/// ### Logging
/// - Extensive tracing logs are emitted throughout the method to trace setup progress and capture errors.
///
pub trait Api {
    /// Selects and returns a set of authorization scopes.
    ///
    /// This function is designed to retrieve and collect a set of unique scopes,
    /// which are typically used for accessing resources or defining permissions
    /// in APIs and authentication processes.
    ///
    /// # Returns
    ///
    /// A `HashSet<String>` containing the selected authorization scopes. Each
    /// scope in the set is a unique string representing a specific permission
    /// or resource access level.
    ///
    /// # Notes
    ///
    /// - The exact method or criteria used to select scopes is determined
    ///   within the implementation of this function.
    /// - This function guarantees uniqueness of scopes, since the `HashSet`
    ///   data structure automatically eliminates duplicates.
    ///
    /// # Panics
    ///
    /// This function does not explicitly document any conditions under
    /// which it might panic, but this depends on its specific implementation.
    ///
    /// # Errors
    ///
    /// If the implementation involves operations that may fail (e.g., reading
    /// from external sources or parsing), those errors should be transparently
    /// handled or documented.
    fn select_scopes() -> HashSet<String>;

    ///
    /// Sets up a Spotify client for authentication and API use.
    ///
    /// # Arguments
    /// - `is_test` - A boolean flag that determines whether the client is being set up for testing purposes.
    ///   If `true`, it skips the token prompt and directly initializes the client.
    /// - `scopes` - An optional set of authentication scopes required by the application.
    ///   If none are provided, default values will be set.
    ///
    /// # Returns
    /// An `async` future that resolves to an `AuthCodeSpotify` instance for API interaction.
    ///
    /// # Behavior
    /// 1. Loads the `.env` configuration file to retrieve Spotify credentials (`RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET`).
    /// 2. Parses environment variables for the required credentials.
    /// 3. Warns the user and raises an error if credentials are not found:
    ///     - Notifies if the `.env` file is missing.
    ///     - Indicates missing client ID or secret, providing suggestions for resolution.
    /// 4. Applies configuration settings, such as enabling token caching and refreshing.
    /// 5. If `is_test` is `true`, directly returns the Spotify client without token authorization.
    /// 6. Otherwise, prompts the user to authorize the application using a URL.
    /// 7. Initializes and returns the Spotify client after successful authentication.
    ///
    /// # Errors
    /// - The function will panic if:
    ///   - The `.env` file cannot be found and loaded.
    ///   - Required credentials (`RSPOTIFY_CLIENT_ID` and/or `RSPOTIFY_CLIENT_SECRET`) are not available.
    ///
    /// # Notes
    /// - The `.env` file should be located in the directory specified by the configuration path.
    /// - Credentials need to be placed in the `.env` file:
    ///
    ///   - RSPOTIFY_CLIENT_ID=<your-client-id>
    ///   - RSPOTIFY_CLIENT_SECRET=<your-client-secret>
    ///
    /// - To retrieve Spotify credentials, refer to Spotify's [Developer Dashboard](https://developer.spotify.com/dashboard/).
    ///
    /// # Scope
    /// - If no explicit scopes are provided, the client will use an empty default.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// Setting up a Spotify client:
    /// use std::collections::HashSet;
    /// use spotify_assistant_core::traits::apis::Api;
    ///
    /// let is_test = false;
    /// let scopes = Some(HashSet::from(["user-read-private".to_string(), "playlist-read-private".to_string()]));
    /// let spotify_client = Api::set_up_client(is_test, scopes);
    /// // Use the `spotify_client` for further API interactions
    /// ```
    fn set_up_client(
        is_test: bool,
        scopes: Option<HashSet<String>>,
    ) -> impl Future<Output = AuthCodeSpotify> + Send {
        async move {
            let suc_span = trace_span!("api-client");
            let _enter = suc_span.enter();
            dotenv::from_path(ProjectFiles::DotEnv.path()).ok();
            trace!(
                target: "api-setup",
                parent: suc_span.clone(),
                "{} .env file.",
                if dotenv::from_path(ProjectFiles::DotEnv.path()).is_ok() {
                    "Successfully loaded"
                } else {
                    "Failed to load"
                }
            );
            let credentials = match Credentials::from_env() {
                Some(creds) => {
                    event!(
                        target: "api_setup",
                        parent: suc_span.clone(),
                        Level::INFO,
                        "ID and Secret credentials were successfully obtained from .env file"
                    );
                    creds
                }
                None => {
                    let env_file = ProjectFiles::DotEnv.path();
                    if !env_file.exists() {
                        let path = ProjectDirectories::Config.path().clone();
                        error!(
                            name: "credentials",
                            target: "api-setup",
                            parent: suc_span.clone(),
                            env_directory =  ProjectDirectories::Config.path().to_str().unwrap(),
                            ".env file was not found on the system. This file should be created in your configuration directory {:?}",
                            // fixme <On message line above> Add terminology that aligns with the user's operating system (e.g. directory vs folder)
                            { path.to_str().unwrap() }
                        );
                    } else {
                        let _ = env::args().filter(|key| {
                            if key.contains("RSPOTIFY_CLIENT_ID") {
                                error!(
                                    name: "client-setup.credentials",
                                    target: "client-setup",
                                    "Client secret was not found in .env file."
                                    // fixme <On message line above> Add resource to tell user how to obtain client secret and where to store it
                                );
                                false
                            } else if key.contains("RSPOTIFY_CLIENT_SECRET") {
                                error!(
                                    name: "client-setup.credentials",
                                    target: "client-setup",
                                    "Client ID was not found in .env file."
                                    // fixme <On message line above> Add resource to tell user how to obtain client ID and where to store it
                                );
                                false
                            } else {
                                false
                            }
                        });
                        error!(
                            name: "client-setup.credentials",
                            target: "client-setup",
                            "Credentials not found in .env file.",
                        );
                    }
                    panic!("Credentials not found.")
                }
            };

            let config = Config {
                cache_path: ProjectDirectories::Cache.path().join("token_cache"),
                token_cached: true,
                token_refreshing: true,
                ..Default::default()
            };

            let oath = OAuth::from_env(scopes.unwrap_or_default()).unwrap_or_default();
            let spotify_client =
                AuthCodeSpotify::with_config(credentials.clone(), oath.clone(), config.clone());
            if is_test {
                return spotify_client;
            }
            let url = spotify_client.get_authorize_url(false).unwrap();
            spotify_client.prompt_for_token(&url).await.unwrap();
            event!(Level::TRACE, "Client was initialized");
            spotify_client
        }
    }

    /// Returns a `Market` object representing the United States.
    ///
    /// This function creates a `Market` instance specifically set to the
    /// United States by utilizing the `rspotify::model::Country::UnitedStates`
    /// variant. This can be useful for restricting or localizing API requests
    /// or data to a specific geographic region.
    ///
    /// # Returns
    /// * `Market` - A market object set to the United States.
    fn market() -> Market {
        Market::Country(rspotify::model::Country::UnitedStates)
    }

    /// Removes duplicate elements from a vector based on their unique identifiers and returns a cleaned vector.
    ///
    /// This function works with any type `T` that implements the `Clone`, `Eq`, `Id`, and `std::hash::Hash` traits.
    /// It guarantees that each element in the returned vector is unique, as determined by its ID.
    ///
    /// # Type Parameters
    /// - `T`: The type of elements in the vector. Must implement the traits `Clone`, `Eq`, `Id`, and `std::hash::Hash`.
    ///
    /// # Parameters
    /// - `data`: A vector containing elements of type `T`. It may include duplicates.
    ///
    /// # Returns
    /// A new vector of type `T` containing only unique elements from the input vector,
    /// preserving their order of first occurrence in the original input.
    ///
    /// # Note
    /// The function uses a `HashSet` to track already-seen elements,
    /// relying on the `hash` and `eq` implementations of the type to identify duplicates.
    fn clean_duplicate_id_vector<T: Clone + Eq + Id + std::hash::Hash>(data: Vec<T>) -> Vec<T> {
        let mut cleaned_vec = Vec::new();
        let mut seen = HashSet::new();
        data.into_iter().for_each(|item| {
            if seen.insert(item.clone()) {
                cleaned_vec.push(item);
            }
        });
        cleaned_vec
    }
}

/// A trait for creating asynchronous instances of a type through a querying mechanism.
///
/// This trait defines a function for initializing an instance of a type asynchronously.
/// It can be implemented for use cases where initialization might require asynchronous
/// operations, such as querying a database, making network requests, or performing
/// other I/O-bound tasks.
///
/// # Associated Types
/// - `Self`: The concrete type implementing the `Querying` trait.
///
/// # Required Methods
///
/// ## `new`
///
/// Creates and returns an instance of the implementing type asynchronously.
///
/// ### Returns
/// - An instance of `impl std::future::Future<Output = Self> + Send`, where `Self` is
///   the type implementing this trait.
/// - The returned future should be `Send`, meaning it can safely be sent across threads.
///
/// # Example
/// ```
/// use spotify_assistant_core::traits::apis::Querying;
/// use std::future::Future;
///
/// pub struct MyStruct {
///     pub data: String,
/// }
///
/// impl Querying for MyStruct {
///     fn new() -> impl Future<Output = MyStruct> + Send {
///         async {
///             MyStruct { data: "Initialized".to_string() }
///         }
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let instance = MyStruct::new().await;
///     println!("{}", instance.data); // Output: "Initialized"
/// }
/// ```
pub trait Querying {
    fn new() -> impl Future<Output = Self> + Send;
}
