use crate::{common::DuplicateChecker, user::User, msg::Chat};
use mongodb::{Collection, error, bson::doc};
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
    fn new(name: String, member: Vec<User>, channels: Vec<Channel>) -> Self {
        Self {
            name,
            member,
            channels
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
    async fn is_duplicate<T>(&self, collec: Collection<T>) -> error::Result<bool>
    where
        T: Send + Sync + Unpin + 'static,
    {
        let data = collec
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
    async fn is_duplicate<T>(&self, collec: Collection<T>) -> error::Result<bool>
    where
        T: Send + Sync + Unpin + 'static,
    {
        let data = collec
            .count_documents(doc! {
                "name": self.name.clone(),
                "server": self.parent_server_name.clone()
            }, None)
            .await?;
        if data > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
