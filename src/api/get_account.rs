use axum::Extension;
use axum::extract::Json;
use axum::http::StatusCode;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

use crate::model::account::Account;
use crate::model::clients::DynDBClient;
use crate::service::redis::redis_service_adapter::{self, GetAccountDLInput};

#[derive(Deserialize, Debug)]
pub struct GetAccountRequest {
    account_id: Uuid,
}

pub async fn get_account(
    Json(request): Json<GetAccountRequest>,
    Extension(db_client): Extension<DynDBClient>
) -> Result<Json<Account>, StatusCode> {
    info!("{:?}", request);
    let input = GetAccountDLInput { account_id: &request.account_id };
    let output = redis_service_adapter::get_account(input, db_client).await;

    match output.result {
        Ok(account) => {
            info!("{:?}", account);
            Ok(Json(account))
        },
        Err(code) => {
            error!("{:?}", code);
            Err(code)
        }
    }
}