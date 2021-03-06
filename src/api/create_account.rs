use axum::extract::{Extension, Json};
use axum::http::StatusCode;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

use crate::model::account::{Account, AccountBuilder};
use crate::model::clients::DynDBClient;
use crate::service::redis::redis_service_adapter::{self, CreateAccountDLInput};

#[derive(Deserialize, Debug)]
pub struct CreateAccountRequest {
    username: String,
    public_key: String,
}

pub async fn create_account(
    Json(request): Json<CreateAccountRequest>,
    Extension(db_client): Extension<DynDBClient>
    ) -> Result<Json<Account>, StatusCode> {
        info!("{:?}", request);
        let account_id: Uuid = get_account_id();
        let username: String = request.username;
        let public_key: String = request.public_key;
        let balance: u128 = 0;

        let account = AccountBuilder::default()
            .account_id(account_id)
            .username(username)
            .public_key(public_key)
            .balance(balance)
            .build()
            .unwrap();
        
        let input = CreateAccountDLInput { account: &account };
        let output = redis_service_adapter::create_account(input, db_client).await;

        match output.result {
            Ok(_) => {
                info!("{:?}", account);
                Ok(Json(account))
            },
            Err(code) => {
                error!("{:?}", code);
                Err(code)
            }
        }
}

fn get_account_id() -> Uuid {
    Uuid::new_v4()
}