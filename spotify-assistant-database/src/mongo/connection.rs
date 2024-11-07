use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

use spotify_assistant_core::enums::fs::ProjectFiles;

use crate::mongo::groups::Clusters;
use crate::mongo::models::RecommendedRecord;
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
        let mongo_ob = Artist { client };
        Ok(mongo_ob)
    }
}

// pub
pub struct Recommendations {
    client: Client,
    database_name: &'static str,
    collection_name: &'static str,
}

impl MongoConnection for Recommendations {}

impl Recommendations {
    pub async fn new() -> mongodb::error::Result<Self> {
        let connection_string = Self::connection_string();

        let mut client_options = ClientOptions::parse(&connection_string).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options)?;
        if Recommendations::test_connection(&client).await? {
            let mongo_ob = Recommendations {
                client,
                database_name: "spotify",
                collection_name: "recommendations",
            };
            Ok(mongo_ob)
        } else {
            Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not connect to the database",
            )))
        }
    }
    pub async fn update_document(&self) -> mongodb::error::Result<()> {
        let coll: Collection<RecommendedRecord> = self
            .client
            .database(self.database_name)
            .collection(self.collection_name);
        let update = coll
            .update_one(
                doc! {"nickname": "dub_trap_11_04_24"},
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
