use async_trait::async_trait;
use axum::http::{HeaderMap, StatusCode};
use mongodb::{bson::doc, Database};
use crate::common::{DuplicateChecker, get_header_string};
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
    async fn is_duplicate(&self, db: &Database) -> Result<bool, StatusCode> {
        let data = db.collection::<User>("users")
            .count_documents(doc! {
                "email": self.email.clone()
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

impl User {
    fn new(name: String, email: String, profile_image: String, friends: Option<Vec<User>>) -> Self {
        Self {
            name,
            email,
            profile_image,
            status: Status::Online,
            friends: friends.unwrap_or_default()
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

    pub async fn login(&self, name: String, email: String, profile_image: String) -> Result<User, StatusCode> {
        let user = User::new(name, email, profile_image, None);
        if !user.is_duplicate(&self.db).await? {
            match self.db.collection::<User>("users")
                .insert_one(user.clone(), None)
                .await {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{:?}", e);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
        } else {
            return Err(StatusCode::CONFLICT);
        }
        Ok(user)
    }
}

pub async fn login_end(header: HeaderMap) -> Result<String, StatusCode> {
    let name = get_header_string(&header, "name")?;
    let email = get_header_string(&header, "email")?;
    let profile_image = get_header_string(&header, "profile_image")?;
    let db = crate::common::get_db().await;
    let userendpoint = UserEndpoint::new(db);
    let user = userendpoint.login(name, email, profile_image).await.unwrap();
    Ok(serde_json::to_string(&user).unwrap())
}

pub async fn get_user_info(header: HeaderMap) -> Result<String, StatusCode> {
    let email = get_header_string(&header, "email")?;
    let db = crate::common::get_db().await;
    let collec = db.collection::<User>("users");
    let user = collec.find_one(doc! {
        "email": email
    }, None).await.unwrap();
    match user {
        None => Err(StatusCode::NOT_FOUND),
        Some(a) => Ok(serde_json::to_string(&a).unwrap())
    }
}
