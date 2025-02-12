use alloy::primitives::{Address, ChainId, U256};
use alloy::providers::ProviderBuilder;
use l2_sequencer::{L2Transaction, Queue};
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let mut queue = Queue::new(provider);

    let transaction = L2Transaction::new(
        0, // nonce: typical transaction count for an account
        Address::from([0x42; 20]),
        Some(Address::from([0x23; 20])), // For contract creation, this would be None
        U256::from(1_000_000_000_000_000_000u64), // value: 1 ETH in wei
        vec![0x68, 0x65, 0x6c, 0x6c, 0x6f].into(), // data: "hello" in hex, converted to Bytes
        21_000,                          // gas_limit: standard ETH transfer gas limit
        U256::from(30_000_000_000u64),   // gas_price: 30 gwei
        Some(ChainId::from(42161u64)),   // chain_id: Arbitrum One
        0,                               // l1_block_number: starting from 0 or get from provider
        U256::from(1_000_000u64),        // submission_fee: example fee
    );

    match queue.process_transaction(transaction.clone()).await {
        Ok(()) => println!("Transaction processed successfully!"),
        Err(e) => println!("Transaction processing failed: {}", e),
    }

    queue.print_queue_state();

    Ok(())
}
