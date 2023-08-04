use mongodb::{Database, error};
use async_trait::async_trait;
use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use std::env::var;

#[async_trait]
pub trait DuplicateChecker: Send {
    async fn is_duplicate(&self, db: &Database) -> error::Result<bool>;
}

pub async fn get_db() -> Database{
    println!("{}", var("MONGO_URI").unwrap());
    let mut client_options =
    ClientOptions::parse(var("MONGO_URI").expect("MONGO_URI not found"))
        .await.expect("option error");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).expect("option error");
    let db = client.database("anachat");
    db
}
