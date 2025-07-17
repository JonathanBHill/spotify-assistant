use clap::{ArgMatches, ValueEnum};

/// Represents different shell types for auto-completion.
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord, ValueEnum)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl std::fmt::Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellType::Bash => write!(f, "bash"),
            ShellType::Zsh => write!(f, "zsh"),
            ShellType::Fish => write!(f, "fish"),
            ShellType::PowerShell => write!(f, "powershell"),
            ShellType::Elvish => write!(f, "elvish"),
        }
    }
}

/// Represents different types of argument values in a HashMap.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum HashMapArgTypes {
    String(String),
    Bool(bool),
}

impl HashMapArgTypes {
    /// Returns the type of the variant as a string
    #[allow(dead_code)]
    pub fn variant_type(&self) -> &'static str {
        match self {
            HashMapArgTypes::String(_) => "String",
            HashMapArgTypes::Bool(_) => "Bool",
        }
    }

    /// Creates a `HashMapArgTypes` variant from an `ArgMatches` option and an identifier.
    ///
    /// # Arguments
    /// * `arg_val` - An optional reference to `ArgMatches`.
    /// * `id` - A string identifier for the argument.
    ///
    /// # Returns
    /// A `HashMapArgTypes` variant.
    #[allow(dead_code)]
    fn from_gen(arg_val: Option<&ArgMatches>, id: String) -> Self {
        match arg_val {
            None => {
                println!("no arg matches; returning hsmbool");
                HashMapArgTypes::Bool(false)
            }
            Some(val) => {
                if id == "updaterr" {
                    if let Some(val) = val.get_one::<bool>(id.as_str()) {
                        println!("update true; returning hsmbool");
                        HashMapArgTypes::Bool(val.clone())
                        // t
                    } else {
                        println!("update false; returning hsmbool");
                        HashMapArgTypes::Bool(false)
                    }
                } else if id == "queryrr" {
                    if let Some(val) = val.get_one::<String>(id.as_str()) {
                        println!("list true; returning hsmstring");
                        HashMapArgTypes::String(val.to_string())
                    } else {
                        println!("list false; returning hsmbool");
                        HashMapArgTypes::Bool(false)
                    }
                } else if id == "rrcompare" {
                    if let Some(val) = val.get_one::<String>(id.as_str()) {
                        println!("compare true; returning hsmstring");
                        HashMapArgTypes::String(val.to_string())
                    } else {
                        println!("compare false; returning hsmbool");
                        HashMapArgTypes::Bool(false)
                    }
                } else {
                    println!("invalid flag; returning hsmbool");
                    HashMapArgTypes::Bool(false)
                }
            }
        }
    }
}

pub enum ReleaseRadarCmds {
    Update(ArgMatches),
    Compare(ArgMatches),
    Empty,
}

