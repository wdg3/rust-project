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
pub struct WithdrawRequest {
    account_id: Uuid,
    amount: u128,
    destination: String,
    signature: String,
}

pub async fn withdraw(
    Json(request): Json<WithdrawRequest>,
    Extension(db_client): Extension<DynDBClient>
) -> Result<Json<Transaction>, StatusCode> {
    info!("{:?}", request);

    let account_id = request.account_id;
    let tx_id = get_tx_id();
    let amount = request.amount;
    let source = request.destination;
    let signature = request.signature;
    
    let transaction = TransactionBuilder::default()
        .tx_id(tx_id)
        .tx_type(TransactionType::Withdrawal)
        .amount(amount)
        .source(
            TransactionPartyBuilder::default()
                .party_type(TransactionPartyType::Internal)
                .id(account_id.to_string())
                .build()
                .unwrap()
        )
        .destination(
            TransactionPartyBuilder::default()
                .party_type(TransactionPartyType::External)
                .id(source)
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