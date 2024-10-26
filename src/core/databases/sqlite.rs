use std::collections::HashMap;
use std::path::PathBuf;

use rusqlite::{Connection, Error, params};
use tracing::{debug, event, info, Level};

use crate::core::databases::querying::UserDB;
use crate::core::enums::db::{InitTables, Insert, SQLiteStatements};
use crate::core::utilities::filesystem::initialization::ProjectFileSystem;

#[cfg(feature = "sqlite")]
pub struct Sqweel {
    pub conn: rusqlite::Connection,
    pub data_dir: PathBuf,
}
impl Sqweel {
    pub fn new() -> Result<Self, Error> {
        let span = tracing::span!(Level::INFO, "Sqweel.new");
        let _enter = span.enter();

        let data_dir = ProjectFileSystem::new().data_directory.path();
        let db_data_path = "databases/main.db".to_string();
        let db_path = data_dir.clone().join(db_data_path);
        let conn = Connection::open(db_path)?;
        info!("Connection to database has been established.");
        Ok(Sqweel { conn, data_dir })
    }
    pub fn init(&self) -> Result<(), Error> {
        let span = tracing::span!(Level::DEBUG, "Sqweel.init");
        let _enter = span.enter();

        let mut init_table_statements = HashMap::new();
        init_table_statements.insert("user", SQLiteStatements::InitTables(InitTables::User).string());
        init_table_statements.insert("playlists", SQLiteStatements::InitTables(InitTables::Playlists).string());
        init_table_statements.insert("playlist_tracks", SQLiteStatements::InitTables(InitTables::PlaylistTracks).string());
        init_table_statements.insert("followed_artists", SQLiteStatements::InitTables(InitTables::FollowedArtists).string());
        init_table_statements.insert("liked_tracks", SQLiteStatements::InitTables(InitTables::LikedTracks).string());
        init_table_statements.insert("liked_track_artists", SQLiteStatements::InitTables(InitTables::LikedTrackArtists).string());

        for (key, statement) in init_table_statements.iter() {
            let mut key_as_chars = key.chars();
            let table_name = match key_as_chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + key_as_chars.as_str(),
            };
            event!(Level::DEBUG, table_name = ?table_name, statement = ?statement, "Initializing table");
            self.conn.execute(statement, [])?;
            event!(Level::INFO, "{} table has been initialized.", table_name);
        }
        Ok(())
    }
    pub fn view_tables(&self) -> Result<(), Error> {
        let span = tracing::span!(Level::INFO, "Sqweel.view_tables");
        let _enter = span.enter();

        let mut tables = self.conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let table_iter = tables.query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })?;
        for table in table_iter {
            debug!("{:?}", table);
        }
        Ok(())
    }
    pub async fn write_user(&self) -> Result<(), Error> {
        let span = tracing::span!(Level::INFO, "Sqweel.write_user");
        let _enter = span.enter();

        let user_statement = SQLiteStatements::Insert(Insert::User).async_string().await;
        let user = UserDB::new().await;

        self.conn.execute("delete from user", [])?;
        info!("Wiping user table");
        self.conn.execute(&user_statement, params![
            &user.id, &user.name, &user.email, &user.plan,
            &user.followers, &user.explicit_filter_enabled,
            &user.explicit_filter_locked, &user.spotify_url,
            &user.href, &user.image, &user.last_updated,
        ])?;
        Ok(())
    }
}
