use super::transaction::L2Transaction;
use alloy::primitives::B256;

#[derive(Debug)]
pub struct Block {
    pub transactions: Vec<L2Transaction>,
    pub parent_hash: B256,
    pub state_root: B256,
    pub timestamp: u64,
}

impl Block {
    pub fn new(
        transactions: Vec<L2Transaction>,
        parent_hash: B256,
        state_root: B256,
        timestamp: u64,
    ) -> Self {
        Self {
            transactions,
            parent_hash,
            state_root,
            timestamp,
        }
    }
}
