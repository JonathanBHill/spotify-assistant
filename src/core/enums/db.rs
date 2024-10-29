#[cfg(feature = "rusqlite")]
pub enum SQLiteStatements {
    InitTables(InitTables),
    Insert(Insert),
}

#[cfg(feature = "rusqlite")]
impl SQLiteStatements {
    pub fn string(&self) -> String {
        match self {
            SQLiteStatements::InitTables(init) => {
                let mut table_columns: HashMap<&str, &str> = HashMap::new();
                match init {
                    InitTables::User => {
                        table_columns = TableColumns::User.with_properties();
                    }
                    InitTables::Playlists => {
                        table_columns = TableColumns::Playlists.with_properties();
                    }
                    InitTables::PlaylistTracks => {
                        table_columns = TableColumns::Tracks.with_properties();
                    }
                    InitTables::FollowedArtists => {
                        table_columns = TableColumns::Artists.with_properties();
                    }
                    InitTables::LikedTrackArtists => {
                        table_columns = TableColumns::Artists.with_properties();
                    }
                    InitTables::LikedTracks => {
                        table_columns = TableColumns::Tracks.with_properties();
                    }
                }
                init.new_table(table_columns)
            }
            _ => "".to_string(),
        }
    }
    pub async fn async_string(&self) -> String {
        match self {
            SQLiteStatements::Insert(insert) => insert.string().await,
            _ => "".to_string(),
        }
    }
}

#[cfg(feature = "rusqlite")]
pub enum TableColumns {
    User,
    Playlists,
    Tracks,
    Artists,
}

#[cfg(feature = "rusqlite")]
impl TableColumns {
    pub fn with_properties(&self) -> HashMap<&str, &str> {
        let mut table_columns: HashMap<&str, &str> = HashMap::new();
        table_columns.insert("id", "integer primary key");
        table_columns.insert("name", "varchar(60) not null");
        table_columns.insert("spotify_url", "varchar(60)");
        table_columns.insert("href", "varchar(60)");
        table_columns.insert("image", "varchar(60)"); // Image URL
        match self {
            TableColumns::User => {
                table_columns.insert("user_id", "varchar(45) not null");
                table_columns.insert("email", "varchar(45) not null");
                table_columns.insert("plan", "varchar(7) not null");
                table_columns.insert("followers", "integer");
                table_columns.insert("explicit_filter_enabled", "boolean");
                table_columns.insert("explicit_filter_locked", "boolean");
                table_columns.insert("last_updated", "datetime");
                table_columns
            }
            TableColumns::Playlists => {
                table_columns.insert("collaborative", "boolean");
                table_columns.insert("description", "varchar(100)");
                table_columns.insert("followers", "integer"); // Follower totals
                table_columns.insert("owner", "varchar(60)");
                table_columns.insert("public", "boolean");
                table_columns.insert("snapshot_id", "varchar(60)");
                table_columns.insert("tracks", "integer"); // Track totals
                table_columns
            }
            TableColumns::Tracks => {
                table_columns.insert("album", "varchar(60)");
                table_columns.insert("artist", "varchar(30)");
                table_columns.insert("artist_id", "varchar(60)");
                table_columns.insert("additional_artists", "varchar(60)");
                table_columns.insert("additional_artist_ids", "varchar(60)");
                table_columns.insert("disc_number", "integer");
                table_columns.insert("duration", "integer");
                table_columns.insert("explicit", "boolean");
                table_columns.insert("external_id", "varchar(60)");
                table_columns.insert("stored_local", "boolean");
                table_columns.insert("playable", "boolean");
                table_columns.insert("linked_from", "varchar(60)");
                table_columns.insert("restrictions", "varchar(60)");
                table_columns.insert("popularity", "integer");
                table_columns.insert("track_number", "integer");
                table_columns.insert("preview_url", "varchar(60)");
                table_columns.insert("playlist_id", "integer not null");
                table_columns.insert("added_at", "datetime");
                table_columns.insert("added_by", "varchar(60)");
                table_columns
            }
            TableColumns::Artists => {
                table_columns.insert("followers", "integer");
                table_columns.insert("genres", "varchar(60)");
                table_columns.insert("popularity", "integer");
                table_columns
            }
        }
    }
    pub fn names(&self, primary_key: bool) -> Vec<&str> {
        let mut table_names = match self {
            TableColumns::User => {
                vec![
                    "user_id",
                    "name",
                    "email",
                    "plan",
                    "followers",
                    "explicit_filter_enabled",
                    "explicit_filter_locked",
                    "spotify_url",
                    "href",
                    "image",
                    "last_updated",
                ]
            }
            TableColumns::Playlists => {
                vec![
                    "collaborative",
                    "description",
                    "followers",
                    "owner",
                    "public",
                    "snapshot_id",
                    "tracks",
                ]
            }
            TableColumns::Tracks => {
                vec![
                    "album",
                    "artist",
                    "artist_id",
                    "additional_artists",
                    "additional_artist_ids",
                    "disc_number",
                    "duration",
                    "explicit",
                    "external_id",
                    "stored_local",
                    "playable",
                    "linked_from",
                    "restrictions",
                    "popularity",
                    "track_number",
                    "preview_url",
                    "playlist_id",
                    "added_at",
                    "added_by",
                ]
            }
            TableColumns::Artists => {
                vec!["followers", "genres", "popularity"]
            }
        };
        if primary_key {
            table_names.insert(0, "id");
        }
        table_names
    }
}

