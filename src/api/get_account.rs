use axum::extract::Json;
use axum::http::StatusCode;
use serde::Deserialize;
use tracing::{instrument, error, info};
use uuid::Uuid;

use crate::model::account::Account;
use crate::service::redis::redis_service_adapter::{self, GetAccountDLInput};

#[derive(Deserialize, Debug)]
pub struct GetAccountRequest {
    account_id: Uuid,
}

#[instrument(name = "GetAccount")]
pub async fn get_account(
    Json(request): Json<GetAccountRequest>
) -> Result<Json<Account>, StatusCode> {
    info!("{:?}", request);
    let input = GetAccountDLInput { account_id: &request.account_id };
    let output = redis_service_adapter::get_account(input);

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