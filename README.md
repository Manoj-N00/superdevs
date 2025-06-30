# Solana HTTP Server

A Rust-based HTTP server that exposes Solana-related endpoints for generating keypairs, handling SPL tokens, signing/verifying messages, and constructing valid on-chain instructions.

## Features

- Generate Solana keypairs
- Create and mint SPL tokens
- Sign and verify messages using Ed25519
- Create SOL transfer instructions
- Create SPL token transfer instructions
- Comprehensive error handling
- Input validation
- Secure cryptographic operations

## Prerequisites

- Rust and Cargo (latest stable version)
- Basic understanding of Solana and SPL tokens

## Setup

1. Clone the repository
2. Install dependencies:
   ```bash
   cargo build
   ```
3. Run the server:
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:8080`

## API Endpoints

### Generate Keypair
`POST /keypair`
- Generates a new Solana keypair
- Returns base58-encoded public and secret keys

### Create Token
`POST /token/create`
- Creates a new SPL token initialization instruction
- Requires mint authority, mint address, and decimals

### Mint Token
`POST /token/mint`
- Creates a mint-to instruction for SPL tokens
- Requires mint address, destination address, authority, and amount

### Sign Message
`POST /message/sign`
- Signs a message using Ed25519
- Requires message and base58-encoded secret key

### Verify Message
`POST /message/verify`
- Verifies a signed message
- Requires message, base64-encoded signature, and base58-encoded public key

### Send SOL
`POST /send/sol`
- Creates a SOL transfer instruction
- Requires sender address, recipient address, and amount in lamports

### Send Token
`POST /send/token`
- Creates an SPL token transfer instruction
- Requires destination address, mint address, owner address, and amount

## Security Considerations

- No private keys are stored on the server
- All cryptographic operations use standard libraries
- Input validation for all endpoints
- Proper error handling to avoid information leakage

## Error Handling

All endpoints return responses in a consistent format:

```json
{
  "success": true/false,
  "data": { /* endpoint-specific result */ },
  "error": "Error message if success is false"
}
```