#[cfg(feature = "rusqlite")]
pub enum InitTables {
    User,
    Playlists,
    PlaylistTracks,
    LikedTrackArtists,
    FollowedArtists,
    LikedTracks,
}

#[cfg(feature = "rusqlite")]
impl InitTables {
    pub fn new_table(&self, table_columns: HashMap<&str, &str>) -> String {
        let return_data = match self {
            InitTables::User => {
                let ordered_columns = TableColumns::User.names(true);
                (
                    "user",
                    ordered_columns
                        .iter()
                        .map(|key| format!("{} {}", key, table_columns.get(key).unwrap()))
                        .collect::<Vec<String>>()
                        .join(",\n"),
                )
            }
            InitTables::Playlists => {
                let ordered_columns = TableColumns::Playlists.names(true);
                (
                    "playlists",
                    ordered_columns
                        .iter()
                        .map(|key| format!("{} {}", key, table_columns.get(key).unwrap()))
                        .collect::<Vec<String>>()
                        .join(",\n"),
                )
            }
            InitTables::PlaylistTracks => {
                let ordered_columns = TableColumns::Tracks.names(true);
                (
                    "playlist_tracks",
                    format!(
                        "{},\n{}",
                        ordered_columns
                            .iter()
                            .map(|key| format!("{} {}", key, table_columns.get(key).unwrap()))
                            .collect::<Vec<String>>()
                            .join(",\n"),
                        "foreign key (playlist_id) references playlists(id)"
                    ),
                )
            }
            InitTables::LikedTracks => {
                let ordered_columns = TableColumns::Tracks.names(true);
                (
                    "liked_tracks",
                    ordered_columns
                        .iter()
                        .map(|key| format!("{} {}", key, table_columns.get(key).unwrap()))
                        .collect::<Vec<String>>()
                        .join(",\n"),
                )
            }
            InitTables::LikedTrackArtists => {
                let ordered_columns = TableColumns::Artists.names(true);
                (
                    "liked_track_artists",
                    ordered_columns
                        .iter()
                        .map(|key| format!("{} {}", key, table_columns.get(key).unwrap()))
                        .collect::<Vec<String>>()
                        .join(",\n"),
                )
            }
            InitTables::FollowedArtists => {
                let ordered_columns = TableColumns::Artists.names(true);
                (
                    "followed_artists",
                    ordered_columns
                        .iter()
                        .map(|key| format!("{} {}", key, table_columns.get(key).unwrap()))
                        .collect::<Vec<String>>()
                        .join(",\n"),
                )
            }
        };
        format!(
            "create table if not exists {} ({})",
            return_data.0, return_data.1
        )
    }
}

#[cfg(feature = "rusqlite")]
pub enum Insert {
    User,
    Playlists,
    PlaylistTracks,
    LikedTrackArtists,
    FollowedArtists,
    LikedTracks,
}

#[cfg(feature = "rusqlite")]
impl Insert {
    pub async fn string(&self) -> String {
        match self {
            Insert::User => {
                let column_names = TableColumns::User.names(false);
                format!(
                    "insert into user ({}) values ({})",
                    column_names
                        .iter()
                        .map(|key| key.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    column_names
                        .iter()
                        .enumerate()
                        .map(|(index, _)| format!("?{}", index + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Insert::Playlists => {
                let column_names = TableColumns::Playlists.names(false);
                format!(
                    "insert into playlists ({}) values ({})",
                    column_names
                        .iter()
                        .map(|key| key.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    column_names
                        .iter()
                        .enumerate()
                        .map(|(index, _)| format!("?{}", index + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Insert::PlaylistTracks => {
                let column_names = TableColumns::Tracks.names(false);
                format!(
                    "insert into playlist_tracks ({}) values ({})",
                    column_names
                        .iter()
                        .map(|key| key.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    column_names
                        .iter()
                        .enumerate()
                        .map(|(index, _)| format!("?{}", index + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Insert::LikedTrackArtists => {
                let column_names = TableColumns::Artists.names(false);
                format!(
                    "insert into liked_track_artists ({}) values ({})",
                    column_names
                        .iter()
                        .map(|key| key.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    column_names
                        .iter()
                        .enumerate()
                        .map(|(index, _)| format!("?{}", index + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Insert::FollowedArtists => {
                let column_names = TableColumns::Artists.names(false);
                format!(
                    "insert into followed_artists ({}) values ({})",
                    column_names
                        .iter()
                        .map(|key| key.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    column_names
                        .iter()
                        .enumerate()
                        .map(|(index, _)| format!("?{}", index + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Insert::LikedTracks => {
                let column_names = TableColumns::Tracks.names(false);
                format!(
                    "insert into liked_tracks ({}) values ({})",
                    column_names
                        .iter()
                        .map(|key| key.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    column_names
                        .iter()
                        .enumerate()
                        .map(|(index, _)| format!("?{}", index + 1))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}
