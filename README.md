
# uInfinite Overflow Defense Crate

A lightweight, on-chain-safe fixed-point math crate designed to handle large numeric inputs safely and effectively using automatic `uInfinite` list representation. Built specifically for Solana smart contracts, this crate defends against numeric overflows and simplifies working with arbitrarily large inputs.

## Features

- Automatically splits values >128 characters into `uInfinite` lists
- Simple conversion and reconstruction system
- Fully on-chain safe
- Pre-audited for Solana-native ecosystem integration
- No PDA storage required unless added by developer
- Handles all math operations safely on reconstructed values

## Use Case

Perfect for Solana DeFi protocols, games, data snapshots, or any computation involving potentially massive numerical input.

## How It Works

Any numeric value over 128 characters is automatically sliced into a list of fixed-length strings (128 chars each). This crate treats those slices as valid inputs and reconstructs them internally when performing operations.

## Audit

A full deep audit PDF is included in this repository under `AUDIT_REPORT.pdf`.

## License

MIT OR Apache-2.0
