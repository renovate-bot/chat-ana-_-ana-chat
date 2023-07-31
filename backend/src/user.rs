use async_trait::async_trait;
use mongodb::{Collection, bson::doc, error};
use crate::common::DuplicateChecker;

pub enum Status {
    Online,
    Afk,
    NoDisturb,
    Offline
}

pub struct User {
    nickname: String,
    email: String,
    profile_image: String,
    status: Status,
    friends: Vec<User>
}

#[async_trait]
impl DuplicateChecker for User {
    async fn is_duplicate<T>(&self, collec: Collection<T>) -> error::Result<bool>
    where
        T: Send + Sync + Unpin + 'static,
    {
        let data = collec
            .count_documents(doc! {
                "email": self.email.clone()
            }, None)
            .await?;
        if data > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl User {
    fn new(nickname: String, email: String, profile_image: String) -> Self {
        Self {
            nickname,
            email,
            profile_image,
            status: Status::Online
        }
    }
}