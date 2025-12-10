// use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap, HashSet};

use chrono::{NaiveDate, NaiveDateTime};
use pbr::ProgressBar;
use rspotify::clients::BaseClient;
use rspotify::model::{
    AlbumId, ArtistId, FullAlbum, FullArtist, FullTrack, PlayableId, SimplifiedAlbum,
    SimplifiedTrack, TrackId,
};
use rspotify::{scopes, AuthCodeSpotify, ClientError};
use tracing::{error, event, info, Level};

use crate::enums::validation::BatchLimits;
use crate::paginator::PaginatorRunner;
use crate::traits::apis::Api;

/// The `ArtistXplorer` struct is used to explore details about a specific artist and their albums
/// using the Spotify API.
///
/// # Fields
///
/// - `client`:
///   - Type: `AuthCodeSpotify`
///   - Description: An authenticated Spotify API client used for making requests to the Spotify API.
///
/// - `artist_id`:
///   - Type: `ArtistId<'static>`
///   - Description: The unique identifier for the artist being explored.
///
/// - `artist`:
///   - Type: `FullArtist`
///   - Description: Contains detailed information about the artist, such as their name,
///     followers, popularity, genres, and more.
///
/// - `albums`:
///   - Type: `Vec<SimplifiedAlbum>`
///   - Description: A vector of simplified album objects representing the albums
///     associated with the artist.
pub struct ArtistXplorer {
    client: AuthCodeSpotify,
    artist_id: ArtistId<'static>,
    pub artist: FullArtist,
    pub albums: Vec<SimplifiedAlbum>,
}

impl Api for ArtistXplorer {
    fn select_scopes() -> HashSet<String> {
        scopes!("user-follow-read")
    }
}

impl ArtistXplorer {
    /// Creates a new instance of `ArtistXplorer` by retrieving the artist's information and albums from the associated API.
    ///
    /// # Parameters
    /// - `artist_id`: A unique identifier for the artist (`ArtistId<'static>`).
    ///
    /// # Returns
    /// - `Ok(Self)`: On success, returns an instance of `ArtistXplorer`, which contains the artist's metadata, their albums,
    ///   and a configured client for further API interactions.
    /// - `Err(ClientError)`: If there is an issue retrieving the artist's data or setting up the client, an error is returned.
    ///
    /// # Behavior
    /// - Sets up a scoped `tracing` span for logging and debugging purposes.
    /// - Initializes an API client by invoking `set_up_client`, with predefined scopes.
    /// - Attempts to fetch artist data based on the provided `artist_id`. If successful, logs the result.
    /// - Logs an error and returns early if the artist's data could not be retrieved.
    /// - Retrieves the artist's albums via the `artist_albums` method and performs further processing using the `albums` helper function.
    /// - Logs the successful retrieval of the artist's metadata and albums.
    ///
    /// # Errors
    /// - Returns a `ClientError` if either the artist's data or albums cannot be retrieved successfully.
    /// - Logs relevant information and error context using the `tracing` crate to aid debugging.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let artist_id = ArtistId::new("unique_artist_id");
    /// let artist_xplorer = ArtistXplorer::new(artist_id).await?;
    /// println!("Artist Name: {}", artist_xplorer.artist.name);
    /// ```
    pub async fn new(artist_id: ArtistId<'static>) -> Result<Self, ClientError> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.new");
        let _enter = span.enter();

        let client = Self::set_up_client(false, Some(Self::select_scopes())).await;
        let artist = match client.artist(artist_id.clone()).await {
            Ok(artist) => {
                info!(
                    "Data has been retrieved for the artist, '{}'.",
                    artist.name.clone()
                );
                artist
            }
            Err(error) => {
                error!(artist_id = ?artist_id.clone(), "Was not able to get data for the requested artist");
                return Err(error);
            }
        };
        let albums_req = client.artist_albums(artist_id.clone(), None, Some(Self::market()));
        let paginator = PaginatorRunner::new(albums_req, ());
        let albums = paginator.run().await.unwrap_or_else(|err| {
            event!(
                Level::ERROR,
                "Could not retrieve the artist's albums: {:?}",
                err
            );
            Vec::new()
        });
        info!("Data has been retrieved for the artist, '{}'.", artist.name);
        Ok(ArtistXplorer {
            client,
            artist_id,
            artist,
            albums,
        })
    }

