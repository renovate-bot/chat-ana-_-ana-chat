use std::collections::HashSet;

use crate::{common::{DuplicateChecker, get_header_string}, user::User, msg::{info_chat, Chat}};
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
    message: Vec<String>,
    html: String,
    member_html: String,
}

impl ServerForJSON {
    async fn new(serv: Server) -> Self {
        let mut html = String::new();
        let mut member_html = String::new();
        let mut member = serv.member.into_iter().collect::<Vec<String>>();
        let mut message = serv.message.into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

        member.sort();
        message.sort();
        for i in message.clone().into_iter().rev(){
            let mut header = HeaderMap::new();
            header.insert("chatid", i.to_string().parse().unwrap());
            let j: Chat = serde_json::from_str(&info_chat(header).await.unwrap()).unwrap();
            html.push_str(&format!("<section id=\"msg-{}\"> <b>{}</b>: {}</section>", j._id, j.sender, j.content));
        }


        for name in &member{
            member_html.push_str(&format!("<a class=\"userInfo\"> <img src=\"/user/1.png\"/> <b>{name}</b> <span></span> </a>"),);
        }
        // println!("{html:?}");


        let a = Self {
            name: serv.name,
            member,
            message,
            html,
            member_html
        };
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
    let serv: Option<Server> = db.collection::<Server>("servers")
        .find_one(doc! {
            "name": name
        }, None)
        .await
        .unwrap();
    match serv {
        Some(s) => Ok(serde_json::to_string(&ServerForJSON::new(s).await).unwrap()),
        None => Err(StatusCode::NOT_FOUND)
    }
}
