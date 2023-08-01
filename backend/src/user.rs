use async_trait::async_trait;
use axum::http::{HeaderMap, StatusCode};
use mongodb::{bson::doc, error, Database};
use crate::common::DuplicateChecker;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
    Online,
    Afk,
    NoDisturb,
    Offline
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    name: String,
    email: String,
    profile_image: String,
    status: Status,
    friends: Vec<User>
}

#[async_trait]
impl DuplicateChecker for User {
    async fn is_duplicate(&self, db: &Database) -> error::Result<bool> {
        let data = db.collection::<User>("users")
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
    fn new(name: String, email: String, profile_image: String, friends: Option<Vec<User>>) -> Self {
        Self {
            name,
            email,
            profile_image,
            status: Status::Online,
            friends: friends.unwrap_or(Vec::new())
        }
    }
}

pub struct UserEndpoint {
    db: Database
}

impl UserEndpoint {
    pub fn new(db: Database) -> Self {
        Self {
            db
        }
    }

    pub async fn login(&self, name: String, email: String, profile_image: String) -> error::Result<User> {
        let user = User::new(name, email, profile_image, None);
        if !user.is_duplicate(&self.db).await? {
            self.db.collection::<User>("users")
                .insert_one(user.clone(), None)
                .await?;
        }
        Ok(user)
    }
}

pub async fn login_end(header: HeaderMap) -> String {
    let name = header.get("name").unwrap().to_str().unwrap().to_string();
    let email = header.get("email").unwrap().to_str().unwrap().to_string();
    let profile_image = header.get("profile_image").unwrap().to_str().unwrap().to_string();
    let db = crate::common::get_db().await;
    let userendpoint = UserEndpoint::new(db);
    let user = userendpoint.login(name, email, profile_image).await.unwrap();
    serde_json::to_string(&user).unwrap()
}

pub async fn get_user_info(header: HeaderMap) -> Result<String, axum::http::StatusCode> {
    let email = header.get("email").unwrap().to_str().unwrap().to_string();
    let db = crate::common::get_db().await;
    let collec = db.collection::<User>("users");
    let user = collec.find_one(doc! {
        "email": email
    }, None).await.unwrap();
    match user {
        None => return Err(StatusCode::NOT_FOUND),
        Some(a) => Ok(serde_json::to_string(&a).unwrap())
    }
}
