#[cfg(feature = "mongo")]
use mongodb::bson::doc;
#[cfg(feature = "mongo")]
use mongodb::Client;

#[cfg(feature = "mongo")]
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
}
