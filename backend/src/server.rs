use crate::{common::DuplicateChecker, user::User, msg::Chat};
use mongodb::{error, bson::doc, Database};
use async_trait::async_trait;

struct Server {
    name: String,
    member: Vec<User>,
    channels: Vec<Channel>
}

struct Channel {
    name: String,
    chats: Vec<Chat>,
    parent_server_name: String
}

impl Server {
    fn new(name: String, member: Option<Vec<User>>, channels: Option<Vec<Channel>>) -> Self {
        Self {
            name,
            member: member.unwrap_or(Vec::new()),
            channels: channels.unwrap_or(Vec::new())
        }
    }
}

impl Channel {
    fn new(name: String, chats: Vec<Chat>, parent_server: Server) -> Self {
        Self {
            name,
            chats,
            parent_server_name: parent_server.name
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

#[async_trait]
impl DuplicateChecker for Channel {
    async fn is_duplicate(&self, db: &Database) -> error::Result<bool> {
        let data = db.collection::<Channel>("channels")
            .count_documents(doc! {
                "name": &self.name,
                "server": &self.parent_server_name
            }, None)
            .await?;
        if data > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
