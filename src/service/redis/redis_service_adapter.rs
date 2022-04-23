use axum::http::StatusCode;
use serde_json;
use tracing::error;
use uuid::Uuid;

use crate::service::redis::redis_service::{
    create_account_call,
    get_account_call,
    create_transaction_call,
};
use crate::model::account::Account;
use crate::model::transaction::Transaction;

pub struct CreateAccountDLInput<'a> {
    pub account: &'a Account,
}

pub struct CreateAccountDLOutput {
    pub result: Result<(), StatusCode>,
}

pub struct GetAccountDLInput<'a> {
    pub account_id: &'a Uuid,
}

pub struct GetAccountDLOutput {
    pub result: Result<Account, StatusCode>,
}

pub struct CreateTransactionDLInput<'a> {
    pub transaction: &'a Transaction,
}

pub struct CreateTransactionDLOutput {
    pub result: Result<(), StatusCode>,
}

pub fn create_account(input: CreateAccountDLInput) -> CreateAccountDLOutput {
    let key: String = format!("account:{}", input.account.account_id);
    let val: String = serde_json::to_string(&input.account).unwrap();

    let result = create_account_call(key, val);
    match result {
        Ok(_) => CreateAccountDLOutput { result: Ok(()) },
        Err(_) => CreateAccountDLOutput { result: Err(StatusCode::FAILED_DEPENDENCY) },
    }
}

pub fn get_account(input: GetAccountDLInput) -> GetAccountDLOutput {
    let key: String = format!("account:{}", input.account_id);

    let result = get_account_call(key);
    match result {
        Ok(s) => {
            let account = serde_json::from_str(&s).unwrap();
            GetAccountDLOutput { result: Ok(account) }
        },
        Err(_) => GetAccountDLOutput { result: Err(StatusCode::FAILED_DEPENDENCY) },
    }
}

pub fn create_transaction(input: CreateTransactionDLInput) -> CreateTransactionDLOutput {
    let key: String = format!("transaction:{}", input.transaction.tx_id);
    let val: String = serde_json::to_string(&input.transaction).unwrap();

    let result = create_transaction_call(key, val);
    match result {
        Ok(_) => CreateTransactionDLOutput { result: Ok(()) },
        Err(err) => {
            error!("{:?}", err);
            CreateTransactionDLOutput { result: Err(StatusCode::FAILED_DEPENDENCY) }
        },
    }
}