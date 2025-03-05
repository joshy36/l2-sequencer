use crate::models::L2Transaction;
use alloy::providers::Provider;
use bincode;
use brotli::CompressorWriter;
use std::io::Write;

pub struct Queue<T: Provider> {
    provider: T,
    transactions: Vec<L2Transaction>,
    batch_size: usize,
}

impl<T: Provider> Queue<T> {
    pub fn new(provider: T) -> Self {
        Self {
            provider,
            transactions: Vec::new(),
            batch_size: 5,
        }
    }

    pub fn queue_transaction(&mut self, tx: &L2Transaction) {
        self.transactions.push(tx.clone());
        if self.transactions.len() >= self.batch_size {
            let _ = self.batch_transactions();
        }
    }

    pub fn batch_transactions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.transactions.is_empty() {
            return Ok(());
        }

        let batch_size = std::cmp::min(self.batch_size, self.transactions.len());
        let batch: Vec<L2Transaction> = self.transactions.drain(0..batch_size).collect();

        println!("Batched transactions:");
        for (i, tx) in batch.iter().enumerate() {
            println!("  {}: from {:?}", i + 1, tx.from);
        }

        self.compress_batch(&batch)?;
        Ok(())
    }

    pub fn compress_batch(
        &mut self,
        batch: &Vec<L2Transaction>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut output = Vec::new();
        {
            let mut compressor = CompressorWriter::new(&mut output, 4096, 3, 22);
            let bytes = bincode::serialize(batch)?;
            // let bytes = postcard::to_allocvec(batch)?;
            println!("pre-compression: {} bytes", bytes.len());
            println!("Bytes: {:?}", bytes);
            compressor.write_all(&bytes)?;
            compressor.flush()?;
        }

        println!("post-compression: {} bytes", output.len());

        Ok(output)
    }

    pub fn print_queue_state(&self) {
        println!("Queue state: {} transactions", self.transactions.len());
        for (i, tx) in self.transactions.iter().enumerate() {
            println!("  {}: from {:?}", i + 1, tx.from);
        }
    }
}
