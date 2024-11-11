use std::collections::HashMap;
use std::future::Future;

use mongodb::{Client, Collection};
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use rspotify::model::{FullArtist, FullTrack, Recommendations, RecommendationsSeedType};
use rspotify::prelude::{BaseClient, Id};
use serde::{Deserialize, Serialize};

use spotify_assistant_core::enums::fs::ProjectFiles;

use crate::mongo::groups::Clusters;
use crate::mongo::models::{ArtistRecord, RecommendedRecord};
use crate::mongo::traits::MongoConnection;

pub struct Connection {
    client: Client,
}

impl Connection {
    pub async fn new() -> mongodb::error::Result<Self> {
        dotenv::from_path(ProjectFiles::DotEnv.path()).ok();
        let username = std::env::var("MONGODB_USERNAME").expect("MONGODB_USERNAME must be set");
        let password = std::env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD must be set");
        let clust = Clusters::default();
        let connection_string = clust.cluster_url(username, password);
        let mut client_options = ClientOptions::parse(&connection_string).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options)?;
        let mongo_ob = Connection { client };
        mongo_ob.test_connection().await?;
        Ok(mongo_ob)
    }
    pub async fn alive(&self) -> mongodb::error::Result<bool> {
        let test = self
            .client
            .database("admin")
            .run_command(doc! {"ping": 1})
            .await;
        match test {
            Ok(_) => {
                println!("Alive");
                Ok(true)
            }
            Err(err) => {
                println!("Error: {:?}", err);
                Ok(false)
            }
        }
    }
    async fn test_connection(&self) -> mongodb::error::Result<()> {
        self.client
            .database("admin")
            .run_command(doc! {"ping": 1})
            .await?;
        println!("Pinged your deployment. You successfully connected to MongoDB!");
        Ok(())
    }
}

pub struct Artist {
    client: Client,
    database_name: &'static str,
    collection_name: &'static str,
    collection: Collection<ArtistRecord>,
}

impl MongoConnection for Artist {}

impl Artist {
    pub async fn new() -> mongodb::error::Result<Self> {
        let connection_string = Self::connection_string();

        let mut client_options = ClientOptions::parse(&connection_string).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options)?;
        Artist::test_connection(&client).await?;
        let collection: Collection<ArtistRecord> = client
            .database("spotify")
            .collection("artists");
        let mongo_ob = Artist {
            client,
            database_name: "spotify",
            collection_name: "artists",
            collection,
        };
        Ok(mongo_ob)
    }
    pub async fn get_documents(&self, filter: HashMap<&str, &str>) -> mongodb::error::Result<Vec<ArtistRecord>> {
        let (x, y) = filter.get_key_value("artist_name").unwrap();
        let filter = doc! {x.to_string(): y.to_string()};
        // let doc = self.collection.find(filter).await?;
        // let t = doc.deserialize_current();
        // t
        let mut return_records = Vec::new();
        let mut cursor = self.collection.find(filter).await?;

        while cursor.advance().await? {
            return_records.push(cursor.deserialize_current()?);
            println!("{:?}", cursor.deserialize_current()?);
        };
        Ok(return_records)


        // match t {
        //         Ok(doc) => {
        //             println!("{:?}", doc);
        //             Ok(doc)
        //         },
        //         // None => Err(mongodb::error::Error::from(mongodb::error::ErrorKind::InvalidArgument {
        //         //     message: format!("No document could be found with the following filter: {}", filter),
        //         // })),
        //         _ => Err(mongodb::error::Error::from(std::io::Error::new(
        //             std::io::ErrorKind::Other,
        //             "No document could be found with the following filter",
        //         ))),
        //     }
    }
    pub fn format_documents(&self, artists: Vec<FullArtist>, followed: Option<bool>) -> Vec<ArtistRecord> {
        let records = artists
            .iter()
            .map(|artist| self.format_document(artist.clone(), followed))
            .collect();
        records
    }
    pub fn format_document(&self, artist: FullArtist, followed: Option<bool>) -> ArtistRecord {
        let now = chrono::Local::now();
        let date_formatted = now.format("%m-%d-%Y").to_string();
        let time_formatted = now.format("%H:%M:%S").to_string();
        let mut datetime = HashMap::from([
            ("date".to_string(), date_formatted.clone()),
            ("time".to_string(), time_formatted),
        ]);
        ArtistRecord {
            id: artist.id.id().to_string(),
            name: artist.name.clone(),
            external_url: artist.external_urls.clone().get("spotify").unwrap().clone(),
            genres: artist.genres.clone(),
            followers: artist.followers.total,
            followed: followed.unwrap_or(false),
            updated: datetime,
            albums: None,
            singles: None,
            compilations: None,
            appears_on: None,
            popularity: artist.popularity,
            tracks: None,
        }
    }
    pub async fn insert_documents(&self, artists: Vec<ArtistRecord>) -> mongodb::error::Result<()> {
        self.collection.insert_many(artists).await?;
        let count = self.collection.estimated_document_count().await?;
        println!("{:?} documents were successfully added", count);
        Ok(())
    }
    pub async fn insert_document(&self, doc: ArtistRecord) -> mongodb::error::Result<()> {
        self.collection.insert_one(doc).await?;
        let count = self.collection.estimated_document_count().await?;
        println!("{:?} document was successfully added", count);
        Ok(())
    }
    pub async fn remove_document(&self, artist_name: String) -> mongodb::error::Result<()> {
        let coll: Collection<RecommendedRecord> = self
            .client
            .database(self.database_name)
            .collection(self.collection_name);
        let delete = coll.delete_one(doc! {"name": artist_name}).await?;
        println!(
            "{:?} document was successfully removed",
            delete.deleted_count
        );
        Ok(())
    }
    pub async fn replace_document(&self, name: String, doc: ArtistRecord) -> mongodb::error::Result<()> {
        let replace = self.collection.replace_one(doc! {"name": name}, doc).await?;
        println!(
            "{:?} document was successfully replaced",
            replace.modified_count
        );
        Ok(())
    }
}

