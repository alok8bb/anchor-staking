# Anchor NFT Staking

A simple NFT staking program with anchor. Users can stake their NFTs to earn reward tokens based on the time staked and configured parameters.

## Instructions

### `initialize_config(points_per_stake: u8, max_stake: u8, freeze_period: u32)`
Initializes the staking configuration with reward parameters and limits.

**Parameters:**
- `points_per_stake`: Points earned per NFT staked
- `max_stake`: Maximum number of NFTs a user can stake
- `freeze_period`: Duration (in seconds) NFTs remain frozen after staking

### `initialize_user()`
Creates a user account to track staking activity and earned points.

### `stake()`
Stakes an NFT by transferring it to the program and freezing it. Users earn points based on the configured rate.

**Requirements:**
- NFT must be from a verified collection
- User must not exceed maximum stake limit
- NFT must be approved for the program

### `unstake()`
Unstakes an NFT and returns it to the user. NFTs can only be unstaked after the freeze period has elapsed.

### `claim()`
Claims earned reward tokens based on accumulated points. Points are reset to zero after claiming.

## Reward System

- **Points Calculation**: `points = amount_staked * points_per_stake`
- **Reward Tokens**: Configurable SPL token mint with 6 decimals
- **Claim Process**: Convert points to reward tokens at 1:1 ratio (adjusted for decimals)

## Account Structure

### Initialize Config (5 accounts)
- `admin` (signer) - Program administrator
- `config` (PDA) - Staking configuration account
- `rewards_mint` (PDA) - Reward token mint
- `system_program` - System program
- `token_program` - SPL Token program

### Initialize User (3 accounts)
- `user` (signer) - User initializing their account
- `user_account` (PDA) - User's staking state account
- `system_program` - System program

### Stake (13 accounts)
- `user` (signer) - NFT owner
- `mint` - NFT mint account
- `collection_mint` - Collection mint account
- `user_mint_ata` - User's NFT token account
- `metadata` - NFT metadata account
- `edition` - NFT edition account
- `stake_account` (PDA) - Individual stake record
- `config` (PDA) - Staking configuration
- `user_state` (PDA) - User's staking state
- `system_program` - System program
- `token_program` - SPL Token program
- `metadata_program` - Metaplex metadata program

### Unstake (10 accounts)
- `user` (signer) - NFT owner
- `mint` - NFT mint account
- `user_mint_ata` - User's NFT token account
- `stake_account` (PDA) - Individual stake record
- `config` (PDA) - Staking configuration
- `user_state` (PDA) - User's staking state
- `metadata` - NFT metadata account
- `system_program` - System program
- `token_program` - SPL Token program
- `metadata_program` - Metaplex metadata program

### Claim (7 accounts)
- `user` (signer) - User claiming rewards
- `rewards_mint` (PDA) - Reward token mint
- `user_account` (PDA) - User's staking state
- `rewards_ata` - User's reward token account
- `config` (PDA) - Staking configuration
- `system_program` - System program
- `token_program` - SPL Token program
- `associated_token_program` - Associated Token program

## State Accounts

### StakeConfig
```rust
pub struct StakeConfig {
    pub points_per_stake: u8,    // Points earned per NFT
    pub max_stake: u8,           // Maximum NFTs per user
    pub freeze_period: u32,      // Freeze duration in seconds
    pub rewards_bump: u8,        // Rewards mint PDA bump
    pub bump: u8,                // Config PDA bump
}
```

### UserAccount
```rust
pub struct UserAccount {
    pub points: u32,             // Accumulated reward points
    pub amount_staked: u8,       // Number of NFTs currently staked
    pub bump: u8,                // User account PDA bump
}
```

### StakeAccount
```rust
pub struct StakeAccount {
    pub owner: Pubkey,           // NFT owner
    pub mint: Pubkey,            // NFT mint address
    pub staked_at: i64,          // Timestamp when staked
    pub bump: u8,                // Stake account PDA bump
}
```
