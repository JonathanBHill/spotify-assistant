use std::collections::HashSet;
use std::env;

use rspotify::{AuthCodeSpotify, Config, Credentials, OAuth};
use rspotify::model::{Id, Market};
use rspotify::prelude::OAuthClient;
use tracing::{error, event, Level, span, trace};

use crate::enums::fs::{ProjectDirectories, ProjectFiles};

pub trait Api {
    fn select_scopes() -> HashSet<String>;
    fn set_up_client(
        is_test: bool,
        scopes: Option<HashSet<String>>,
    ) -> impl std::future::Future<Output = AuthCodeSpotify> + Send {
        async move {
            let suc_span = span!(Level::TRACE, "Api.set_up_client");
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
                        error!(
                            name: "credentials",
                            target: "api-setup",
                            parent: suc_span.clone(),
                            env_directory =  ProjectDirectories::Config.path().to_str().unwrap(),
                            ".env file was not found on the system. This file should be created in your configuration directory {:?}",
                            // fixme <On message line above> Add terminology that aligns with the user's operating system (e.g. directory vs folder)
                            { ProjectDirectories::Config.path().to_str().unwrap() }
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
                    error!("Credentials not found.");
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
            spotify_client
        }
    }
    fn market() -> Market {
        Market::Country(rspotify::model::Country::UnitedStates)
    }
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

pub trait Querying {
    fn new() -> impl std::future::Future<Output = Self> + Send;
}
