use std::collections::HashSet;

use crate::common::DuplicateChecker;
use axum::http::HeaderMap;
use mongodb::{error, bson::{doc, uuid::Uuid}, Database};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    name: String,
    member: HashSet<String>,
    pub message: HashSet<Uuid>
}

impl Server {
    fn new(name: String, member: Option<HashSet<String>>, message: Option<HashSet<Uuid>>) -> Self {
        Self {
            name,
            member: member.unwrap_or(HashSet::new()),
            message: message.unwrap_or(HashSet::new())
        }
    }
}

#[async_trait]
impl DuplicateChecker for Server {
    async fn is_duplicate(&self, db: &Database) -> error::Result<bool>{
        let data = db.collection::<Server>("servers")
            .count_documents(doc! {
                "name": &self.name
            }, None)
            .await?;
        if data > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub async fn create_server(header: HeaderMap) -> String {
    let name = header.get("name").unwrap().to_str().unwrap().to_string();
    let db = crate::common::get_db().await;
    let serv = Server::new(name, None, None);
    if !serv.is_duplicate(&db).await.unwrap() {
        db.collection::<Server>("servers")
        .insert_one(&serv, None)
        .await
        .unwrap();
    }
    serde_json::to_string(&serv).unwrap()
}

pub async fn join_server(header: HeaderMap) -> String {
    let servname = header.get("servername").unwrap().to_str().unwrap().to_string();
    let name = header.get("username").unwrap().to_str().unwrap().to_string();
    let db = crate::common::get_db().await;
    let mut serv = db.collection::<Server>("servers")
        .find_one(doc! {
            "name": &servname
        }, None)
        .await
        .unwrap()
        .unwrap();
    if serv.member.get(&name).is_none() {
        serv.member.insert(name);
    }
    db.collection::<Server>("servers")
        .update_one(doc! {
            "name": servname
        }, doc! {
            "$set": {
                "member": serv.member.clone().into_iter().collect::<Vec<String>>()
            }
        }, None)
        .await
        .unwrap();
    serde_json::to_string(&serv).unwrap()
}

pub async fn info_server(header: HeaderMap) -> String {
    let name = header.get("name").unwrap().to_str().unwrap().to_string();
    let db = crate::common::get_db().await;
    let serv = db.collection::<Server>("servers")
        .find_one(doc! {
            "name": name
        }, None)
        .await
        .unwrap()
        .unwrap();
    serde_json::to_string(&serv).unwrap()
}
