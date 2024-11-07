use std::env;

use mongodb::bson::doc;
use mongodb::Client;

use spotify_assistant_core::enums::fs::ProjectFiles;

use crate::mongo::groups::Clusters;

pub trait MongoConnection {
    fn connection_string() -> String {
        dotenv::from_path(ProjectFiles::DotEnv.path()).ok();
        let username = env::var("MONGODB_USERNAME").expect("MONGODB_USERNAME must be set");
        let password = env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD must be set");
        let clust = Clusters::default();
        clust.cluster_url(username, password)
    }
    fn test_connection(
        client: &Client,
    ) -> impl std::future::Future<Output = mongodb::error::Result<bool>> + Send {
        async {
            let test = client
                .database("spotify")
                .run_command(doc! {"ping": 1})
                .await;
            match test {
                Ok(_) => {
                    println!("Successfully pinged the spotify database.");
                    Ok(true)
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    Ok(false)
                }
            }
        }
    }
    // fn alive(&self) -> std::future::Future<Output = bool> + Send;
    // fn insert_document(&self, db_name: &str, coll_name: &str, doc: Person) -> std::future::Future<Output = ()> + Send;
    // fn get_collection(&self, db_name: &str, coll_name: &str) -> Collection<FullArtist>;
}
