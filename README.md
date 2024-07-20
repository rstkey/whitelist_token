# Whitelist Token Sale Program

## Overview
This Anchor-based Solana program enables a whitelist-gated token sale with a fixed token price and a purchase limit per wallet.

## Features
- **Initialize Sale**: Set token price, wallet purchase limit, and total supply.
- **Whitelist Users**: Add or remove users from the whitelist.
- **Purchase Tokens**: Allow whitelisted users to buy tokens.
- **Set Token Price**: Update the token price.
- **Pause/Resume Sale**: Temporarily halt or resume the sale.
- **Return Tokens**: Return tokens to the vault.

## Instructions

### Initialize Sale
```rust
pub fn initialize(ctx: Context<Initialize>, token_price: u64, purchase_limit_per_wallet: u64, total_supply: u64) -> Result<()>;
```

### Whitelist User
```rust
pub fn whitelist_user(ctx: Context<WhitelistUser>, user: Pubkey) -> Result<()>;
```

### Purchase Tokens
```rust
pub fn purchase_tokens(ctx: Context<PurchaseTokens>, amount: u64) -> Result<()>;
```

### Remove Whitelist User
```rust
pub fn remove_whitelist_user(ctx: Context<RemoveWhitelistUser>, user: Pubkey) -> Result<()>;
```

### Set Token Price
```rust
pub fn set_token_price(ctx: Context<SetTokenPrice>, new_price: u64) -> Result<()>;
```

### Pause Sale
```rust
pub fn pause_sale(ctx: Context<PauseSale>) -> Result<()>;
```

### Resume Sale
```rust
pub fn resume_sale(ctx: Context<ResumeSale>) -> Result<()>;
```

### Return Tokens
```rust
pub fn return_tokens(ctx: Context<ReturnTokens>, amount: u64) -> Result<()>;
```

### Accounts
## SaleAccount
State of the sale, including token mint, vault, and sale parameters.

## BuyerInfo
State of a whitelisted buyer, including purchase amount and whitelist status.

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [here](https://www.rust-lang.org/tools/install).
- **Solana CLI**: Install the Solana CLI tools by following the instructions [here](https://docs.solana.com/cli/install-solana-cli-tools).
- **Anchor**: Install Anchor by following the instructions [here](https://book.anchor-lang.com/chapter_2/installation.html).
- **Node.js**: Ensure you have Node.js and npm installed. You can download them from [here](https://nodejs.org/).

### Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/whitelist_token.git
   cd whitelist_token

2. **Install dependencies**:
   ```sh
   npm install

3. **Build the project**:
   ```sh
   anchor build

## Deployment

1. **Deploy the program**:
   ```sh
   anchor deploy

2. **Verify the deployment**:
   Ensure the program ID in lib.rs matches the deployed program ID.

## Usage

1. **Initialize**:
   Set up your Solana environment by configuring your wallet and network settings.

2. **Run the program**:
   Execute the program using Solana CLI commands or scripts provided in the repository.

3. **Test the program**:
   Run the tests to ensure everything works correctly:
   ```sh
   anchor test  

### Contributing
Please open issues or submit pull requests for improvements or bug fixes.

### Dependencies

1. **Anchor**: Framework for Solana programs.

2. **Anchor-Spl-Token**: SPL Token library for working with the Solana program tokens.

### Contact

If you have any questions, feel free to reach out at [ritikbhatt020@gmail.com].