// pub
pub struct Recommendation {
    client: Client,
    database_name: &'static str,
    collection_name: &'static str,
    collection: Collection<RecommendedRecord>,
}

impl MongoConnection for Recommendations {}

impl Recommendation {
    pub async fn new() -> mongodb::error::Result<Self> {
        let connection_string = Self::connection_string();

        let mut client_options = ClientOptions::parse(&connection_string).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options)?;
        let collection = client.database("spotify").collection("recommendations").clone();
        if Recommendation::test_connection(&client).await? {
            let mongo_ob = Recommendation {
                client: client,
                database_name: "spotify",
                collection_name: "recommendations",
                collection,
            };
            Ok(mongo_ob)
        } else {
            Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not connect to the database",
            )))
        }
    }
    pub async fn format_document(&self, tracks: Vec<FullTrack>, recommendations: Recommendations) -> RecommendedRecord {
        let now = chrono::Local::now();
        let date_formatted = now.format("%m-%d-%Y").to_string();
        let time_formatted = now.format("%H:%M:%S").to_string();
        let mut datetime = HashMap::new();
        datetime.insert("date".to_string(), date_formatted.clone());
        datetime.insert("time".to_string(), time_formatted);
        let generation_seeds = recommendations.seeds;
        // let tracks = self.simple_to_full_tracks(recommendations.tracks).await;
        let mut tracker: HashMap<&str, i8> =
            HashMap::from([("artists", 0), ("genres", 0), ("tracks", 0)]);
        fn add_to_tracker<'a>(tracker: &mut HashMap<&'a str, i8>, key: &'a str) {
            let current = tracker.remove(key).unwrap();
            let new = current + 1;
            tracker.insert(key, new);
        }
        generation_seeds
            .iter()
            .for_each(|seed| {
                let key = match seed._type.clone() {
                    RecommendationsSeedType::Artist => "artists",
                    RecommendationsSeedType::Genre => "genres",
                    RecommendationsSeedType::Track => "tracks",
                };
                add_to_tracker(&mut tracker, key);
            });
        let name = format!(
            "{}art_{}gen_{}trk_{}",
            tracker.get("artists").unwrap(),
            tracker.get("genres").unwrap(),
            tracker.get("tracks").unwrap(),
            date_formatted.clone()
        );
        RecommendedRecord {
            name,
            generation_seeds,
            tracks,
            datetime,
        }
    }
    pub async fn update_document(&self, nickname: &str) -> mongodb::error::Result<()> {
        let coll: Collection<RecommendedRecord> = self
            .client
            .database(self.database_name)
            .collection(self.collection_name);
        let update = coll
            .update_one(
                // nickname = "dub_trap_11_04_24"
                doc! {"nickname": nickname},
                doc! {"$set": doc! {"generated_on": {"date": "11-04-2024"}}},
            )
            .await?;
        println!(
            "{:?} document was successfully updated",
            update.modified_count
        );
        Ok(())
    }
    pub async fn remove_document(&self, nickname: String) -> mongodb::error::Result<()> {
        let coll: Collection<RecommendedRecord> = self
            .client
            .database(self.database_name)
            .collection(self.collection_name);
        let delete = coll.delete_one(doc! {"nickname": nickname}).await?;
        println!(
            "{:?} document was successfully removed",
            delete.deleted_count
        );
        Ok(())
    }
    pub async fn replace_document(
        &self,
        nickname: String,
        doc: RecommendedRecord,
    ) -> mongodb::error::Result<()> {
        let coll: Collection<RecommendedRecord> = self
            .client
            .database(self.database_name)
            .collection(self.collection_name);
        let replace = coll.replace_one(doc! {"nickname": nickname}, doc).await?;
        println!(
            "{:?} document was successfully replaced",
            replace.modified_count
        );
        Ok(())
    }
    pub async fn insert_document(&self, doc: RecommendedRecord) -> mongodb::error::Result<()> {
        let coll: Collection<RecommendedRecord> = self
            .client
            .database(self.database_name)
            .collection(self.collection_name);
        coll.insert_one(doc).await?;
        let count = coll.estimated_document_count().await?;
        println!("{:?} document was successfully added", count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection() {
        let connection = Artist::new().await.unwrap();
        let alive = Artist::test_connection(&connection.client).await.unwrap();
        assert!(alive);
    }
}