    /// Categorizes and groups albums based on their release date and the specified time unit.
    ///
    /// This method processes a vector of albums (`self.albums`) and organizes them into a `BTreeMap`,
    /// where the keys are time periods (e.g., year, month, or a combination) defined by the input
    /// parameter `unit`, and the values are vectors of `SimplifiedAlbum` objects representing albums
    /// released during the respective time periods. The time periods are generated based on the release
    /// date of the albums.
    ///
    /// # Parameters
    /// - `unit`: An `Option<&str>` that specifies the time unit for grouping the albums. It can be:
    ///   - `"month"`: Groups albums by their release month.
    ///   - `"yearmonth"` or `"monthyear"`: Groups albums by a combination of year and month
    ///     (formatted as "year_month").
    ///   - `None` or any other value: Defaults to grouping albums by year.
    ///
    /// # Returns
    /// A `BTreeMap<String, Vec<SimplifiedAlbum>>` where:
    /// - The key (`String`) represents the time period (year, month, or year_month).
    /// - The value (`Vec<SimplifiedAlbum>`) is a vector of simplified albums released in that period.
    ///
    /// # Panics
    /// This function will panic if the `release_date` field of an album is `None` or is in an
    /// unexpected format that cannot be split into valid date components (e.g., year, month,
    /// or day). The panic message will include the name of the album for which parsing failed.
    ///
    /// # Behavior
    /// - The function iterates through the list of albums in `self.albums`.
    /// - For each album:
    ///   - The release date is parsed and split into components (e.g., year and month).
    ///   - The appropriate key for grouping is determined based on the `unit` parameter.
    ///   - The album is added to the corresponding key in the `BTreeMap`.
    /// - Logging information is generated for debugging purposes, including album names and
    ///   release dates.
    ///
    /// # Example
    /// ```no_run,ignore
    /// // Suppose `albums` is a vector of `SimplifiedAlbum` structures with release dates.
    /// let artist = ArtistXplorer::new(artist_id).await?;
    ///
    /// // Group albums by year.
    /// let albums_by_year = artist.albums_by_date(None);
    ///
    /// // Group albums by month.
    /// let albums_by_month = artist.albums_by_date(Some("month"));
    ///
    /// // Group albums by year and month.
    /// let albums_by_year_month = artist.albums_by_date(Some("yearmonth"));
    /// ```
    ///
    /// # Notes
    /// - The function clones the `albums` vector since it needs to mutate the data during processing.
    /// - Duplicate albums with the same release date may appear in their respective grouped vectors.
    /// - The use of a `BTreeMap` ensures that the grouped keys are sorted in lexicographical order.
    ///
    /// # Dependencies
    /// - `tracing` for logging and diagnostics.
    /// - `std::collections::BTreeMap` for organizing the grouped albums.
    pub fn albums_by_date(&self, unit: Option<&str>) -> BTreeMap<String, Vec<SimplifiedAlbum>> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.albums_by_date");
        let _enter = span.enter();

