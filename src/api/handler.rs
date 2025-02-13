use crate::models::L2Transaction;
use crate::services::parser::{parse_raw_transaction, RawTransactionData};
use crate::types::AppState;
use alloy::primitives::U256;
use alloy::providers::Provider;
use axum::{extract::State, Json};
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
                code: "PARSE_ERROR".to_string(),
            })
        })?;

    println!("Transaction queued successfully");

    let response = Json(TransactionResponse {
        status: "queued".to_string(),
    });

    Ok(response)
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

    let expected_nonce = provider.get_transaction_count(sender).await?;

    if tx.nonce != expected_nonce {
        return Err(format!(
            "Invalid nonce: expected {}, got {}",
            expected_nonce, tx.nonce
        )
        .into());
    }

    Ok(())
}
