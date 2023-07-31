use crate::user::User;
use chrono::prelude::{DateTime, Utc};

pub struct Chat {
    sender: User,
    content: String,
    date: DateTime<Utc>
}

impl Chat {
    pub fn new(sender: User, content: String, date: Option<DateTime<Utc>>) -> Self {
        Chat {
            sender, content, date: date.unwrap_or(Utc::now())
        }
    }
}
