use axum::Extension;
use axum::extract::Json;
use axum::http::StatusCode;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

use crate::model::clients::DynDBClient;
use crate::model::transaction::{
    Transaction,
    TransactionBuilder,
    TransactionType,
    TransactionPartyBuilder,
    TransactionPartyType,
};

use crate::service::redis::redis_service_adapter::{self, CreateTransactionDLInput};
use crate::validation::validation::validate;

#[derive(Deserialize, Debug)]
pub struct TransferRequest {
    source_account_id: Uuid,
    destination_account_id: Uuid,
    amount: u128,
    signature: String,
}

pub async fn transfer(
    Json(request): Json<TransferRequest>,
    Extension(db_client): Extension<DynDBClient>
) -> Result<Json<Transaction>, StatusCode> {
    info!("{:?}", request);

    let source_acct_id = request.source_account_id;
    let dest_acct_id = request.destination_account_id;
    let tx_id = get_tx_id();
    let amount = request.amount;
    let signature = request.signature;
    
    let transaction = TransactionBuilder::default()
        .tx_id(tx_id)
        .tx_type(TransactionType::Withdrawal)
        .amount(amount)
        .source(
            TransactionPartyBuilder::default()
                .party_type(TransactionPartyType::Internal)
                .id(source_acct_id.to_string())
                .build()
                .unwrap()
        )
        .destination(
            TransactionPartyBuilder::default()
                .party_type(TransactionPartyType::Internal)
                .id(dest_acct_id.to_string())
                .build()
                .unwrap()
        )
        .signature(signature)
        .build()
        .unwrap();

    let validation_status = validate(&transaction, db_client.clone()).await;
    if validation_status.is_err() {
        return Err(validation_status.err().unwrap())
    }

    let input = CreateTransactionDLInput { transaction: &transaction };
    let output = redis_service_adapter::create_transaction(input, db_client).await;

    match output.result {
        Ok(_) => {
            info!("{:?}", transaction);
            Ok(Json(transaction))
        },
        Err(code) => {
            error!("{:?}", code);
            Err(code)
        }
    }
}

fn get_tx_id() -> Uuid {
    Uuid::new_v4()
}