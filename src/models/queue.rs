use crate::models::L2Transaction;
use alloy::providers::Provider;

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

    pub fn queue_transaction(&mut self, tx: &L2Transaction) {
        self.transactions.push(tx.clone());
    }

    pub fn print_queue_state(&self) {
        println!("Queue state: {} transactions", self.transactions.len());
        for (i, tx) in self.transactions.iter().enumerate() {
            println!("  {}: from {:?}", i + 1, tx.from);
        }
    }
}
