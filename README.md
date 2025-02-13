endpoint: `POST /send_transaction`

```json
{
  "raw_tx": {
    "nonce": "hex string starting w 0x",
    "from": "20-byte hex address",
    "to": "20-byte hex address",
    "value": "hex string for wei amount",
    "data": "hex string for calldata",
    "gas_limit": "hex gas limit",
    "gas_price": "hex gas price in wei",
    "chain_id": 42161,
    "l1_block_number": 0,
    "submission_fee": "hex string for l1 submission fee"
  }
}
```

Example Request

curl -X POST http://localhost:3000/send_transaction \
-H "Content-Type: application/json" \
-H "Authorization: Bearer 7a28b4cd5e614a5688b36639c4af959d9641def00f33f8951da1fa2bf0726359" \
-d '{
"raw_tx": {
"nonce": "0x0",
"from": "0x1111111111111111111111111111111111111111",
"to": "0x2222222222222222222222222222222222222222",
"value": "0xde0b6b3a7640000",
"data": "0x68656c6c6f",
"gas_limit": "0x5208",
"gas_price": "0x6fc23ac00",
"chain_id": 42161,
"l1_block_number": 0,
"submission_fee": "0xf4240"
}
}'
