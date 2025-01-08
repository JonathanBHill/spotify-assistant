use std::collections::HashMap;
use std::error::Error;

use clap::builder::{styling, BoolValueParser, BoolishValueParser, Styles, TypedValueParser};
use clap::{arg, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, ColorChoice, Command};
use clap_complete::{generate, Shell};
use futures::{stream, StreamExt};
use rspotify::model::idtypes::ArtistId;
use rspotify::model::{IdError, PlaylistId};
use rspotify::ClientError;
use tracing::{event, span, Level};

use spotify_assistant_core::actions::exploration::artist::ArtistXplorer;
use spotify_assistant_core::actions::exploration::playlist::PlaylistXplr;
use spotify_assistant_core::actions::playlists::user::UserPlaylists;
use spotify_assistant_core::actions::update::Updater;
use spotify_assistant_core::models::blacklist::{Blacklist, BlacklistArtist};

use crate::enums::{BlacklistArgs, ConfigArgs, QueryArgs, ReleaseRadarArgs, ReleaseRadarCmds, ShellType};

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

/// Handles the presence of a specific argument in the matches.
///
/// This macro checks if a specific argument is present in the matches and returns
/// a corresponding `HashMapArgTypes` variant based on the argument's presence..
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

/// Represents the core application interface for the Spotify Assistant CLI.
pub struct TerminalApp {
    /// A `clap::Command` instance that defines the main CLI structure, including subcommands, arguments, and behavior.
    command: Command,
}

