/// Generates auto-complete scripts for different shell types.
///
/// This macro takes a shell type and a command, and generates the appropriate
/// auto-complete script for the specified shell.
///
/// # Arguments
/// * `$shell` - The shell type for which to generate the auto-complete script.
/// * `$cmd` - The command for which to generate the auto-complete script.
macro_rules! generate_auto_complete {
    ($shell:expr, $cmd:expr) => {
        match $shell {
            ShellType::Bash => generate(Shell::Bash, &mut $cmd, "spotcli", &mut std::io::stdout()),
            ShellType::Zsh => generate(Shell::Zsh, &mut $cmd, "spotcli", &mut std::io::stdout()),
            ShellType::Fish => generate(Shell::Fish, &mut $cmd, "spotcli", &mut std::io::stdout()),
            ShellType::PowerShell => generate(
                Shell::PowerShell,
                &mut $cmd,
                "spotcli",
                &mut std::io::stdout(),
            ),
            ShellType::Elvish => {
                generate(Shell::Elvish, &mut $cmd, "spotcli", &mut std::io::stdout())
            }
        }
    };
}
pub(crate) use generate_auto_complete;

/// Handles the presence of a specific argument in the matches.
///
/// This macro checks if a specific argument is present in the matches and returns
/// a corresponding `HashMapArgTypes` variant.
///
/// # Arguments
/// * `$matches` - The argument matches to check.
/// * `$key` - The key of the argument to check.
/// * `$type` - The type of the argument.
/// * `$variant` - The variant of `HashMapArgTypes` to return if the argument is present.
#[allow(unused_macros)]
macro_rules! handle_rr {
    ($matches:expr, $key:expr, $type:ty, $variant:ident) => {{
        let presence = match $matches.get_one::<$type>($key) {
            Some(val) => HashMapArgTypes::$variant(val.to_owned()),
            None => HashMapArgTypes::Bool(false),
        };
        presence
    }};
}
