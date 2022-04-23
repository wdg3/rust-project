use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Builder, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub tx_id: Uuid,
    pub tx_type: TransactionType,
    pub amount: u128,
    pub source: TransactionParty,
    pub destination: TransactionParty,
    pub signature: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
pub struct TransactionParty {
    pub party_type: TransactionPartyType,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TransactionPartyType {
    Internal,
    External,
}