use alloy::consensus::Transaction;
use alloy::eips::{eip2930::AccessList, eip7702::SignedAuthorization, Typed2718};
use alloy::primitives::{Address, Bytes, ChainId, TxKind, B256, U256};

#[derive(Debug, Clone)]
pub struct L2Transaction {
    // Core fields
    pub nonce: u64,
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    pub data: Bytes,
    pub gas_limit: u64,
    pub gas_price: U256,

    // Additional fields needed for L2
    pub chain_id: Option<ChainId>,
    pub l1_block_number: u64,
    pub submission_fee: U256,
}

impl L2Transaction {
    pub fn new(
        nonce: u64,
        from: Address,
        to: Option<Address>,
        value: U256,
        data: Bytes,
        gas_limit: u64,
        gas_price: U256,
        chain_id: Option<ChainId>,
        l1_block_number: u64,
        submission_fee: U256,
    ) -> Self {
        Self {
            nonce,
            from,
            to,
            value,
            data,
            gas_limit,
            gas_price,
            chain_id,
            l1_block_number,
            submission_fee,
        }
    }
}

impl Typed2718 for L2Transaction {
    fn ty(&self) -> u8 {
        // Custom type for Arbitrum transactions
        // Using a number higher than standard Ethereum types (0-4)
        // but still within EIP-2718 limit of 0x7f
        0x64
    }
}

impl Transaction for L2Transaction {
    fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }

    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn gas_limit(&self) -> u64 {
        self.gas_limit
    }

    fn gas_price(&self) -> Option<u128> {
        Some(self.gas_price.try_into().unwrap_or(u128::MAX))
    }

    fn max_fee_per_gas(&self) -> u128 {
        self.gas_price.try_into().unwrap_or(u128::MAX)
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        None // L2 might handle fees differently
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        None // Unless implementing blob transactions on L2
    }

    fn priority_fee_or_price(&self) -> u128 {
        self.gas_price.try_into().unwrap_or(u128::MAX)
    }

    fn effective_gas_price(&self, _base_fee: Option<u64>) -> u128 {
        self.gas_price.try_into().unwrap_or(u128::MAX)
    }

    fn is_dynamic_fee(&self) -> bool {
        false // Unless implementing EIP-1559 style fees
    }

    fn kind(&self) -> TxKind {
        match self.to {
            Some(to) => TxKind::Call(to),
            None => TxKind::Create,
        }
    }

    fn is_create(&self) -> bool {
        self.to.is_none()
    }

    fn value(&self) -> U256 {
        self.value
    }

    fn input(&self) -> &Bytes {
        &self.data
    }

    fn access_list(&self) -> Option<&AccessList> {
        None // Unless implementing EIP-2930
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        None // Unless implementing EIP-4844
    }

    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        None // Unless implementing EIP-7702
    }
}
