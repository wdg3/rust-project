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
    routing::post,
    Router,
};
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let app = Router::new()
        .route("/accounts/create", post(create_account))
        .route("/accounts/get", post(get_account))
        .route("/deposit", post(deposit))
        .route("/withdraw", post(withdraw))
        .route("/transfer", post(transfer));

    let redis_manager =
        redis::Client::open("redis://127.0.0.1/")
            .unwrap()
            .get_tokio_connection_manager()
            .await
            .unwrap();

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}