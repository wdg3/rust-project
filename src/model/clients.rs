use axum::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands, RedisError};
use std::sync::Arc;

pub type DynDBClient = Arc<dyn DBClient + Send + Sync>;

#[async_trait]
pub trait DBClient {
    async fn read(&self, key: String) -> Result<String, RedisError>;

    async fn write(&self, key: String, val: String) -> Result<(), RedisError>;
}

#[async_trait]
impl DBClient for ConnectionManager {
    async fn read(&self, key: String) -> Result<String, RedisError> {
        self.clone().get(key).await
    }

    async fn write(&self, key: String, val: String) -> Result<(), RedisError> {
        self.clone().set(key, val).await
    }
}