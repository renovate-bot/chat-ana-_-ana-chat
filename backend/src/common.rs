use mongodb::{Database, error};
use async_trait::async_trait;

#[async_trait]
pub trait DuplicateChecker: Send {
    async fn is_duplicate(&self, db: &Database) -> error::Result<bool>;
}