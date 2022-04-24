use redis::{RedisResult};
use tracing::info;

use crate::model::clients::DynDBClient;

pub async fn create_account_call(key: String, val: String, client: DynDBClient) -> RedisResult<()> {
    info!("Creating account {} at {}", val, key);

    let response = client.write(key, val).await;
    info!("Retrieved {:?}", &response);
    response
}

pub async fn get_account_call(key: String, client: DynDBClient) -> RedisResult<String> {
    info!("Retrieving account from {}", key);

    let response = client.read(key).await;
    info!("Retrieved {:?}", &response);
    response
}

pub async fn create_transaction_call(key: String, val: String, client: DynDBClient) -> RedisResult<()> {
    info!("Recording transaction {} at {}", val, key);
    let response = client.write(key, val).await;
    info!("Retrieved {:?}", &response);
    response
}