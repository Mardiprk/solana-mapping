# Simple Map

A minimal Solana program that demonstrates basic data storage using Program Derived Addresses (PDAs). Each user can store and update a single `u64` value on-chain.

## Features

- Initialize a personal account with a value
- Update stored value (authority-protected)
- Uses PDA for deterministic account addressing

## Quick Start

```bash
# Install dependencies
yarn install

# Run tests
anchor test
```

## How it Works

The program creates a PDA for each user using the seed `["user", authority.key()]`. Only the account authority can update the stored value.

```rust
// Initialize with a value
pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()>

// Update the value (authority required)
pub fn set(ctx: Context<Set>, new_value: u64) -> Result<()>
```

## Account Structure

```rust
#[account]
pub struct UserAccount {
    pub authority: Pubkey,  // 32 bytes
    pub data: u64,          // 8 bytes
}
```

## Test Output

```
📦 PDA: 8zy2dNPzUPX3kiqRAbBF4YZV3gvzoijmHbvyByM77rrb
✅ Stored Data: 23
🔁 Updated Value: 99
```

---

*Built with Anchor on Solana*