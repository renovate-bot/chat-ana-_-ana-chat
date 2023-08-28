use std::collections::HashSet;

use crate::{common::{DuplicateChecker, get_header_string}, user::User};
use axum::http::{HeaderMap, StatusCode};
use mongodb::{bson::{doc, oid::ObjectId}, Database};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    name: String,
    member: HashSet<String>,
    pub message: HashSet<ObjectId>
}

impl Server {
    fn new(name: String, member: Option<HashSet<String>>, message: Option<HashSet<ObjectId>>) -> Self {
        Self {
            name,
            member: member.unwrap_or_default(),
            message: message.unwrap_or_default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct ServerForJSON {
    name: String,
    member: Vec<String>,
    message: Vec<String>
}

impl ServerForJSON {
    fn new(serv: Server) -> Self {
        let mut a = Self {
            name: serv.name,
            member: serv.member.into_iter().collect::<Vec<String>>(),
            message: serv.message.into_iter().map(|x| x.to_string()).collect()
        };
        a.member.sort();
        a.message.sort();
        a
    }
}

#[async_trait]
impl DuplicateChecker for Server {
    async fn is_duplicate(&self, db: &Database) -> Result<bool, StatusCode>{
        let data = db.collection::<Server>("servers")
            .count_documents(doc! {
                "name": &self.name
            }, None)
            .await;
        match data {
            Ok(a) => {
                if a > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            Err(e) => {
                println!("{:?}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn create_server(header: HeaderMap) -> Result<String, StatusCode> {
    let name = get_header_string(&header, "name")?;
    let db = crate::common::get_db().await;
    let serv = Server::new(name, None, None);
    if !serv.is_duplicate(&db).await? {
        db.collection::<Server>("servers")
        .insert_one(&serv, None)
        .await
        .unwrap();
    }
    Ok(serde_json::to_string(&serv).unwrap())
}

pub async fn join_server(header: HeaderMap) -> Result<String, StatusCode> {
    let servname = get_header_string(&header, "servername")?;
    let name = get_header_string(&header, "username")?;
    let db = crate::common::get_db().await;
    let mut serv = match db.collection::<Server>("servers")
        .find_one(doc! {
            "name": &servname
        }, None)
        .await
        .unwrap() {
            Some(a) => a,
            None => return Err(StatusCode::NOT_FOUND)
        };
    let mut member = match db.collection::<User>("users")
        .find_one( doc! {
            "name": &name
        }, None).await.unwrap() {
            Some(a) => a,
            None => return Err(StatusCode::NOT_FOUND)
        };
    serv.member.insert(member.name.clone());
    member.servers.push(serv.name.clone());
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
    db.collection::<User>("users")
    .update_one(doc! {
        "name": member.name
    }, doc! {
        "$set": {
            "servers": member.servers.clone().into_iter().collect::<Vec<String>>()
        }
    }, None)
    .await
    .unwrap();
    Ok(serde_json::to_string(&serv).unwrap())
}

pub async fn info_server(header: HeaderMap) -> Result<String, StatusCode> {
    let name = get_header_string(&header, "name")?;
    let db = crate::common::get_db().await;
    let serv = db.collection::<Server>("servers")
        .find_one(doc! {
            "name": name
        }, None)
        .await
        .unwrap();
    match serv {
        Some(s) => Ok(serde_json::to_string(&ServerForJSON::new(s)).unwrap()),
        None => Err(StatusCode::NOT_FOUND)
    }
}
