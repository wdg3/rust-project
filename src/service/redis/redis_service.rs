use redis::{Commands, RedisResult};
use tracing::info;

pub fn create_account_call(key: String, val: String) -> RedisResult<()> {
    info!("Creating account {} at {}", val, key);

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let response = con.set(key, val);
    info!("Retrieved {:?}", &response);
    response
}

pub fn get_account_call(key: String) -> RedisResult<String> {
    info!("Retrieving account from {}", key);
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let response = con.get(key);
    info!("Retrieved {:?}", &response);
    response
}

pub fn create_transaction_call(key: String, val: String) -> RedisResult<()> {
    info!("Recording transaction {} at {}", val, key);
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let response = con.set(key, val);
    info!("Retrieved {:?}", &response);
    response
}