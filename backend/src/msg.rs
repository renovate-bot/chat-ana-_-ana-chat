use axum::http::HeaderMap;
use chrono::prelude::{DateTime, Utc};
use mongodb::bson::{doc, uuid::Uuid};
use serde::{Deserialize, Serialize};
use crate::server::Server;

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    _id: Option<Uuid>,
    sender: String,
    content: String,
    date: DateTime<Utc>
}

impl Chat {
    pub fn new(sender: String, content: String, date: Option<DateTime<Utc>>) -> Self {
        Chat {
            _id: Some(Uuid::new()), sender, content, date: date.unwrap_or(Utc::now())
        }
    }
}

pub async fn send_chat(header: HeaderMap) -> String {
    let sender = header.get("sender").unwrap().to_str().unwrap().to_string();
    let content = header.get("content").unwrap().to_str().unwrap().to_string();
    let name = header.get("servername").unwrap().to_str().unwrap().to_string();
    let chat = Chat::new(sender, content, None);
    let db = crate::common::get_db().await;
    db.collection::<Chat>("chats")
        .insert_one(&chat, None)
        .await
        .unwrap();
    let mut serv = db.collection::<Server>("servers")
        .find_one(doc! {
            "name": &name
        }, None)
        .await
        .unwrap()
        .unwrap();
    serv.message.insert(chat._id.unwrap().clone());
    db.collection::<Server>("servers")
    .update_one(doc! {
        "name": name
    }, doc! {
        "$set": {
            "message": serv.message.into_iter().map(|x| {
                std::str::from_utf8(&x.bytes()).unwrap().to_string()
            }).collect::<Vec<String>>()
        }
    }, None)
    .await
    .unwrap();
    serde_json::to_string(&chat).unwrap()
}

pub async fn info_chat(header: HeaderMap) -> String {
    let chat_id = header.get("chatid").unwrap().to_str().unwrap().to_string();
    let db = crate::common::get_db().await;
    let chat = db.collection::<Chat>("chats")
        .find_one(doc! {
            "_id": Uuid::parse_str(&chat_id).unwrap()
        }, None)
        .await
        .unwrap()
        .unwrap();
    serde_json::to_string(&chat).unwrap()
}
