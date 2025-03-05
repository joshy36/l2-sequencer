use crate::models::L2Transaction;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct L2Client {
    client: Client,
    endpoint: String,
    auth_token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid response: {0}")]
    ResponseError(String),
    #[error("Server error: {status}, body: {body}")]
    ServerError { status: u16, body: String },
}

impl L2Client {
    pub fn new(endpoint: String, auth_token: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            endpoint,
            auth_token,
        }
    }

    pub async fn send_transaction(&self, tx: L2Transaction) -> Result<(), ClientError> {
        let tx_json = json!({
            "raw_tx": {
                "nonce": format!("0x{:x}", tx.nonce),
                "from": format!("{:#x}", tx.from),
                "to": tx.to.map(|addr| format!("{:#x}", addr)),
                "value": format!("0x{:x}", tx.value),
                "data": format!("0x{}", hex::encode(&tx.data)),
                "gas_limit": format!("0x{:x}", tx.gas_limit),
                "gas_price": format!("0x{:x}", tx.gas_price),
                "chain_id": tx.chain_id.unwrap_or(42161),
                "l1_block_number": tx.l1_block_number,
                "submission_fee": format!("0x{:x}", tx.submission_fee)
            }
        });

        let response = self
            .client
            .post(&format!("{}/send_transaction", self.endpoint))
            .bearer_auth(&self.auth_token)
            .json(&tx_json)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Failed to send request: {}", e);
                ClientError::RequestError(e)
            })?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "No response body".into());
            eprintln!(
                "Request body: {}",
                serde_json::to_string_pretty(&tx_json).unwrap()
            );
            eprintln!("Response status: {}", status);
            eprintln!("Response body: {}", body);

            return Err(ClientError::ServerError { status, body });
        }

        Ok(())
    }
}
