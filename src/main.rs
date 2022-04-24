mod api;
mod error;
mod model;
mod service;
mod validation;
use api::{
    create_account::create_account,
    get_account::get_account,
    deposit::deposit,
    withdraw::withdraw,
    transfer::transfer,
};

use axum::{
    extract::Extension,
    routing::post,
    Router,
};
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let db_client = Arc::new(
        redis::Client::open("redis://127.0.0.1/")
        .unwrap()
        .get_tokio_connection_manager()
        .await
        .unwrap()) as model::clients::DynDBClient;

    let app = Router::new()
        .route("/accounts/create", post(create_account))
        .route("/accounts/get", post(get_account))
        .route("/deposit", post(deposit))
        .route("/withdraw", post(withdraw))
        .route("/transfer", post(transfer))
        .layer(Extension(db_client));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}