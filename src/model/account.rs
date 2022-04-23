use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Builder, Debug, Deserialize, Serialize)]
pub struct Account {
    pub account_id: Uuid,
    pub username: String,
    pub public_key: String,
    pub balance: u128,
}