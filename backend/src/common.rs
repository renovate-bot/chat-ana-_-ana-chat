use axum::http::{HeaderMap, StatusCode};
use async_trait::async_trait;
use mongodb::{Database, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use std::env::var;

#[async_trait]
pub trait DuplicateChecker: Send {
    async fn is_duplicate(&self, db: &Database) -> Result<bool, StatusCode>;
}

pub fn get_header_string(header: &HeaderMap, name: &str) -> Result<String, StatusCode>{
    match header.get(name) {
        Some(val) => {
            match val.to_str() {
                Ok(val) => Ok(val.to_string()),
                Err(_) => Err(StatusCode::BAD_REQUEST)
            }
        },
        None => Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn get_db() -> Database{
    let mut client_options =
    ClientOptions::parse(var("MONGO_URI").expect("MONGO_URI not found"))
        .await.expect("option error");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).expect("option error");
    
    client.database("anachat")
}