impl ReleaseRadarCmds {
    pub fn from_matches(matches: &ArgMatches) -> ReleaseRadarCmds {
        if let Some(update_arguments) = matches.subcommand_matches("update") {
            ReleaseRadarCmds::Update(update_arguments.to_owned())
        } else if let Some(compare_arguments) = matches.subcommand_matches("compare") {
            ReleaseRadarCmds::Compare(compare_arguments.to_owned())
        } else {
            ReleaseRadarCmds::Empty
        }
    }
}
pub enum QueryArgs {
    QStock(bool),
    QCustom(bool),
    QBlacklist(bool),
    QLibrary(bool),
    Empty,
}
impl QueryArgs {
    pub fn from_query_matches(matches: &ArgMatches) -> QueryArgs {
        let stock_rr_arg = Self::arg_exists("qstock", matches);
        let custom_rr_arg = Self::arg_exists("qcustom", matches);
        let blacklist_arg = Self::arg_exists("qblacklist", matches);
        let liked_songs_arg = Self::arg_exists("qlibrary", matches);
        if stock_rr_arg {
            QueryArgs::QStock(stock_rr_arg)
        } else if custom_rr_arg {
            QueryArgs::QCustom(custom_rr_arg)
        } else if blacklist_arg {
            QueryArgs::QBlacklist(blacklist_arg)
        } else if liked_songs_arg {
            QueryArgs::QLibrary(liked_songs_arg)
        } else {
            QueryArgs::Empty
        }
    }
    fn arg_exists(command: &str, matches: &ArgMatches) -> bool {
        match matches.get_one::<bool>(command) {
            Some(exists) => { *exists }
            None => { false }
        }
    }
}
#[derive(Debug)]
pub enum ReleaseRadarArgs {
    UPrint(bool),
    CPlaylists(String),
    Empty,
}
impl ReleaseRadarArgs {
    pub fn from_update_matches(matches: &ArgMatches) -> ReleaseRadarArgs {
        let uprint = Self::arg_exists("print", matches);
        if uprint {
            ReleaseRadarArgs::UPrint(uprint)
        } else {
            ReleaseRadarArgs::Empty
        }
    }
    fn arg_exists(command: &str, matches: &ArgMatches) -> bool {
        match matches.get_one::<bool>(command) {
            Some(exists) => { *exists }
            None => { false }
        }
    }
    pub fn from_compare_matches(matches: &ArgMatches) -> ReleaseRadarArgs {
        if let Some(let_playlists) = matches.get_one::<String>("playlisttocompare") {
            ReleaseRadarArgs::CPlaylists(let_playlists.to_string())
        } else {
            ReleaseRadarArgs::Empty
        }
    }
    pub fn from_matches(matches: &ArgMatches) -> ReleaseRadarArgs {
        let args = vec!["print", "playlisttocompare"];
        for arg in args {
            match arg {
                "print" => {
                    // ReleaseRadarArgs::from_update_matches(matches)
                    if ReleaseRadarArgs::bool_exists(arg, matches) {
                        println!("Printed");
                        return ReleaseRadarArgs::from_update_matches(matches);
                    } else {
                        println!("Does not exist")
                    }
                }
                "playlisttocompare" => {
                    if ReleaseRadarArgs::exists(false, arg, matches) {
                        return ReleaseRadarArgs::from_compare_matches(matches);
                    }
                }
                _ => {
                    return ReleaseRadarArgs::Empty;
                }
            }
        };
        ReleaseRadarArgs::Empty
    }
    fn bool_exists(command: &str, matches: &ArgMatches) -> bool {
        if let Some(_) = matches.get_one::<bool>(command) {
            true
        } else {
            false
        }
    }
    fn exists(is_bool: bool, command: &str, matches: &ArgMatches) -> bool {
        if is_bool {
            match matches.get_one::<bool>(command) {
                Some(_) => {
                    true
                }
                None => {
                    false
                }
            }
        } else {
            match matches.get_one::<String>(command) {
                Some(_) => {
                    true
                }
                None => {
                    false
                }
            }
        }
    }
}

pub enum ConfigArgs {
    Set(String, String),
    Get(String),
    Unset(String),
    Shell(ShellType),
    Blacklist(ArgMatches),
    Empty,
}
impl ConfigArgs {
    pub fn from_matches(matches: &ArgMatches) -> ConfigArgs {
        if let Some(set_value) = matches.get_one::<String>("cset") {
            ConfigArgs::Set("set".to_string(), set_value.to_string())
        } else if let Some(get_value) = matches.get_one::<String>("cget") {
            ConfigArgs::Get(get_value.to_string())
        } else if let Some(unset_value) = matches.get_one::<String>("cunset") {
            ConfigArgs::Unset(unset_value.to_string())
        } else if let Some(shell_value) = matches.get_one::<ShellType>("cshell") {
            ConfigArgs::Shell(shell_value.clone())
        } else if let Some(blacklist_argument) = matches.subcommand_matches("blacklist") {
            ConfigArgs::Blacklist(blacklist_argument.to_owned())
        } else {
            ConfigArgs::Empty
        }
    }
}
pub enum BlacklistArgs {
    Add(String),
    AddFromPlaylist(String),
    RemoveByName(String),
    RemoveBySelect,
    Empty,

}
impl BlacklistArgs {
    pub fn from_matches(matches: &ArgMatches) -> BlacklistArgs {
        if !matches.args_present() {
            BlacklistArgs::Empty
        } else if let Some(add_value) = matches.get_one::<String>("bladd") {
            BlacklistArgs::Add(add_value.to_string())
        } else if let Some(let_playlist) = matches.get_one::<String>("bladdfromplaylist") {
            BlacklistArgs::AddFromPlaylist(let_playlist.to_string())
        } else {
            match matches.get_one::<String>("blremove") {
                Some(remove_value) => {
                    BlacklistArgs::RemoveByName(remove_value.to_string())
                }
                None => {
                    BlacklistArgs::RemoveBySelect
                }
            }
        }
    }
}
