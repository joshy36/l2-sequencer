use alloy::providers::ProviderBuilder;
use axum::middleware;
use axum::{
    routing::{get, post},
    Router,
};
use sequencer::api::auth::auth_middleware;
use sequencer::api::cors::create_cors_middleware;
use sequencer::api::handler::{send_transaction, transaction_feed};
use sequencer::services::queue_service::setup_queue;
use sequencer::types::AppState;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let rpc_url = env::var("RPC_URL")
        .unwrap_or_else(|_| "https://eth.merkle.io".to_string())
        .parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let queue_provider = provider.clone();
    let (queue_handle, mut processor) = setup_queue(queue_provider);

    tokio::spawn(async move {
        processor.run().await;
    });

    let state = AppState {
        queue: queue_handle,
        provider,
    };

    let app = Router::new()
        .route("/send_transaction", post(send_transaction))
        .route("/transaction_feed", get(transaction_feed))
        .layer(middleware::from_fn(auth_middleware))
        .layer(create_cors_middleware())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    println!("Server running on http://0.0.0.0:3001");
    axum::serve(listener, app).await?;

    Ok(())
}
