use axum::http::{HeaderMap, StatusCode};
use chrono::prelude::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use crate::{server::Server, common::get_header_string};

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    _id: ObjectId,
    sender: String,
    content: String,
    date: DateTime<Utc>
}

impl Chat {
    pub fn new(sender: String, content: String) -> Self {
        Chat {
            _id: ObjectId::new(), sender, content, date: Utc::now()
        }
    }
}

pub async fn send_chat(header: HeaderMap) -> Result<String, StatusCode> {
    let sender = get_header_string(&header, "sender")?;
    let content = get_header_string(&header, "content")?;
    let name = get_header_string(&header, "servername")?;
    let chat = Chat::new(sender, content);
    let db = crate::common::get_db().await;
    db.collection::<Chat>("chats")
        .insert_one(&chat, None)
        .await
        .unwrap();
    let mut serv = match db.collection::<Server>("servers")
        .find_one(doc! {
            "name": &name
        }, None)
        .await
        .unwrap() {
            Some(a) => a,
            None => return Err(StatusCode::NOT_FOUND)
        };
    serv.message.insert(chat._id);
    db.collection::<Server>("servers")
    .update_one(doc! {
        "name": name
    }, doc! {
        "$set": {
            "message": serv.message.into_iter().collect::<Vec<ObjectId>>()
        }
    }, None)
    .await
    .unwrap();
    Ok(serde_json::to_string(&chat).unwrap())
}

pub async fn info_chat(header: HeaderMap) -> Result<String, StatusCode> {
    let chat_id = get_header_string(&header, "chatid")?;
    let db = crate::common::get_db().await;
    let chat = db.collection::<Chat>("chats")
        .find_one(doc! {
            "_id": ObjectId::parse_str(&chat_id).unwrap()
        }, None)
        .await
        .unwrap();
    match chat {
        Some(a) => Ok(serde_json::to_string(&a).unwrap()),
        None => Err(StatusCode::NOT_FOUND)
    }
}