        let mut final_hash: BTreeMap<String, Vec<SimplifiedAlbum>> = BTreeMap::new();
        let mut annual_album_group: Vec<SimplifiedAlbum> = Vec::new();
        self.albums.clone().iter().for_each(|album| {
            let release_date = match album.release_date.clone() {
                Some(date) => date
                    .split("-")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                None => panic!("Could not get release date for album {}", album.name),
            };
            info!("Name: {:?} | Release Date: {:?}", album.name, release_date);
            let btree_key = match unit {
                Some("month") => {
                    let key = release_date[1].to_string();
                    key
                }
                Some("yearmonth") | Some("monthyear") => {
                    let year = release_date[0].to_string();
                    let month = release_date[1].to_string();
                    format!("{}_{}", year, month)
                }
                _ => release_date[0].to_string(),
            };
            match final_hash.get_mut::<str>(&btree_key) {
                Some(albums) => albums.push(album.clone()),
                None => {
                    annual_album_group.push(album.clone());
                    final_hash.insert(btree_key.to_string(), annual_album_group.clone());
                }
            };
        });
        final_hash
    }

    /// Categorizes and groups the `SimplifiedAlbum` objects from an artist's discography by album types
    /// and returns them as a `HashMap`. Each group is based on the `album_type` property of the album.
    ///
    /// # Parameters
    /// - `no_print` (`bool`): A boolean flag that, when set to `true`, suppresses printing information
    ///   (such as album name and type) to logs during the processing of albums. If set to `false`,
    ///   additional debug information is logged.
    ///
    /// # Returns
    /// - `HashMap<&'static str, Vec<SimplifiedAlbum>>`: A HashMap categorizing albums into the following keys:
    ///   - `"album"`: Contains all albums of type `album`.
    ///   - `"single"`: Contains all albums of type `single`.
    ///   - `"compilation"`: Contains all albums of type `compilation`.
    ///   - `"appears_on"`: Contains all albums of type `appears_on`.
    ///
    /// # Behavior
    /// 1. Iterates through the `albums` property of the calling instance.
    /// 2. Logs the name of each album if logging is enabled.
    /// 3. Matches each album's `album_type` to classify it into the appropriate category:
    ///     - If the `album_type` is `None`, the type is treated as `"n/a"`.
    ///     - If the type doesn't match `"album"`, `"single"`, `"compilation"`, or `"appears_on"`, an error
    ///       log is created for that album.
    /// 4. Returns a `HashMap` with categorized albums.
    /// 5. Optionally logs detailed album information (name and type) based on the value of `no_print`.
    ///
    /// # Logging
    /// - Logs at `INFO` level:
    ///   - The method invocation via a tracing span (`ArtistXplorer.album_by_type`).
    ///   - The names of processed albums.
    ///   - Album details if `no_print` is `false`.
    /// - Logs at `ERROR` level:
    ///   - For any album with an unrecognized or unsupported `album_type`.
    ///
    /// # Example Usage
    /// ```no_run,ignore
    ///
    /// let artist_xplorer = ArtistXplorer::new(artist_id).await?;
    /// let albums_by_category = artist_xplorer.albums_by_type(false);
    /// println!("{:?}", albums_by_category.get("album"));
    /// ```
    ///
    /// # Notes
    /// - The method clones the album's data, so the original album collection is not altered.
    /// - This implementation assumes that `SimplifiedAlbum` contains at minimum a `name` and an optional
    ///   `album_type`.
    pub fn albums_by_type(&self, no_print: bool) -> HashMap<&'static str, Vec<SimplifiedAlbum>> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.album_by_type");
        let _enter = span.enter();
        let mut albums = Vec::new();
        let mut singles = Vec::new();
        let mut compilations = Vec::new();
        let mut appears_on = Vec::new();
        self.albums.clone().iter().for_each(|album| {
            info!("{:?}", album.name);
            let alb_type = match album.album_type.clone() {
                None => "n/a".to_string(),
                Some(typeofalbum) => typeofalbum,
            };
            match alb_type.as_str() {
                "album" => {
                    albums.push(album.clone());
                }
                "single" => {
                    singles.push(album.clone());
                }
                "compilation" => {
                    compilations.push(album.clone());
                }
                "appears_on" => {
                    appears_on.push(album.clone());
                }
                _ => {
                    error!("Album type is not available for album: {:?}", album.name);
                }
            };
            if !no_print {
                info!("Name: {:?} | Type: {:?}", album.name, alb_type);
            }
        });
        HashMap::from([
            ("album", albums),
            ("single", singles),
            ("compilation", compilations),
            ("appears_on", appears_on),
        ])
    }

    /// Retrieves a list of album IDs associated with the artist's albums.
    ///
    /// This method iterates over all albums stored in the current instance, logging the name of each album
    /// using the `tracing` crate. For each album, it checks if an ID exists. If the ID exists, it is cloned
    /// and added to the resulting list. If an album does not have an ID, the method will panic with a message
    /// indicating the missing ID and the album name.
    ///
    /// # Returns
    ///
    /// A `Vec<AlbumId>` containing the IDs of all the artist's albums that have an ID.
    ///
    /// # Panics
    ///
    /// This function will panic if any album does not have an `id`, providing the name of the album
    /// in the panic message.
    ///
    /// # Logging
    ///
    /// - Logs the names of all albums at the `INFO` level using the `tracing` crate.
    /// - The tracing span is named "ArtistXplorer.album_ids" with level `INFO` for better observability.
    ///
    /// # Examples
    ///
    /// ```no_run,ignore
    /// let artist = ArtistXplorer::new(artist_id).await?;
    /// let album_ids = artist.album_ids();
    /// assert_eq!(album_ids, vec![AlbumId(1), AlbumId(2)]);
    /// ```
    pub fn album_ids(&self) -> Vec<AlbumId<'_>> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.album_ids");
        let _enter = span.enter();
        self.albums
            .clone()
            .iter()
            .map(|album| {
                info!("{:?}", album.name);
                match album.id.clone() {
                    Some(id) => id,
                    None => panic!("Could not get album ID for album {}", album.name),
                }
            })
            .collect::<Vec<AlbumId>>()
    }

    /// Filters the albums of an artist based on a cutoff date. If a cutoff date is not provided, defaults to one year ago from the current date.
    ///
    /// This method clones the current artist's album list and filters it to include only those albums whose `release_date` is later than the cutoff date.
    ///
    /// # Parameters
    ///
    /// - `cutoff`: An optional parameter of type `Option<NaiveDateTime>` representing the cutoff date. If a value is provided, it is converted to a `NaiveDate`.
    /// If no value is provided, the cutoff date is calculated as one year prior to the current local date.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `ArtistXplorer` containing the filtered list of albums in the `albums` field, while replicating the other fields (client, artist_id, artist).
    ///
    /// # Panics
    ///
    /// - This function will panic if:
    ///   - The `release_date` of an album is `None`. This indicates an album does not have a valid release date.
    ///   - Parsing the `release_date` from a string to a `NaiveDate` fails. This occurs if the release date format does not match `%Y-%m-%d`.
    ///
    /// # Examples
    ///
    /// ```no_run,ignore
    /// use chrono::{NaiveDateTime, NaiveDate, Duration};
    ///
    /// let some_cutoff = NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").ok();
    /// let artist_xplorer = ArtistXplorer::new(artist_id).await?;
    /// let filtered_albums = artist_xplorer.album_slice(some_cutoff);
    ///
    /// // Or, without providing a cutoff date:
    /// let filtered_albums_default = artist_xplorer.album_slice(None);
    /// ```
    ///
    /// This function uses `tracing` for structured logging. It logs the album names being processed, which can be useful for debugging or monitoring purposes.
    ///
    /// # Dependencies
    ///
    /// This function uses the following crates:
    /// - `tracing`: For creating and entering a logging span.
    /// - `chrono`: For working with dates and time, specifically to calculate the fallback cutoff date and parse album release dates.
    ///
    /// # Notes
    ///
    /// - The function clones data which might increase memory usage for large datasets of albums.
    /// - Ensure the `release_date` of each album is correctly formatted as `%Y-%m-%d`; otherwise, the parsing will fail.
    ///
    /// # See Also
    ///
    /// - [`chrono::NaiveDate`](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html)
    /// - [`tracing::span`](https://docs.rs/tracing/latest/tracing/span/)
    pub fn album_slice(&self, cutoff: Option<NaiveDateTime>) -> Self {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.filter_albums");
        let _enter = span.enter();
        let cutoff = match cutoff {
            Some(date) => NaiveDate::from(date),
            None => {
                let now = chrono::Local::now();
                let cutoff_date = now.date_naive() - chrono::Duration::days(365);
                cutoff_date
            }
        };

        let final_vec = self
            .albums
            .clone()
            .iter()
            .filter_map(|album| {
                info!("{:?}", album.name);
                let release_date = match album.release_date.clone() {
                    Some(date) => match NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d") {
                        Ok(dttime) => dttime,
                        Err(e) => {
                            panic!("Could not parse date: {:?}", e)
                        }
                    },
                    None => panic!("Could not get release date for album {}", album.name),
                };
                if release_date > cutoff {
                    Some(album.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<SimplifiedAlbum>>();
        let test = ArtistXplorer {
            client: self.client.clone(),
            artist_id: self.artist_id.clone(),
            artist: self.artist.clone(),
            albums: final_vec.clone(),
        };
        test
    }

    /// Asynchronously fetches the details of all full albums associated with the artist.
    ///
    /// This function retrieves the metadata for a list of album IDs tied to the current artist
    /// and compiles them into a list of `FullAlbum` objects.
    ///
    /// # Implementation Details
    /// - The function uses tracing to log its operations with an `INFO` level span.
    /// - Fetches album details in batches, determined by the `BatchLimits::Albums` limit.
    /// - Iterates over chunks of album IDs, sending requests for each chunk to the API client (`self.client`).
    /// - On successful API responses, fetched albums are logged and appended to the results list.
    /// - If the API call fails, the function will panic and display error information.
    ///
    /// # Returns
    /// A `Vec<FullAlbum>` containing all the full album details corresponding to the provided album IDs.
    ///
    /// # Panics
    /// This function will panic if:
    /// - Any API request to fetch album details fails.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// let artist_xplorer = ArtistXplorer::new(client);
    /// let albums = artist_xplorer.full_albums().await;
    /// println!("Fetched {} albums.", albums.len());
    /// ```
    ///
    /// # Performance
    /// To prevent overloading the API, the function chunks requests based on the configured `BatchLimits` size.
    /// Multiple requests may be made depending on the number of albums.
    ///
    /// # Logs
    /// The function logs:
    /// - The number of albums fetched per batch.
    ///
    pub async fn full_albums(&self) -> Vec<FullAlbum> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.full_albums");
        let _enter = span.enter();

        let mut full_albums = Vec::new();
        let limit = BatchLimits::Albums.get_limit();
        for album_id_chunk in self.album_ids().chunks(limit) {
            let full_album = match self
                .client
                .albums(album_id_chunk.to_vec(), Some(Self::market()))
                .await
            {
                Ok(full_albums) => {
                    info!("{} albums have been requested.", full_albums.len());
                    full_albums
                }
                Err(error) => panic!(
                    "ERROR: Was not able to get album from the requested artist.\nError information: {:?}",
                    error
                ),
            };
            full_albums.extend(full_album);
        }
        full_albums
    }

    /// Calculates the total number of tracks across all the albums for the current artist.
    ///
    /// This asynchronous function first retrieves all the full albums related to the artist,
    /// then iterates through the albums and sums up the total number of tracks.
    /// Debugging and informational messages are logged during the process for tracing and analysis.
    ///
    /// # Returns
    ///
    /// A `usize` representing the total number of tracks across all the albums.
    ///
    /// # Usage
    ///
    /// ```no_run,ignore
    /// let artist_instance = ArtistXplorer::new(artist_id).await?;
    /// let total_tracks = artist_instance.total_tracks().await;
    /// println!("Total number of tracks: {}", total_tracks);
    /// ```
    ///
    /// # Logging
    ///
    /// - Logs an informational message with the number of albums queried for the artist.
    /// - Logs an informational message displaying the running total of tracks for better traceability.
    ///
    /// # Errors
    ///
    /// This function assumes that the `full_albums` method correctly fetches album data
    /// and does not directly handle any errors that might occur during the asynchronous call.
    /// Ensure error handling is implemented where `total_tracks` is called.
    ///
    /// # Dependencies
    ///
    /// - `tracing::span!` is used for structured logging and trace spans.
    /// - The artist's name is extracted from the `self.artist` property.
    /// - The `full_albums` async method should return a vector of albums where each album contains `tracks.total`.
    /// ```
    pub async fn total_tracks(&self) -> usize {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.full_tracks");
        let _enter = span.enter();

        let albums = self.full_albums().await;
        info!(
            "{} albums queried for {}",
            albums.len(),
            self.artist.name.clone()
        );
        albums.clone().iter().fold(0, |acc, album| {
            info!("Running total: {}", acc + album.tracks.total);
            acc + album.tracks.total
        }) as usize
    }

    /// Asynchronously retrieves the complete list of tracks for an artist, including their detailed information.
    ///
    /// This method fetches all the tracks associated with the artist by iterating
    /// through the albums, collecting the track IDs, and then making batch API
    /// requests to retrieve full track details. The batching is necessary to comply
    /// with API limitations on the number of items that can be fetched per request.
    ///
    /// # Implementation Details
    /// - The method initializes a tracing span for observability and tracking.
    /// - Gathers all the track IDs from the albums associated with the artist.
    /// - The track IDs are divided into batches based on the API-defined limit.
    /// - For each batch, corresponding tracks are requested from the API in an asynchronous manner.
    /// - An optional progress bar is used to indicate the progress in case a large number of tracks are involved.
    ///
    /// # Error Handling
    /// - If any track does not have an ID, the function will use `panic!`, causing the application to terminate
    ///   with a message identifying the track missing an ID.
    /// - If there is an error in the API request, the function will also `panic!`, printing the error details.
    ///
    /// # Throttling
    /// To avoid overwhelming the API and to introduce delays for longer lists, a
    /// simulated wait time is applied if the total number of tracks exceeds the defined
    /// threshold (`wait_threshold`).
    ///
    /// # Progress Bar
    /// For scenarios with many tracks, a progress bar is displayed in the terminal
    /// to visually inform the user of request progress.
    ///
    /// # Returns
    /// Returns a vector containing `FullTrack` objects, each of which represents the detailed
    /// information for a track associated with the artist.
    ///
    /// # Panics
    /// - If a track does not have an ID.
    /// - If the API request to fetch tracks fails.
    ///
    /// # Examples
    /// ```no_run,ignore
    /// let artist_xplorer = ArtistXplorer::new(client);
    /// let all_full_tracks = artist_xplorer.full_tracks().await;
    /// for track in all_full_tracks {
    ///     println!("Track Name: {}", track.name);
    /// }
    /// ```
    ///
    /// # Notes
    /// - The function uses a static method `Self::market()` to pass the market
    ///   parameter for API requests. Ensure it is properly implemented.
    /// - ProgressBar is used for UI-related functionality. Ensure the `indicatif`
    ///   crate is included in your dependencies if this method is needed.
    ///
    /// # Dependencies
    /// - [`tracing`] for logging.
    /// - [`tokio`] for asynchronous runtime.
    /// - [`indicatif`] for ProgressBar.
    pub async fn full_tracks(&self) -> Vec<FullTrack> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.full_tracks");
        let _enter = span.enter();

        let mut full_tracks = Vec::new();
        let limit = BatchLimits::Tracks.get_limit();
        let albums = self.full_albums().await;
        let track_ids = albums
            .clone()
            .iter()
            .flat_map(|album| {
                album
                    .tracks
                    .items
                    .clone()
                    .iter()
                    .map(|track| match track.id.clone() {
                        Some(id) => id,
                        None => panic!("Could not get track ID for track {}", track.name),
                    })
                    .collect::<Vec<TrackId>>()
            })
            .collect::<Vec<TrackId>>();
        let chunked_ids = track_ids.chunks(limit);
        let loops = chunked_ids.len();
        let wait_threshold = 200;
        let count = 25;
        for (index, track_id_chunk) in track_ids.chunks(limit).enumerate() {
            let full_track = match self
                .client
                .tracks(track_id_chunk.to_vec(), Some(Self::market()))
                .await
            {
                Ok(full_tracks) => {
                    let remaining = (loops - (index + 1)) * limit;
                    info!(
                        "{} tracks have been requested. {} remaining tracks",
                        full_tracks.len(),
                        remaining
                    );
                    full_tracks
                }
                Err(error) => {
                    panic!(
                        "ERROR: Was not able to get album from the requested artist.\nError information: {:?}",
                        error
                    )
                }
            };
            full_tracks.extend(full_track);
            let mut pb = ProgressBar::new(count);
            pb.format("╢▌▌░╟");
            if track_ids.len() > wait_threshold {
                for _ in 0..count {
                    pb.inc();
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
            pb.finish_print("Done");
        }
        full_tracks
    }

    /// Asynchronously fetches and returns a list of collaborators (artists) for a given main artist.
    ///
    /// This function uses the artist's albums to extract all associated artists who collaborated
    /// on those albums, filters out duplicates, removes the main artist from the list, and finally
    /// retrieves the full artist data using chunks of artist IDs.
    ///
    /// # Returns
    /// A vector containing `FullArtist` objects representing each unique collaborator.
    ///
    /// # Errors
    /// - This function will panic if an artist ID cannot be retrieved from the album data.
    /// - This function will panic if an error occurs while retrieving artist information from the API.
    ///
    /// # Tracing
    /// - The process is wrapped in a tracing `span` for logging purposes.
    /// - Logs information about the length of the artist list before and after deduplication.
    /// - Logs the number of artists successfully fetched and their names.
    ///
    /// # Logic
    /// 1. Retrieves artist IDs from all albums associated with the main artist.
    /// 2. Filters out duplicate artist IDs.
    /// 3. Removes the main artist's ID from the list of collaborators.
    /// 4. Retrieves full details of the remaining unique artist collaborators by processing them in chunks.
    ///
    /// # Performance
    /// This function uses batch processing (based on `BatchLimits::Artists`) to minimize API calls
    /// when fetching artists' full details, improving the performance by dividing IDs into smaller groups.
    ///
    /// # Panics
    /// - If an artist ID is `None` in the album data, the function will panic with an error message
    ///   indicating which artist could not be processed.
    /// - If the API request to fetch artist details fails, the function will panic with detailed error output.
    ///
    /// # Example
    /// ```no_run,ignore
    /// #[tokio::main]
    /// async fn main() {
    ///     let your_struct = YourStruct::new(); // Initialize your struct
    ///     let collaborators = your_struct.collaborators().await;
    ///     for collaborator in collaborators {
    ///         println!("Collaborator: {}", collaborator.name);
    ///     }
    /// }
    /// ```
    pub async fn collaborators(&self) -> Vec<FullArtist> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.collaborations");
        let _enter = span.enter();

        let mut collaborations = Vec::new();
        let mut artists = self
            .albums
            .clone()
            .iter()
            .flat_map(|album| {
                album
                    .artists
                    .clone()
                    .iter()
                    .map(|artist| match artist.id.clone() {
                        None => panic!("Could not get artist ID for artist {}", artist.name),
                        Some(id) => id,
                    })
                    .collect::<Vec<ArtistId>>()
            })
            .collect::<Vec<ArtistId>>();
        info!("Artist length: {:?}", artists.len());
        artists = Self::clean_duplicate_id_vector(artists);
        artists.retain(|artist| *artist != self.artist_id.clone());
        info!("Artist length: {:?}", artists.len());
        let limit = BatchLimits::Artists.get_limit();
        for artist_id_chunk in artists.chunks(limit) {
            let full_artists_vec = match self.client.artists(artist_id_chunk.to_vec()).await {
                Ok(full_artists) => {
                    info!("{} artists have been requested.", full_artists.len());
                    info!(
                        "{:?}",
                        full_artists
                            .iter()
                            .map(|artist| artist.name.clone())
                            .collect::<Vec<String>>()
                    );
                    full_artists
                }
                Err(error) => panic!(
                    "ERROR: Was not able to get album from the requested artist.\nError information: {:?}",
                    error
                ),
            };
            collaborations.extend(full_artists_vec);
        }
        collaborations
    }

    /// Asynchronously fetches and returns a list of track IDs associated with the current instance.
    ///
    /// This method retrieves the tracks related to the instance using `self.tracks()`
    /// and collects their IDs into a `Vec<TrackId>`. If a track does not have an ID,
    /// the method panics with an appropriate error message.
    ///
    /// A tracing span is created to log the operation at the `INFO` level, ensuring better
    /// observability of the function's execution. Additionally, track names are logged at the
    /// `INFO` level during the process of collecting track IDs.
    ///
    /// # Returns
    ///
    /// A `Vec<TrackId>` containing the IDs of all successfully fetched tracks.
    ///
    /// # Panics
    ///
    /// Panics if a track does not contain an ID. The panic message includes the name of the track
    /// for which the ID could not be retrieved.
    ///
    /// # Examples
    ///
    /// ```no_run,ignore
    /// let artist_xplorer = ArtistXplorer::new();
    /// let track_ids = artist_xplorer.track_ids().await;
    /// println!("Track IDs: {:?}", track_ids);
    /// ```
    ///
    /// # Tracing
    ///
    /// This function utilizes the `tracing` crate to log diagnostic information:
    /// - A span named `"ArtistXplorer.track_ids"` is created at the `INFO` level.
    /// - Each track's name is logged at the `INFO` level as the IDs are collected.
    ///
    /// Note: Ensure that `self.tracks()` is implemented correctly to return a list of tracks,
    /// where each track optionally has an ID.
    pub async fn track_ids(&self) -> Vec<TrackId<'_>> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.track_ids");
        let _enter = span.enter();

        let mut track_ids = Vec::new();
        for track in self.tracks().await {
            info!("{:?}", track.name);
            match track.id.clone() {
                Some(id) => track_ids.push(id),
                None => panic!("Could not get track ID for track {}", track.name),
            }
        }
        track_ids
    }

    /// Asynchronously retrieves the tracks associated with the albums of an artist.
    ///
    /// This function fetches all the tracks belonging to the albums of a specific artist.
    /// It uses the `album_ids` associated with the artist to fetch the tracks page by page
    /// and aggregates them into a vector of `SimplifiedTrack`s.
    ///
    /// # Returns
    ///
    /// A `Vec` containing `SimplifiedTrack` objects, which represent the tracks associated
    /// with the artist's albums.
    ///
    /// # Errors
    ///
    /// If an error occurs during the retrieval of track data for an album, the function
    /// will panic with an error message indicating the issue.
    ///
    /// # Example
    ///
    /// ```no_run,ignore
    /// let artist_xplorer = ArtistXplorer::new(ArtistId::from_id("7u160I5qtBYZTQMLEIJmyz").unwrap()).await.unwrap();
    /// let tracks = artist_xplorer.tracks().await;
    /// for track in tracks {
    ///     println!("Track Name: {}", track.name);
    /// }
    /// ```
    ///
    /// # Implementation Details
    ///
    /// - This function utilizes the `self.client.album_track` method to fetch tracks for a given album ID.
    /// - The `Some(Self::market())` specifies the market context for the tracks being fetched.
    /// - It uses an `async` iterator pattern (`altracks.next().await`) to retrieve paginated results.
    ///
    /// # Tracing
    ///
    /// A `tracing::span` is created for logging and observing the process of fetching tracks for debugging
    /// purposes. The span is entered at the start of the method.
    pub async fn tracks(&self) -> Vec<SimplifiedTrack> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.tracks");
        let _enter = span.enter();

        let mut album_tracks = Vec::new();

        for album_id in self.album_ids() {
            let altracks = self
                .client
                .album_track(album_id.clone(), Some(Self::market()));
            let paginator = PaginatorRunner::new(altracks, ());
            match paginator.run().await {
                Ok(tracks) => tracks
                    .into_iter()
                    .for_each(|track| album_tracks.push(track)),
                Err(err) => {
                    panic!(
                        "ERROR: Was not able to get tracks from the requested artist.\nError information: {:?}",
                        err
                    )
                }
            }
        }
        album_tracks
    }

    /// Fetches and returns the top tracks of the current artist as a vector of `PlayableId`.
    ///
    /// This asynchronous method interacts with the client to retrieve the top tracks for the supplied artist ID.
    /// It logs the details of the operation and ensures that each track in the result set is converted into a `PlayableId`.
    ///
    /// # Returns
    /// A `Vec<PlayableId>` containing the top tracks of the artist, where each track is represented as a `PlayableId::Track`.
    ///
    /// # Panics
    /// The function will panic in the following scenarios:
    /// - If a track does not have an associated ID.
    /// - If the API call to fetch the top tracks fails, a panic is raised with the corresponding error details.
    ///
    /// # Logging
    /// The method logs the following information:
    /// - Each track's name as it processes the list of top tracks.
    /// - An informational span indicating the method context (`ArtistXplorer.top_tracks_as_playable_ids`).
    ///
    /// # Example
    /// ```no_run,ignore
    /// let artist_xplorer = ArtistXplorer::new(ArtistId::from_id("7u160I5qtBYZTQMLEIJmyz").unwrap()).await.unwrap();
    /// let playable_ids = artist_xplorer.top_tracks_as_playable_ids().await;
    /// for id in playable_ids {
    ///     println!("{:?}", id);
    /// }
    /// ```
    ///
    /// # Errors
    /// Although the function panics on errors, it is expected that this behavior will be managed
    /// by error handling in the calling context. Users may want to consider adding more resilient error handling.
    pub async fn top_tracks_as_playable_ids(&self) -> Vec<PlayableId<'_>> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.top_tracks_as_playable_ids");
        let _enter = span.enter();

        match self
            .client
            .artist_top_tracks(self.artist_id.clone(), Some(Self::market()))
            .await
        {
            Ok(top_tracks) => top_tracks
                .iter()
                .map(|track| {
                    info!("{:?}", track.name);
                    let track_id = match track.id.clone() {
                        Some(id) => id,
                        None => panic!("Could not get track ID for track {}", track.name),
                    };
                    PlayableId::Track(track_id)
                })
                .collect::<Vec<PlayableId>>(),
            Err(error) => panic!(
                "ERROR: Was not able to get album from the requested artist.\nError information: {:?}",
                error
            ),
        }
    }

    /// Fetches a list of artists related to the current artist.
    ///
    /// This asynchronous function retrieves a list of related artists using the client's `artist_related_artists` method.
    /// Each related artist is logged with their index, name, genres, follower count, and popularity score.
    /// In case of a failure during the API call, the function will panic and display an error message.
    ///
    /// # Returns
    /// A `Vec` containing `FullArtist` instances, each representing an artist related to the current artist.
    ///
    /// # Panics
    /// This function will panic if the `artist_related_artists` API call fails, with an error message describing the failure.
    ///
    /// # Logging
    /// For each related artist retrieved, information such as the artist's name, genres, follower count, and popularity is logged at the "INFO" level.
    ///
    /// # Example
    /// ```no_run,ignore
    /// let artist_xplorer_instance = ArtistXplorer::new(ArtistId::from_id("7u160I5qtBYZTQMLEIJmyz").unwrap()).await.unwrap();
    /// let related_artists = artist_xplorer_instance.related_artists().await;
    /// for artist in related_artists {
    ///     println!("Related artist: {}", artist.name);
    /// }
    /// ```
    ///
    /// # Dependencies
    /// - `tracing`: Used for span-based structured logging. The function creates a span to track this operation.
    /// - `self.client.artist_related_artists`: Fetches related artists from an external API.
    ///
    /// # Parameters
    /// - `self`: A reference to the instance of the struct containing the `related_artists` method. This struct must include `client` and `artist_id`.
    ///
    /// # Notes
    /// - Ensure that the `client` used to make API requests is properly initialized and authenticated.
    /// - The `artist_id` field must contain the current artist's ID for the query to succeed.
    ///
    /// # Errors
    /// This function does not handle errors gracefully; it panics if an error occurs. For production use, consider implementing proper error handling.
    #[deprecated(
        since = "0.14.0",
        note = "The endpoint for the artist_related_artists client method is no longer supported by Spotify"
    )]
    pub async fn related_artists(&self) -> Vec<FullArtist> {
        let span = tracing::span!(Level::INFO, "ArtistXplorer.related_artists");
        let _enter = span.enter();

        #[allow(deprecated)]
        match self
            .client
            .artist_related_artists(self.artist_id.clone())
            .await
        {
            Ok(related) => {
                for (index, artist) in related.iter().enumerate() {
                    info!(
                        "{}). {} - genres: {:?} | {} followers | {} popularity",
                        index,
                        artist.name,
                        artist.genres,
                        artist.followers.total,
                        artist.popularity
                    );
                }
                related
            }
            Err(error) => panic!(
                "ERROR: Was not able to get album from the requested artist.\nError information: {error:?}"
            ),
        }
    }

    /// Retrieves a list of genres associated with the artist.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the genres of the artist. This method clones the list of genres stored
    /// in the `artist` field of the struct, ensuring that the original data is not modified.
    ///
    /// # Example
    ///
    /// ```no_run,ignore
    /// let artist = ArtistXplorer::new(ArtistId::from_id("7u160I5qtBYZTQMLEIJmyz").unwrap()).await.unwrap();
    /// let library = Library { artist };
    /// let genres = library.genres();
    /// assert_eq!(genres, vec!["Rock", "Pop"]);
    /// ```
    ///
    /// # Note
    ///
    /// Ensure that the `artist` field is properly initialized before calling this method,
    /// as an uninitialized or empty `genres` could result in returning an empty vector.
    pub fn genres(&self) -> Vec<String> {
        self.artist.genres.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::offline::OfflineObjects;
    use chrono::NaiveDate;
    use rspotify::model::{ArtistId, FullArtist, SimplifiedAlbum};
    use rspotify::prelude::Id;

    // Test-only constructor to avoid network
    impl ArtistXplorer {
        fn new_offline(
            artist_id: ArtistId<'static>,
            artist: FullArtist,
            albums: Vec<SimplifiedAlbum>,
        ) -> Self {
            Self {
                client: OfflineObjects::dummy_client(),
                artist_id,
                artist,
                albums,
            }
        }
    }

    // Build an offline ArtistXplorer with minimal album data
    fn build_offline() -> ArtistXplorer {
        let artist_id = ArtistId::from_id("ARTIST1234567890123456").unwrap();
        let artist = OfflineObjects::sample_full_artist();
        let albums = OfflineObjects::sample_simplified_album_for(artist_id.id(), "Example Artist");
        ArtistXplorer::new_offline(artist_id, artist, albums)
    }

    #[test]
    fn test_offline_albums_by_type_and_ids() {
        let x = build_offline();
        let by_type = x.albums_by_type(true);
        assert_eq!(by_type.get("album").unwrap().len(), 1);
        assert_eq!(by_type.get("single").unwrap().len(), 1);
        let ids = x.album_ids();
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_offline_album_slice_and_genres() {
        let x = build_offline();
        // cutoff after 2023 so only the 2024 album remains
        let cutoff_dt = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let sliced = x.album_slice(Some(cutoff_dt));
        assert_eq!(sliced.albums.len(), 1);
        assert_eq!(sliced.albums[0].name, "Example Album");
        let genres = x.genres();
        assert_eq!(genres, vec!["Rock".to_string(), "Alt".to_string()]);
    }

    // Offline test for collaborators() would require network; we will test track_ids and tracks equivalents via album data only.
}
