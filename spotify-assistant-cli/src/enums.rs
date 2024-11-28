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
enum HashMapArgTypes {
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
                    // HashMapArgTypes::Bool(Some(val.get_one::<bool>(id.as_str()).clone()))
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
                // if let Some(val) = val.get_one::<String>(id.as_str()) {
                //     HashMapArgTypes::String(val.to_string())
                // } else if let Some(val) = val.get_one::<bool>(id.as_str()) {
                //     HashMapArgTypes::Bool(val.clone())
                // } else {
                //     HashMapArgTypes::Bool(false)
                // }
            }
        }
    }
}
