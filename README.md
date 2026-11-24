# dex-swap-invoker

<p align="center">

**Native Solana Multi-DEX Swap Execution Program**

[![Solana](https://img.shields.io/badge/Solana-Program-9945FF?logo=solana)](https://solana.com/)
[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)

</p>

`dex-swap-invoker` is a native Solana program that executes direct and multi-hop token swaps across multiple DEX protocols using Cross-Program Invocations (CPI).

It provides a unified execution layer to chain swaps across different protocols within a single atomic transaction.

## Supported Protocols

| Protocol | Operations |
|---|---|
| Pump.fun | Buy / Sell |
| PumpSwap | Buy / Sell |
| Orca Whirlpool | Buy / Sell |

## Features

- Multi-DEX swap execution.
- Multi-hop routing within a single transaction.
- Sequential token routing.
- Buy and Sell operations.
- Minimum output amount protection.
- Native Solana implementation using Rust.
- Borsh instruction serialization.

## How It Works

1. Receives a `MultiHopSwapArgs` instruction.
2. Parses shared accounts and route information.
3. Executes each swap through the corresponding DEX CPI.
4. Uses each hop's output as the next hop's input.
5. Validates the minimum final output amount.

## Multi-Hop Swap Example

The program allows chaining multiple swaps across different DEX protocols within a single transaction.

For example, swapping Token A into Token C through Token B:

```text
                 Single Solana Transaction
┌─────────────────────────────────────────────────┐
│                                                 │
│  Token A                                        │
│     │                                           │
│     ▼                                           │
│  ┌──────────────┐                               │
│  │   Pump.fun   │                               │
│  └──────────────┘                               │
│     │                                           │
│     ▼                                           │
│  Token B                                        │
│     │                                           │
│     ▼                                           │
│  ┌──────────────┐                               │
│  │   PumpSwap   │                               │
│  └──────────────┘                               │
│     │                                           │
│     ▼                                           │
│  Token C                                        │
│                                                 │
└─────────────────────────────────────────────────┘
```

The route is defined as:

```rust
MultiHopSwapArgs {
    hops: vec![
        Protocol::Pumpfun,
        Protocol::Pumpswap,
    ],
    direction: SwapDirection::Buy,
    amount_in,
    min_amount_out,
}
```

Each hop consumes the output of the previous swap. If any swap fails or the final output is below `min_amount_out`, the entire transaction is reverted.

## Instruction

```rust
pub struct MultiHopSwapArgs {
    pub hops: Vec<Protocol>,
    pub direction: SwapDirection,
    pub amount_in: u64,
    pub min_amount_out: u64,
}
```

## Requirements

- Rust (2024 edition).
- Solana CLI.
- Solana SBF build tools.

## Installation

```bash
git clone https://github.com/enesanz/dex-swap-invoker.git
cd dex-swap-invoker
```

## Build

```bash
cargo check
cargo test
cargo build-sbf
```

The compiled program is generated at:

```text
target/deploy/dex_swap_invoker.so
```

## Deployment

### 1. Configure wallet

```bash
solana-keygen new
solana config set --url devnet
```

### 2. Fund wallet

```bash
solana airdrop 2
```

### 3. Deploy to Devnet

```bash
cargo build-sbf

solana program deploy target/deploy/dex_swap_invoker.so \
  --program-id target/deploy/dex_swap_invoker-keypair.json
```

### 4. Verify deployment

```bash
solana program show <PROGRAM_ID>
```

### 5. Deploy to Mainnet

After testing on Devnet:

```bash
solana config set --url mainnet-beta

cargo build-sbf

solana program deploy target/deploy/dex_swap_invoker.so \
  --program-id target/deploy/dex_swap_invoker-keypair.json
```

Make sure your wallet has sufficient SOL for deployment.

## Upgrade

To deploy an updated version while keeping the same Program ID:

```bash
cargo build-sbf

solana program deploy target/deploy/dex_swap_invoker.so \
  --program-id target/deploy/dex_swap_invoker-keypair.json
```

The wallet must have the program's upgrade authority.

## Security Considerations

- Validate all accounts, authorities and token mints.
- Respect Solana transaction and compute limits.
- Test routes on Devnet before using Mainnet.
- Keep deployment and upgrade authority keypairs secure.
- `min_amount_out` protects against insufficient final output.

**Built with Rust for Solana.**