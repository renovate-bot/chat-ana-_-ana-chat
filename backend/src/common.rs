use mongodb::{Collection, error};
use async_trait::async_trait;

#[async_trait]
pub trait DuplicateChecker: Send {
    async fn is_duplicate<T>(&self, collec: Collection<T>) -> error::Result<bool>
    where
        T: Send + Sync + Unpin + 'static;
}