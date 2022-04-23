use axum::http::StatusCode;
use uuid::Uuid;

use crate::model::account::Account;
use crate::model::transaction::{Transaction, TransactionType};
use crate::service::redis::redis_service_adapter::{self, GetAccountDLInput};

pub fn validate(transaction: &Transaction) -> Result<(), StatusCode> {
    match transaction.tx_type {
        TransactionType::Deposit => validate_deposit(transaction),
        TransactionType::Withdrawal => validate_withdrawal(transaction),
        TransactionType::Transfer => validate_transfer(transaction),
    }
}

fn validate_deposit(transaction: &Transaction) -> Result<(), StatusCode> {
    let account_id = Uuid::parse_str(&transaction.destination.id).unwrap();
    let signature = &transaction.signature;

    let input = GetAccountDLInput { account_id: &account_id };
    let output = redis_service_adapter::get_account(input);

    match output.result {
        Ok(_) => {
            if !valid_signature(signature, output.result.unwrap()) {
                Err(StatusCode::UNAUTHORIZED)
            } else {
                Ok(())
            }
        },
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

fn validate_withdrawal(transaction: &Transaction) -> Result<(), StatusCode> {
    let account_id = Uuid::parse_str(&transaction.source.id).unwrap();
    let signature = &transaction.signature;

    let input = GetAccountDLInput { account_id: &account_id };
    let output = redis_service_adapter::get_account(input);

    match output.result {
        Err(_) => {
            Err(StatusCode::NOT_FOUND)
        },
        Ok(account) => {
            if account.balance < transaction.amount {
                Err(StatusCode::BAD_REQUEST)
            } else if !valid_signature(signature, account) {
                Err(StatusCode::UNAUTHORIZED)
            } else {
                Ok(())
            }
        }
    }
}

fn validate_transfer(transaction: &Transaction) -> Result<(), StatusCode> {
    let source_id = Uuid::parse_str(&transaction.source.id).unwrap();
    let source_input = GetAccountDLInput { account_id: &source_id };

    let dest_id = Uuid::parse_str(&transaction.destination.id).unwrap();
    let dest_input = GetAccountDLInput { account_id: &dest_id };

    let source_output = redis_service_adapter::get_account(source_input);
    let dest_output = redis_service_adapter::get_account(dest_input);

    match (source_output.result, dest_output.result) {
        (Ok(source_acct), Ok(_)) => {
            if source_acct.balance < transaction.amount {
                return Err(StatusCode::BAD_REQUEST)
            } else {
                Ok(())
            }
        },
        _ => Err(StatusCode::NOT_FOUND)
    }
}

fn valid_signature(signature: &String, account: Account) -> bool {
    true
}