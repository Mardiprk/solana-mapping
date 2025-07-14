# Solana Mapping Example

This project demonstrates a simple Solana smart contract (program) using [Anchor](https://book.anchor-lang.com/). It allows users to initialize an account and store a single `u64` value, which can later be updated by the account's authority.

## Features
- Initialize a user account with a value
- Update the value (only by the account authority)
- Built with Anchor framework

## How it Works

```mermaid
%% Diagram: User interacts with Solana program to store and update data
---
graph TD;
  A["User"] -->|"Initialize Account"| B["Solana Program (simple_map)"];
  B -->|"Stores Data & Authority"| C["UserAccount PDA"];
  A -->|"Set New Value"| B;
  B -->|"Updates Data if Authorized"| C;
  C -->|"Holds: authority, data (u64)"| D["Solana Blockchain"];
```

## Quick Start

1. Install dependencies:
   ```bash
   yarn install
   ```
2. Build and test:
   ```bash
   anchor test
   ```

## Project Structure
- `programs/solana-mapping/` - Solana program source code (Rust)
- `tests/` - Integration tests (TypeScript)
- `migrations/` - Deployment scripts

---

*Created with ❤️ using Anchor on Solana.* 