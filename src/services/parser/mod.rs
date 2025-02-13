use crate::models::L2Transaction;
use alloy::{
    hex,
    primitives::{Address, Bytes, ChainId, U256},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawTransactionData {
    nonce: String,
    from: String,
    to: Option<String>,
    value: String,
    data: String,
    gas_limit: String,
    gas_price: String,
    chain_id: Option<u64>,
    l1_block_number: u64,
    submission_fee: String,
}

pub fn parse_raw_transaction(tx_data: &RawTransactionData) -> Result<L2Transaction, String> {
    let nonce = u64::from_str_radix(tx_data.nonce.trim_start_matches("0x"), 16)
        .map_err(|e| format!("Invalid nonce: {}", e))?;

    let from = parse_address(&tx_data.from)?;
    let to = match &tx_data.to {
        Some(addr) => Some(parse_address(addr)?),
        None => None,
    };

    let value = parse_u256(&tx_data.value)?;
    let gas_price = parse_u256(&tx_data.gas_price)?;
    let submission_fee = parse_u256(&tx_data.submission_fee)?;

    let data = parse_bytes(&tx_data.data)?;

    let gas_limit = u64::from_str_radix(tx_data.gas_limit.trim_start_matches("0x"), 16)
        .map_err(|e| format!("Invalid gas limit: {}", e))?;

    let chain_id = tx_data.chain_id.map(ChainId::from);

    Ok(L2Transaction::new(
        nonce,
        from,
        to,
        value,
        data,
        gas_limit,
        gas_price,
        chain_id,
        tx_data.l1_block_number,
        submission_fee,
    ))
}

fn parse_address(addr: &str) -> Result<Address, String> {
    let addr = addr.trim_start_matches("0x");
    if addr.len() != 40 {
        return Err("Invalid address length".to_string());
    }

    let bytes = hex::decode(addr).map_err(|e| format!("Invalid address hex: {}", e))?;

    Ok(Address::from_slice(&bytes))
}

fn parse_u256(hex_str: &str) -> Result<U256, String> {
    let hex_str = hex_str.trim_start_matches("0x");
    U256::from_str_radix(hex_str, 16).map_err(|e| format!("Invalid U256 value: {}", e))
}

fn parse_bytes(hex_str: &str) -> Result<Bytes, String> {
    let hex_str = hex_str.trim_start_matches("0x");
    hex::decode(hex_str)
        .map_err(|e| format!("Invalid hex data: {}", e))
        .map(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::consensus::Transaction;
    use alloy::primitives::TxKind;

    fn create_valid_tx_data() -> RawTransactionData {
        RawTransactionData {
            nonce: "0x1".to_string(),
            from: "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string(),
            to: Some("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string()),
            value: "0xde0b6b3a7640000".to_string(), // 1 ETH
            data: "0x".to_string(),
            gas_limit: "0x5208".to_string(),      // 21000
            gas_price: "0x4a817c800".to_string(), // 20 Gwei
            chain_id: Some(1),
            l1_block_number: 12345678,
            submission_fee: "0x2386f26fc10000".to_string(), // 0.01 ETH
        }
    }

    #[test]
    fn test_valid_transaction_parsing() {
        let tx_data = create_valid_tx_data();
        let result = parse_raw_transaction(&tx_data);
        assert!(result.is_ok());

        let tx = result.unwrap();
        assert_eq!(tx.nonce(), 1);
        assert_eq!(tx.gas_limit(), 21000);
        assert_eq!(tx.chain_id(), Some(ChainId::from(1u64)));
        assert_eq!(tx.l1_block_number, 12345678);

        // Test Transaction trait implementations
        assert!(!tx.is_dynamic_fee());
        assert_eq!(tx.gas_price().unwrap(), 20_000_000_000u128); // 20 Gwei
        assert_eq!(tx.max_fee_per_gas(), 20_000_000_000u128);
        assert!(tx.max_priority_fee_per_gas().is_none());
        assert!(tx.max_fee_per_blob_gas().is_none());
        assert!(tx.access_list().is_none());
        assert!(tx.blob_versioned_hashes().is_none());
        assert!(tx.authorization_list().is_none());

        // Test addresses
        assert_eq!(
            tx.from,
            parse_address("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap()
        );
        match tx.kind() {
            TxKind::Call(addr) => assert_eq!(
                addr,
                parse_address("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").unwrap()
            ),
            _ => panic!("Expected Call transaction"),
        }
    }

    #[test]
    fn test_contract_deployment() {
        let mut tx_data = create_valid_tx_data();
        tx_data.to = None;
        tx_data.data = "0x60806040".to_string(); // Some contract bytecode

        let result = parse_raw_transaction(&tx_data);
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert!(tx.is_create());
        match tx.kind() {
            TxKind::Create => (),
            _ => panic!("Expected Create transaction"),
        }
        assert_eq!(tx.input(), &Bytes::from(hex::decode("60806040").unwrap()));
    }

    #[test]
    fn test_l2_specific_fields() {
        let tx_data = create_valid_tx_data();
        let tx = parse_raw_transaction(&tx_data).unwrap();

        assert_eq!(tx.l1_block_number, 12345678);
        assert_eq!(
            tx.submission_fee,
            U256::from_str_radix("2386f26fc10000", 16).unwrap()
        );
    }

    #[test]
    fn test_invalid_address() {
        let mut tx_data = create_valid_tx_data();
        tx_data.from = "0xinvalid".to_string();

        let result = parse_raw_transaction(&tx_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid address"));
    }

    #[test]
    fn test_invalid_nonce() {
        let mut tx_data = create_valid_tx_data();
        tx_data.nonce = "0xinvalid".to_string();

        let result = parse_raw_transaction(&tx_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid nonce"));
    }

    #[test]
    fn test_gas_price_overflow() {
        let mut tx_data = create_valid_tx_data();
        // Set gas price higher than u128::MAX to test overflow handling
        tx_data.gas_price =
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string();

        let tx = parse_raw_transaction(&tx_data).unwrap();
        assert_eq!(tx.gas_price().unwrap(), u128::MAX);
        assert_eq!(tx.max_fee_per_gas(), u128::MAX);
        assert_eq!(tx.priority_fee_or_price(), u128::MAX);
    }

    #[test]
    fn test_zero_values() {
        let mut tx_data = create_valid_tx_data();
        tx_data.value = "0x0".to_string();
        tx_data.gas_price = "0x0".to_string();
        tx_data.submission_fee = "0x0".to_string();

        let result = parse_raw_transaction(&tx_data);
        assert!(result.is_ok());
        let tx = result.unwrap();
        assert_eq!(tx.value(), U256::ZERO);
        assert_eq!(tx.gas_price().unwrap(), 0);
        assert_eq!(tx.submission_fee, U256::ZERO);
    }
}
