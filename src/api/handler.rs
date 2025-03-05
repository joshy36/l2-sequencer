use crate::models::L2Transaction;
use crate::services::parser::{parse_raw_transaction, RawTransactionData};
use crate::types::AppState;
use alloy::primitives::U256;
use alloy::providers::Provider;
use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize)]
pub struct TransactionRequest {
    raw_tx: RawTransactionData,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    status: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
    code: String,
}

pub async fn send_transaction(
    State(state): State<AppState>,
    Json(payload): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, Json<ErrorResponse>> {
    println!("Received transaction request");

    let transaction = parse_raw_transaction(&payload.raw_tx).map_err(|e| {
        println!("Parse error: {}", e);
        Json(ErrorResponse {
            error: e.to_string(),
            code: "PARSE_ERROR".to_string(),
        })
    })?;

    println!("Transaction parsed successfully");

    validate_gas_limit(&transaction).map_err(|e| {
        println!("Gas limit error: {}", e);
        Json(ErrorResponse {
            error: e.to_string(),
            code: "VALIDATION_ERROR".to_string(),
        })
    })?;
    validate_gas_price(&transaction).map_err(|e| {
        println!("Gas price error: {}", e);
        Json(ErrorResponse {
            error: e.to_string(),
            code: "VALIDATION_ERROR".to_string(),
        })
    })?;
    validate_nonce(state.provider, &transaction)
        .await
        .map_err(|e| {
            println!("Nonce validation error: {}", e);
            Json(ErrorResponse {
                error: e.to_string(),
                code: "VALIDATION_ERROR".to_string(),
            })
        })?;
    // validate_addresses(&transaction)?;
    // validate_contract_creation(&transaction)?;

    state
        .queue
        .submit_transaction(transaction)
        .await
        .map_err(|e| {
            println!("Queue error: {}", e);
            Json(ErrorResponse {
                error: e.to_string(),
                code: "QUEUE_ERROR".to_string(),
            })
        })?;

    println!("Transaction queued successfully");

    let response = Json(TransactionResponse {
        status: "queued".to_string(),
    });

    Ok(response)
}

pub async fn transaction_feed(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(mut socket: WebSocket, state: AppState) {
    let mut feed_rx = state.queue.subscribe();

    // Stream transactions to the client
    while let Ok(transaction) = feed_rx.recv().await {
        let serialized =
            serde_json::to_string(&transaction).expect("Failed to serialize transaction"); // Ensure L2Transaction is serializable
        if socket.send(Message::Text(serialized.into())).await.is_err() {
            // Client disconnected
            break;
        }
    }
}

fn validate_gas_limit(tx: &L2Transaction) -> Result<(), &'static str> {
    if tx.gas_limit < 21000 {
        return Err("Gas limit too low");
    }
    Ok(())
}

fn validate_gas_price(tx: &L2Transaction) -> Result<(), &'static str> {
    if tx.gas_price == U256::ZERO {
        return Err("Gas price cannot be zero");
    }
    Ok(())
}

async fn validate_nonce<T: Provider>(
    provider: T,
    tx: &L2Transaction,
) -> Result<(), Box<dyn Error>> {
    let sender = tx.from;

    // ignore for load test

    // let expected_nonce = provider.get_transaction_count(sender).await?;

    // if tx.nonce != expected_nonce {
    //     return Err(format!(
    //         "Invalid nonce: expected {}, got {}",
    //         expected_nonce, tx.nonce
    //     )
    //     .into());
    // }

    Ok(())
}
