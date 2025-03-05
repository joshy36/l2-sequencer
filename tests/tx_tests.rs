use alloy::primitives::{Address, Bytes, U256};
use dotenv::dotenv;
use sequencer::client::{ClientError, L2Client};
use sequencer::models::L2Transaction;
use std::env;
use std::str::FromStr;

async fn setup_client() -> Result<L2Client, Box<dyn std::error::Error>> {
    dotenv().ok();

    let endpoint = env::var("L2_ENDPOINT").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let auth_token = env::var("AUTH_TOKEN").map_err(|_| "AUTH_TOKEN must be set in environment")?;

    Ok(L2Client::new(endpoint, auth_token))
}

#[tokio::test]
async fn test_basic_transaction_flow() -> Result<(), Box<dyn std::error::Error>> {
    let client = setup_client().await.map_err(|e| {
        eprintln!("Failed to setup client: {}", e);
        e
    })?;

    let tx = L2Transaction::new(
        0,
        Address::from_str("0x1111111111111111111111111111111111111111")?,
        Some(Address::from_str(
            "0x2222222222222222222222222222222222222222",
        )?),
        U256::from_str("1000000000000000000")?,
        Bytes::from_str("0x68656c6c6f")?,
        21000,
        U256::from_str("30000000000")?,
        Some(42161),
        0,
        U256::from_str("1000000")?,
    );

    match client.send_transaction(tx).await {
        Ok(_) => println!("Transaction sent successfully"),
        Err(ClientError::ServerError { status, body }) => {
            eprintln!("Server rejected transaction ({}): {}", status, body);
            return Err(format!("Transaction rejected: {}", body).into());
        }
        Err(e) => {
            eprintln!("Failed to send transaction: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_multiple_transactions() -> Result<(), Box<dyn std::error::Error>> {
    let client = setup_client().await?;

    for i in 0..5 {
        let tx = L2Transaction::new(
            i as u64,
            Address::from_str("0x1111111111111111111111111111111111111111")?,
            Some(Address::from_str(
                "0x2222222222222222222222222222222222222222",
            )?),
            U256::from_str("100000000000000000")?,
            Bytes::from_str("0x68656c6c6f")?,
            21000,
            U256::from_str("30000000000")?,
            Some(42161),
            0,
            U256::from_str("1000000")?,
        );

        match client.send_transaction(tx).await {
            Ok(_) => println!("Transaction {} sent successfully", i),
            Err(ClientError::ServerError { status, body }) => {
                eprintln!("Server rejected transaction {} ({}): {}", i, status, body);
                return Err(format!("Transaction {} rejected: {}", i, body).into());
            }
            Err(e) => {
                eprintln!("Failed to send transaction {}: {}", i, e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_invalid_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let client = setup_client().await?;

    let tx = L2Transaction::new(
        0,
        Address::from_str("0x1111111111111111111111111111111111111111")?,
        Some(Address::from_str(
            "0x2222222222222222222222222222222222222222",
        )?),
        U256::from_str("999999999999999999999999999999999999999")?,
        Bytes::from_str("0x68656c6c6f")?,
        21000,
        U256::from_str("30000000000")?,
        Some(42161),
        0,
        U256::from_str("1000000")?,
    );

    let result = client.send_transaction(tx).await;
    match result {
        Err(ClientError::ServerError { status, body }) => {
            println!(
                "Expected error received - status: {}, body: {}",
                status, body
            );
            Ok(())
        }
        Ok(_) => Err("Expected transaction to fail but it succeeded".into()),
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            Err(e.into())
        }
    }
}
