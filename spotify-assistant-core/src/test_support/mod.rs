use crate::test_support::test_ws::ROOT;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tempfile::TempDir;

// src/test_support.rs
// #[cfg(test)]
pub mod test_ws {
    pub use once_cell::sync::Lazy;
    use std::fs;
    use std::path::PathBuf;

    fn target_dir() -> PathBuf {
        if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
            dir.into()
        } else {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target")
        }
    }
    fn create_fileset(root: &PathBuf, files: &[&str]) {
        for file in files {
            let path = root.join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            if !path.exists() {
                fs::write(path, b"").unwrap();
            }
        }
    }

    pub static ROOT: Lazy<PathBuf> = Lazy::new(|| {
        let root = target_dir().join(format!("test-workspace-{}", env!("CARGO_PKG_VERSION")));
        let files = [
            "home/.config/spotify-assistant/.env",
            "home/.config/spotify-assistant/config.toml",
            "home/.config/spotify-assistant/blacklist.toml",
            "home/.config/spotify-assistant/constants.toml",
            "home/.config/spotify-assistant/logs/spotify-assistant.log",
            "home/.config/spotify-assistant/logs/unit-tests.log",
            "home/.config/spotify-assistant/logs/integration-tests.log",
        ];

        fs::create_dir_all(&root).unwrap();
        create_fileset(&root, &files);
        root
    });
}


pub mod offline;

type EnvMap = HashMap<&'static str, Option<OsString>>;

/// Guards tests that manipulate global environment variables so they run serially.
pub static ENV_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

const ENV_VARS: [&str; 6] = [
    "HOME",
    "XDG_CONFIG_HOME",
    "XDG_DATA_HOME",
    "XDG_CACHE_HOME",
    "XDG_STATE_HOME",
    "XDG_PREFERENCES_HOME",
];

const APPLICATION_NAME: &str = "spotify-assistant";

/// Provides an isolated temporary directory hierarchy that mirrors the layout
/// used by [`directories::ProjectDirs`] for the application. Environment
/// variables are rewritten so calls to [`ProjectDirectories::path`] resolve to
/// these temporary locations.
///
/// Environment variables are restored when the struct is dropped.
pub struct TestEnvironment {
    _temp_dir: TempDir,
    home: PathBuf,
    project_config: PathBuf,
    project_data: PathBuf,
    project_cache: PathBuf,
    project_state: PathBuf,
    project_preferences: PathBuf,
    env_backup: EnvMap,
}

impl TestEnvironment {
    /// Creates a new [`TestEnvironment`], updates the relevant environment
    /// variables, and ensures the expected directory structure exists.
    /// # Safety
    /// This function is unsafe
    pub unsafe fn new() -> Self {
        let temp_dir = TempDir::new().expect("failed to create temporary test directory");

        let home = ROOT.join("home");
        let config_base = ROOT.join("home/.config");
        let data_base = ROOT.join("home/.local/share");
        let cache_base = ROOT.join("home/.cache");
        let state_base = ROOT.join("home/.local/state");
        let preference_base = ROOT.join("home/.config");

        for path in [
            &home,
            &config_base,
            &data_base,
            &cache_base,
            &state_base,
            &preference_base,
        ] {
            fs::create_dir_all(path).expect("failed to create base directory");
        }

        let project_config = config_base.join(APPLICATION_NAME);
        let project_data = data_base.join(APPLICATION_NAME);
        let project_cache = cache_base.join(APPLICATION_NAME);
        let project_state = state_base.join(APPLICATION_NAME);
        let project_preferences = preference_base.join(APPLICATION_NAME);

        for project_dir in [
            &project_config,
            &project_data,
            &project_cache,
            &project_state,
            &project_preferences,
        ] {
            fs::create_dir_all(project_dir).expect("failed to create project directory");
        }

        fs::create_dir_all(home.join("Templates")).expect("failed to create template directory");

        let env_backup: EnvMap = ENV_VARS
            .iter()
            .map(|&var| (var, env::var_os(var)))
            .collect();

        unsafe {
            set_var("HOME", &home);
            set_var("XDG_CONFIG_HOME", &config_base);
            set_var("XDG_DATA_HOME", &data_base);
            set_var("XDG_CACHE_HOME", &cache_base);
            set_var("XDG_STATE_HOME", &state_base);
            set_var("XDG_PREFERENCES_HOME", &preference_base);
        }

        Self {
            _temp_dir: temp_dir,
            home,
            project_config,
            project_data,
            project_cache,
            project_state,
            project_preferences,
            env_backup,
        }
    }

