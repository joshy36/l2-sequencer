use crate::models::L2Transaction;
use alloy::primitives::U256;
use alloy::providers::Provider;
use std::error::Error;

pub struct Queue<T: Provider> {
    provider: T,
    transactions: Vec<L2Transaction>,
}

impl<T: Provider> Queue<T> {
    pub fn new(provider: T) -> Self {
        Self {
            provider,
            transactions: Vec::new(),
        }
    }

    pub async fn process_transaction(&mut self, tx: L2Transaction) -> Result<(), Box<dyn Error>> {
        self.validate_gas_limit(&tx)?;
        self.validate_gas_price(&tx)?;
        self.validate_nonce(&tx).await?;
        // validate_addresses(&tx)?;
        // validate_contract_creation(&tx)?;
        self.queue_transaction(&tx);
        Ok(())
    }

    pub fn queue_transaction(&mut self, tx: &L2Transaction) {
        self.transactions.push(tx.clone());
    }

    fn validate_gas_limit(&mut self, tx: &L2Transaction) -> Result<(), &'static str> {
        if tx.gas_limit < 21000 {
            return Err("Gas limit too low");
        }
        Ok(())
    }

    fn validate_gas_price(&mut self, tx: &L2Transaction) -> Result<(), &'static str> {
        if tx.gas_price == U256::ZERO {
            return Err("Gas price cannot be zero");
        }
        Ok(())
    }

    async fn validate_nonce(&mut self, tx: &L2Transaction) -> Result<(), Box<dyn Error>> {
        let sender = tx.from;

        let expected_nonce = self.provider.get_transaction_count(sender).await?;

        if tx.nonce != expected_nonce {
            return Err(format!(
                "Invalid nonce: expected {}, got {}",
                expected_nonce, tx.nonce
            )
            .into());
        }

        Ok(())
    }

    pub fn print_queue_state(&self) {
        println!("Queue state: {} transactions", self.transactions.len());
        for (i, tx) in self.transactions.iter().enumerate() {
            println!("  {}: from {:?}", i + 1, tx.from);
        }
    }
}
