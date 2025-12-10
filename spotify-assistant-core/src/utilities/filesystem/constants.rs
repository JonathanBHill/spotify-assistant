use crate::enums::fs::ProjectDirectories;
use crate::traits::file_readers::ConfigReader;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Constants {
    ids: ConstIDs,
}
impl ConfigReader for Constants {
    fn file_path() -> PathBuf {
        let config_path = ProjectDirectories::Config.path();
        config_path.join("constants.toml")
    }

    fn new() -> Constants {
        match fs::read_to_string(Self::file_path()) {
            Ok(string) => toml::from_str(&string).unwrap_or_else(|err| {
                println!("{:?}", Self::file_path());
                panic!(
                    "Error deserializing toml string into a usable constants configuration: {err:?}"
                )
            }),
            Err(err) => {
                println!("{:?}", Self::file_path());
                panic!("Error reading the constants file: {err:?}")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConstIDs {
    playlists: HashSet<ConstPlaylist>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
struct ConstPlaylist {
    name: String,
    id: String,
}

impl ConstPlaylist {
    pub fn new(name: String, id: String) -> Self {
        Self { name, id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{TestEnvironment, constants_toml};

    fn constants_fixture(env: &TestEnvironment) -> Constants {
        let toml = constants_toml(env);
        toml::from_str(&toml).expect("fixture configuration should deserialize")
    }

    #[test]
    fn test_init() {
        let env = unsafe { TestEnvironment::new() };
        let constant = constants_fixture(&env);
        // let consts = Constants::new();
        println!("{:?}", constant);
    }
}