    pub fn home_dir(&self) -> &Path {
        &self.home
    }

    pub fn config_dir(&self) -> &Path {
        &self.project_config
    }

    pub fn data_dir(&self) -> &Path {
        &self.project_data
    }

    pub fn cache_dir(&self) -> &Path {
        &self.project_cache
    }

    pub fn state_dir(&self) -> &Path {
        &self.project_state
    }

    pub fn preferences_dir(&self) -> &Path {
        &self.project_preferences
    }

    pub fn template_dir(&self) -> PathBuf {
        self.project_config.join("templates")
    }

    /// Returns a path within the project configuration directory.
    pub fn config_file(&self, name: &str) -> PathBuf {
        self.project_config.join(name)
    }

    /// Returns a path within the project cache directory.
    pub fn cache_file(&self, name: &str) -> PathBuf {
        self.project_cache.join(name)
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        for (key, value) in &self.env_backup {
            unsafe {
                match value {
                    Some(val) => env::set_var(key, val),
                    None => env::remove_var(key),
                }
            }
        }
    }
}

unsafe fn set_var(key: &str, path: &Path) {
    unsafe {
        env::set_var(key, path);
    }
}

/// Generates a TOML document representing a valid configuration file.
/// Paths are derived from the provided [`TestEnvironment`].
pub fn configuration_toml(env: &TestEnvironment) -> String {
    format!(
        r#"[general]

[behavior.duplicates]
custom_release_radar = true
query_playlist_for_blacklist = false

[cli]
default_shell = "bash"
artist_id_format = "uri"

[paths.files]
env = "{env_path}"
blacklist = "{blacklist_path}"
config = "{config_path}"
constants = "{constants_path}"

[paths.directories]
databases = "{databases}"
listening_history = "{listening_history}"
spotify_account_data = "{account_data}"
top_tracks = "{top_tracks}"

[preferences]
length_of_recently_played = 25
timeout = 15

[spotify]
default_user = "primary"

[spotify.content_ids]
stock_release_radar = "stock"
custom_release_radar = "custom"

[utility]
log_level = "info"
"#,
        env_path = env.config_dir().join(".env").display(),
        blacklist_path = env.config_dir().join("blacklist.toml").display(),
        config_path = env.config_dir().join("config.toml").display(),
        constants_path = env.config_dir().join("constants.toml").display(),
        top_tracks = env.data_dir().join("top_tracks").display(),
        databases = env.data_dir().join("databases").display(),
        listening_history = env.data_dir().join("history").display(),
        account_data = env.data_dir().join("account").display(),
    )
}

pub fn env(env: &TestEnvironment) -> String {
    r#"RELEASE_RADAR_ID="37i9dQZEVXbdINACbjb1qu"
MY_RELEASE_RADAR_ID="46mIugmIiN2HYVwAwlaBAr"
    "#.to_string()
}
pub fn constants_toml(env: &TestEnvironment) -> String {
    r#"title="constants"

        [[ids.playlists]]
    name="stock_release_radar"
    id="3WuaniG4xcoEXAH3ZBmbqX"
    "#.to_string()
}

/// Generates malformed configuration TOML suitable for failure tests.
pub fn invalid_configuration_toml() -> &'static str {
    "[general\ninvalid = true"
}

/// Generates a TOML document representing a valid blacklist file.
pub fn blacklist_toml() -> String {
    r#"[blacklist]

[[blacklist.artists]]
name = "Artist One"
id = "artist_one"

[[blacklist.artists]]
name = "Artist Two"
id = "artist_two"
"#
        .to_string()
}

/// Generates malformed blacklist TOML suitable for failure tests.
pub fn invalid_blacklist_toml() -> &'static str {
    "[blacklist\nartists = ["
}