impl TerminalApp {
    /// List of argument identifiers used in the application.
    const ARGS: &'static [&'static str] = &[
        "ttest", ];


    /// Creates a new instance of `TerminalApp`.
    ///
    /// This function initializes the main command and its subcommands.
    ///
    /// # Returns
    /// A new `TerminalApp` instance.
    pub fn new() -> Self {
        let app_cmd = Command::new("spotass")
            .version("0.2.0")
            .author("Jonathan Hill <jonathans-git@pm.me>")
            .about("A command-line interface for the Spotify Assistant program.")
            .args(
                &[
                    arg!(ttest: -t --test <KEYWORD> "Injects keyword from terminal into the app")
                ]
            )
            .subcommands(
                &[
                    Self::playlist_command(),
                    Self::release_radar_command(),
                    Self::config_command(),
                    Self::listening_history_command(),
                    Self::query_command(),
                ]
            );

        TerminalApp {
            command: app_cmd,
        }
    }

    fn styling() -> Styles {
        Styles::styled()
            .header(styling::AnsiColor::BrightGreen.on_default()
                | styling::Effects::BOLD | styling::Effects::ITALIC)
            .usage(styling::AnsiColor::BrightGreen.on_default()
                | styling::Effects::BOLD | styling::Effects::ITALIC)
            .valid(styling::RgbColor(255, 193, 0).on_default()
                | styling::Effects::ITALIC)
            .literal(styling::AnsiColor::BrightBlue.on_default()
                | styling::Effects::BOLD)
    }

    /// Runs the terminal application.
    ///
    /// This function processes the command-line arguments and executes the corresponding subcommands.
    ///
    /// # Async
    /// Asynchronous support for some commands requiring async processing.
    ///
    /// # Arguments
    /// * `print_info` - A boolean indicating whether to print additional information during execution.
    pub async fn run(&self) {
        let span = span!(Level::DEBUG, "TerminalApp.run");
        let _enter = span.enter();

        let matches = self.command.clone().get_matches();
        let subcommands_stream = stream::iter(self.command.get_subcommands());
        self.check_subcommand_conflicts(&matches);

        subcommands_stream.for_each(|cmd| async {
            let name = cmd.get_name();
            match matches.subcommand_matches(name) {
                None => {
                    event!(Level::TRACE, "No match was made for the {:?} subcommand", &name);
                }
                Some(subcommand) => {
                    match name {
                        "playlists" => {
                            event!(Level::TRACE, "Subcommand 'playlists' detected; executing run_playlist_command \
                            with the following arguments:\n{:?}", &subcommand);
                            self.run_playlist_command(&subcommand).expect("Couldn't complete the playlist command execution");
                        }
                        "config" => {
                            event!(Level::TRACE, "Subcommand 'config' detected; executing run_config_command \
                            with the following arguments:\n{:?}", &subcommand);
                            self.run_config_command(&subcommand).await.expect("Couldn't complete the config command execution");
                        }
                        "releaseradar" => {
                            event!(Level::TRACE, "Subcommand 'releaseradar' detected; executing run_rr_command \
                            with the following arguments:\n{:?}", &subcommand);
                            self.run_rr_command(&subcommand).await.expect("Couldn't complete the release radar command execution");
                        }
                        "query" => {
                            event!(Level::TRACE, "Subcommand 'query' detected; executing run_query_command \
                            with the following arguments:\n{:?}", &subcommand);
                            self.run_query_command(&subcommand).expect("Couldn't complete the query command execution");
                        }
                        "listeninghistory" => {
                            event!(Level::TRACE, "Subcommand 'listeninghistory' detected; executing \
                            run_listening_history_command with the following arguments:\n{:?}", &subcommand);
                        }
                        _ => {
                            event!(Level::TRACE, "No subcommand detected in the following input:\n{:?}", &subcommand);
                        }
                    }
                    event!(Level::DEBUG, "Subcommand: {:?}", &name);
                    event!(Level::DEBUG, "Subcommand matches:\n{:?}", &subcommand);
                }
            }
        }).await;
    }

    /// Scans and processes the playlists subcommand.
    ///
    /// This function handles the arguments for the playlists subcommand and logs information as needed.
    ///
    /// # Arguments
    /// * `matches` - The argument matches for the playlists subcommand.
    fn run_playlist_command(&self, matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        let span = span!(Level::INFO, "TerminalApp.run_playlist_command");
        let _enter = span.enter();

        let plist = matches.get_one::<bool>("plist").unwrap_or(&false).to_string();
        let pcreate = matches.get_one::<bool>("pcreate").unwrap_or(&false).to_string();
        let pmove = matches.get_one::<String>("pmove").unwrap_or(&"none".to_string()).to_string();
        let pdelete = matches.get_one::<bool>("pdelete").unwrap_or(&false).to_string();
        event!(Level::DEBUG, "Playlist list: {:?}; Playlist create: {:?}; Playlist move: {:?}; Playlist delete: {:?}", &plist, &pcreate, &pmove, &pdelete);
        Ok(())
    }

    /// Scans and processes the config subcommand.
    ///
    /// This function handles the arguments for the config subcommand, performs required actions,
    /// and executes blacklist-related functions or shell script generation when required.
    ///
    /// # Arguments
    /// * `config_arguments` - The argument matches for the config subcommand.
    async fn run_config_command(&self, config_arguments: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        let span = span!(Level::INFO, "TerminalApp.run_config_command");
        let _enter = span.enter();

        match ConfigArgs::from_matches(config_arguments) {
            ConfigArgs::Set(key, value) => {
                println!("Setting config value: {} = {}", key, value);
                Ok(())
            }
            ConfigArgs::Unset(key) => {
                println!("Unsetting config value: {}", key);
                Ok(())
            }
            ConfigArgs::Get(key) => {
                println!("Getting config value: {}", key);
                Ok(())
            }
            ConfigArgs::Shell(shell) => {
                let mut cmd = self.command.clone();
                generate_auto_complete!(shell, cmd);
                Ok(())
            }
            ConfigArgs::Blacklist(args) => {
                match self.run_blacklist_subcommand(&args).await {
                    Ok(_) => { Ok(()) }
                    Err(err) => return Err(err)
                }
            }
            ConfigArgs::Empty => {
                println!("No config subcommand");
                Ok(())
            }
        }
    }

    /// Scans and processes the blacklist subcommand.
    ///
    /// This function performs operations on the blacklist, such as adding or removing artists. It supports
    /// operations like adding artists directly, selecting from a playlist, and removing artists by name or selection.
    ///
    /// # Arguments
    /// * `blacklist_arguments` - The argument matches for the blacklist subcommand.
    async fn run_blacklist_subcommand(&self, blacklist_arguments: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        let span = span!(Level::INFO, "TerminalApp.run_config_command");
        let _enter = span.enter();

        let mut blacklist = Blacklist::new();
        match BlacklistArgs::from_matches(blacklist_arguments) {
            BlacklistArgs::Add(artist_id) => {
                let artist = match ArtistId::from_id(artist_id) {
                    Ok(artist) => artist,
                    Err(err) => {
                        println!("Could not create an artist profile from the provided ID string.");
                        eprintln!("ID Error: {:?}", err);
                        return Err(Box::new(err));
                    }
                };

                let explorer = match ArtistXplorer::new(artist.clone()).await {
                    Ok(xplorer) => {
                        xplorer
                    }
                    Err(err) => {
                        event!(Level::ERROR, "Could not retrieve artist data: {:?}", err);
                        return Err(Box::new(err));
                    }
                };
                let artist_name = explorer.artist.name;
                let artist_id = explorer.artist.id.to_string();
                let artist = BlacklistArtist::new(artist_name, artist_id);
                println!("Adding {}'s ID to blacklist: {}", artist.name(), artist.id());
                blacklist.add_artist(artist);
                Ok(())
            }
            BlacklistArgs::AddFromPlaylist(playlist) => {
                let normalized_input = playlist.clone().trim().to_lowercase();
                let user_playlists = UserPlaylists::new().await;
                let playlist_names_and_ids = user_playlists.get_user_playlists().await;
                let playlist_id = match playlist_names_and_ids.iter().find_map(|(name, id)| {
                    event!(Level::DEBUG, "Testing input as name: {:?}", &playlist);
                    if name.clone().to_lowercase() == normalized_input {
                        event!(Level::DEBUG, "Input matches a name (playlist, input): {:?}, {:?}", &playlist, &name.to_lowercase());
                        let new_id_string = id.to_string().split(':').collect::<Vec<&str>>().iter().map(|slice| slice.to_string()).collect::<Vec<String>>();
                        event!(Level::DEBUG, "ID string: {:?}", new_id_string);
                        match PlaylistId::from_id(new_id_string[2].clone()) {
                            Ok(id) => { Some(id) }
                            Err(err) => {
                                event!(Level::ERROR, "Failed to parse playlist ID '{}': {:?}", &playlist, err);
                                return None;
                            }
                        }
                    } else {
                        None
                    }
                }) {
                    Some(id) => id,
                    None => {
                        println!("Could not find a playlist matching the provided name.");
                        match PlaylistId::from_id(playlist.clone()) {
                            Ok(unwrapped_playlist_id) => {
                                event!(Level::DEBUG, "Input as ID: {:?}", playlist.clone());
                                unwrapped_playlist_id
                            },
                            Err(err) => {
                                println!("The provided input does not match any playlist IDs or names in your account.");
                                let available_names: Vec<_> = playlist_names_and_ids
                                    .keys()
                                    .map(|name| name.as_str())
                                    .collect();
                                println!("Available playlist names: {:?}", available_names);
                                eprintln!("ID Error: {:?}", err);
                                return Err(Box::new(err));
                            }
                        }
                    }
                };
                let playlist = PlaylistXplr::new(playlist_id, false).await;
                let artists = playlist.artists_by_album().await;
                let selected = match blacklist.select_artist_to_add_by_album(artists) {
                    Some(blacklisted_artist) => blacklisted_artist,
                    None => {
                        println!("No artist selected to add to the blacklist.");
                        return Ok(());
                    }
                };
                println!("Selected: {:?}", selected);
                blacklist.add_artist(selected);
                Ok(())
            }
            BlacklistArgs::RemoveByName(name) => {
                match blacklist.artists().iter().find(
                    |artist| blacklist.are_names_equal(name.as_str(), artist.name())
                ) {
                    Some(artist) => {
                        let artist_name = artist.name();
                        let artist_id = artist.id();
                        blacklist.remove_artist(artist_name.as_str(), artist_id.as_str());
                    }
                    None => {
                        println!("{} not found in the blacklist.", name);
                        return Ok(())
                    }
                }
                println!("{}'s ID was removed from the blacklist", name);
                Ok(())
            }
            BlacklistArgs::RemoveBySelect => {
                blacklist.select_artist_to_remove();
                Ok(())
            }
            BlacklistArgs::Empty => {
                blacklist.print_blacklist();
                Ok(())
            }
        }
    }

    /// Runs the Release Radar subcommand.
    ///
    /// This function handles the arguments for the Release Radar subcommand and executes the corresponding logic to
    /// update or compare playlists.
    ///
    /// # Arguments
    /// * `release_radar_arguments` - The argument matches for the Release Radar subcommand.
    async fn run_rr_command(&self, release_radar_arguments: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        match ReleaseRadarCmds::from_matches(release_radar_arguments) {
            ReleaseRadarCmds::Update(update_value) => {
                match ReleaseRadarArgs::from_update_matches(&update_value) {
                    ReleaseRadarArgs::UPrint(print) => {
                        println!("Printing Release Radar Progress: {:?}", print);
                        Ok(())
                    }
                    ReleaseRadarArgs::Empty => {
                        println!("Updating Release Radar playlist");
                        Ok(())
                    }
                    _ => {
                        Ok(())
                    }
                }
            }
            ReleaseRadarCmds::Compare(compare_value) => {
                match ReleaseRadarArgs::from_compare_matches(&compare_value) {
                    ReleaseRadarArgs::CPlaylists(playlist) => {
                        println!("Comparing Release Radar playlists: {:?}", playlist);
                        Ok(())
                    }
                    ReleaseRadarArgs::Empty => {
                        println!("No Release Radar compare argument");
                        Ok(())
                    }
                    _ => {
                        Ok(())
                    }
                }
            }
            _ => {
                println!("No Release Radar subcommand");
                Ok(())
            }
        }
    }


    /// Runs the query subcommand.
    ///
    /// This function handles the arguments for the query subcommand, allowing users to interact with playlists
    /// and the blacklist to retrieve data.
    ///
    /// # Arguments
    /// * `query_arguments` - The argument matches for the query subcommand.
    fn run_query_command(&self, query_arguments: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        let span = span!(Level::TRACE, "TerminalApp.run_query_command");
        let _enter = span.enter();

        event!(Level::DEBUG,
            "Stock RR: {:?} | Custom RR: {:?} | Blacklist: {:?}",
            query_arguments.get_one::<bool>("rlstock").unwrap_or(&false).to_string(),
            query_arguments.get_one::<bool>("rlcustom").unwrap_or(&false).to_string(),
            query_arguments.get_one::<bool>("rlblacklist").unwrap_or(&false).to_string()
        );

        match QueryArgs::from_query_matches(&query_arguments) {
            QueryArgs::QStock(stock) => {
                event!(Level::TRACE, "Querying Stock Release Radar playlists: {:?}", stock);
                Ok(())
            }
            QueryArgs::QCustom(custom) => {
                event!(Level::TRACE, "Querying Custom Release Radar playlists: {:?}", custom);
                Ok(())
            }
            QueryArgs::QBlacklist(blacklist) => {
                event!(Level::TRACE, "Querying the current blacklist: {:?}", blacklist);
                let blackist = Blacklist::new();
                blackist.print_blacklist();
                Ok(())
            }
            QueryArgs::Empty => {
                event!(Level::TRACE, "No Release Radar query argument");
                Ok(())
            }
        }
    }

    /// Defines the config subcommand.
    ///
    /// This function creates and returns the configuration subcommand with its arguments.
    ///
    /// # Returns
    /// A `Command` representing the config subcommand.
    fn config_command() -> Command {
        Command::new("config")
            .short_flag('C')
            .long_flag("config")
            .about("Configuration subcommand")
            .arg( // ! need to configure
                Arg::new("cset")
                    .help("Set a configuration value within config")
                    .short('s')
                    .long("set")
                    .help("Set a configuration value within config"),
            )
            .arg( // ! need to configure
                Arg::new("cunset")
                    .help("Unset a configuration value within config")
                    .short('u')
                    .long("unset")
                    .help("Unset a configuration value within config"),
            )
            .arg( // ! need to configure
                Arg::new("cget")
                    .help("Get a configuration value within config")
                    .short('g')
                    .long("get")
                    .help("Get a configuration value within config"),
            )
            .arg(
                Arg::new("cshell")
                    .short('S')
                    .long("shell")
                    .value_name("SHELL")
                    .value_parser(value_parser!(ShellType))
                    .help("The shell to generate the script for"),
            )
            .subcommand(
                Command::new("blacklist")
                    .short_flag('B')
                    .long_flag("blacklist")
                    .after_help("Manage the blacklist")
                    .arg( // * Works perfectly
                        Arg::new("bladd")
                            .short('a')
                            .long("add")
                            .value_name("ARTIST-ID")
                            .help("Add an artist by ID to the blacklist. Only the ID is needed ID will be prepended with spotify:artist:"),
                    )
                    .arg( // * Works perfectly
                        Arg::new("blremove")
                            .short('r')
                            .long("remove")
                            .num_args(0..=1)
                            .value_name("ARTIST-NAME")
                            .help("Remove an artist by name from the blacklist"),
                    )
                    .arg( // * Works perfectly
                        Arg::new("bladdfromplaylist")
                            .short('p')
                            .long("add-from-playlist")
                            .value_name("PLAYLIST-NAME | PLAYLIST-ID")
                            .help("Select an artist to remove from the blacklist"),
                    )
                    .group(
                        ArgGroup::new("blacklist_sub")
                            .args(&["bladd", "blremove"])
                            .required(false),
                    ),
            )
            .group(
                ArgGroup::new("prefs_sub")
                    .args(&["cset", "cunset", "cget"])
                    .required(false),
            )
            .group(
                ArgGroup::new("gen_sub")
                    .args(&["cshell"])
                    .required(false),
            )
            .styles(TerminalApp::styling())
    }

    /// Defines the playlists subcommand.
    ///
    /// This function creates and returns the playlists subcommand with its arguments.
    ///
    /// # Returns
    /// A `Command` representing the playlists subcommand.
    fn playlist_command() -> Command {
        Command::new("playlists")
            .short_flag('p')
            .about("Manage playlists")
            .arg(
                Arg::new("plist")
                    .short('l')
                    .long("list")
                    .value_parser(BoolValueParser::new()
                        .map(|b| if b { true } else { false })
                    )
                    .action(ArgAction::SetTrue)
                    .help("List all playlists"),
            )
            .arg(
                Arg::new("pcreate")
                    .short('c')
                    .long("create")
                    .help("Create a new playlists"),
            )
            .arg(
                Arg::new("pmove")
                    .short('m')
                    .long("move")
                    .value_name("folder-name")
                    .help("Move a playlists"),
            )
            .arg(
                Arg::new("pdelete")
                    .short('d')
                    .long("delete")
                    .help("Delete a playlists"),
            )
            .styles(TerminalApp::styling())
    }

    /// Defines the query subcommand.
    ///
    /// This function creates and returns the query subcommand with its arguments.
    ///
    /// # Returns
    /// A `Command` for querying playlists and blacklists.
    fn query_command() -> Command {
        Command::new("query")
            .short_flag('Q')
            .long_flag("query")
            .arg(
                Arg::new("rlstock")
                    .short('s')
                    .long("spotify")
                    .value_parser(BoolValueParser::new()
                        .map(|b| if b { true } else { false })
                    )
                    .action(ArgAction::SetTrue)
                    .conflicts_with("rlcustom")
                    .help("List all songs in the stock Release Radar playlist"),
            )
            .arg(
                Arg::new("rlcustom")
                    .short('c')
                    .long("custom")
                    .value_parser(BoolValueParser::new()
                        .map(|b| if b { true } else { false })
                    )
                    .action(ArgAction::SetTrue)
                    .help("List all songs in the full Release Radar playlist"),
            )
            .arg(
                Arg::new("rlblacklist")
                    .short('b')
                    .long("blacklist")
                    // .num_args(0..=1)
                    .value_parser(BoolValueParser::new()
                        .map(|b| if b { true } else { false })
                    )
                    .action(ArgAction::SetTrue)
                    // .value_name("ARTIST-NAME")
                    .help("Select from a list of artists pulled from your full release radar playlist \
                            or provide the name of an artist to add to your blacklist"),
            )
            .styles(TerminalApp::styling())
            .after_help("This command will list all songs in the specified Release Radar playlists")
    }

    /// Defines the Release Radar subcommand.
    ///
    /// This function creates and returns the Release Radar subcommand with its arguments and subcommands
    /// (e.g., update and compare).
    ///
    /// # Returns
    /// A `Command` representing the Release Radar subcommand.
    fn release_radar_command() -> Command {
        Command::new("releaseradar")
            .short_flag_alias('R')
            .subcommand(
                Command::new("update")
                    .short_flag('U')
                    .long_flag("update")
                    .color(ColorChoice::Always)
                    .about("Update the Release Radar playlists")
                    .arg(
                        Arg::new("print")
                            .short('p')
                            .long("print")
                            .value_parser(BoolValueParser::new()
                                .map(|b| if b { false } else { true })
                            )
                            .action(ArgAction::SetFalse)
                            .help("Print the update progress"),
                    )
            )
            .subcommand(
                Command::new("compare")
                    .short_flag('C')
                    .long_flag("compare")
                    .color(ColorChoice::Always)
                    .about("Compare the Release Radar playlists to another playlists")
                    .arg(
                        Arg::new("playlisttocompare")
                            .short('p')
                            .long("playlists")
                            .value_name("PLAYLIST-NAME")
                            .help("The exact name of the playlists to use for comparison"),
                    )
            )
            .styles(TerminalApp::styling())
    }

    /// Defines the listening history command.
    ///
    /// This function creates and defines the listening history command, including arguments for filtering,
    /// paging, and interacting with user listening history.
    ///
    /// # Returns
    /// A `Command` representing the listening history command.
    fn listening_history_command() -> Command {
        Command::new("listeninghistory")
            .short_flag('H')
            .long_flag("history")
            .about("View your listening history")
            .arg(
                Arg::new("list")
                    .short('l')
                    .long("list")
                    .num_args(0..=1)
                    .default_value("50")
                    .value_parser(value_parser!(u32))
                    .help("List the most recent 50 tracks/episodes in your listening history"),
            )
            .subcommand(
                Command::new("display")
                    .short_flag('d')
                    .long_flag("display")
                    .arg(
                        Arg::new("artist")
                            .short('a')
                            .long("artist")
                            .value_parser(value_parser!(u8).range(0..=3))
                            .help("display the artist your listening history by artist"),
                    )
                    .arg(
                        Arg::new("album")
                            .short('b')
                            .long("album")
                            .value_parser(BoolishValueParser::new())
                            .action(ArgAction::SetTrue)
                            .help("Filter your listening history by album"),
                    )
                    .arg(
                        Arg::new("track")
                            .short('t')
                            .long("track")
                            .value_parser(BoolishValueParser::new())
                            .action(ArgAction::SetTrue)
                            .help("Filter your listening history by track"),
                    )
                    .about("Get the next page of your listening history"),
            )
            .arg(
                Arg::new("next")
                    .short('n')
                    .long("next")
                    .help("Get the next page of your listening history"),
            )
            .styles(TerminalApp::styling())
    }

    /// Checks for the presence of specific arguments.
    ///
    /// This function verifies if specific arguments are present in the matches and stores their presence in a hashmap.
    ///
    /// # Arguments
    /// * `matches` - The argument matches to verify.
    ///
    /// # Returns
    /// A `HashMap` indicating whether each argument is present.
    #[allow(dead_code)]
    fn check_presence(&self, matches: &ArgMatches) -> HashMap<&str, bool> {
        let presence = HashMap::new();
        let args = Self::ARGS.to_vec();
        args.iter().for_each(|&id| {
            let _id_clone = id.to_string();
            println!("{:?}: {:?} ", id, matches.contains_id(id));
        });
        presence
    }

    /// Checks for subcommand conflicts.
    ///
    /// This function ensures that only one subcommand is used at a time. If multiple subcommands
    /// are detected in the matches, the program exits with an error message.
    ///
    /// # Arguments
    /// * `matches` - The argument matches to check for subcommand conflicts.
    fn check_subcommand_conflicts(&self, matches: &ArgMatches) {
        let _subcommands = matches.subcommand_name();
        let mut subcommand_count = 0;
        if matches.subcommand_matches("config").is_some() {
            subcommand_count += 1;
        }
        if matches.subcommand_matches("releaseradar").is_some() {
            subcommand_count += 1;
        }
        if matches.subcommand_matches("playlists").is_some() {
            subcommand_count += 1;
        }
        if subcommand_count > 1 {
            eprintln!("Error: Only one subcommand can be used at a time");
            std::process::exit(1);
        }
    }
}
