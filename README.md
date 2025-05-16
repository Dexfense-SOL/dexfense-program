# Dexfense Protocol (Rust Anchor Program)

Dexfense Protocol is a game-driven DeFi experiment that connects player performance to real-time token rewards using an on-chain liquidity model. This repository contains the Solana smart contracts written in [Anchor](https://book.anchor-lang.com/) to support core gameplay, token interactions, and reward logic.

## 🔧 Features

- **Game-Session Based Architecture**: Each play session is initialized, recorded, and validated on-chain.
- **Dynamic Treasury Management**: Entry fees and rewards are handled through dedicated vault PDAs.
- **Token Swap Integration**: Built-in interaction with a minimal AMM Dex for swap-based mechanics.
- **Reward Calculation**: Rewards are calculated using in-game performance, difficulty level, and a fixed reward multiplier.
- **Mint-on-Win Model**: Rewards are partially minted, partially pulled from liquidity pools, depending on performance.

## 🛠️ Project Structure

```

dexfense-program/
├── programs/
│   ├── dexfense-core/         # Main game logic and reward settlement
│   └── simple-amm-dex/        # Lightweight AMM for token swaps
├── tests/                     # Anchor-based integration tests
├── scripts/                   # Utility scripts (initialize pool, simulate games, etc.)
└── target/                    # Compiled artifacts

````

## 🚀 Deployment

### 1. Set up your environment

```bash
anchor build
anchor deploy --provider.cluster devnet
````

### 2. Initialize Pool

```bash
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
npx ts-node scripts/initialize_pool.ts
```

### 3. Start Game Session

```bash
anchor run initialize_game_session \
  --provider.cluster devnet \
  -- \
  --token1 <TOKEN1_MINT> \
  --token2 <TOKEN2_MINT> \
  --difficulty easy
```

### 4. Settle Game Result

```bash
anchor run settle_game_result \
  --provider.cluster devnet \
  -- \
  --result <kill_count>
```

## 📦 Token & Pool Management

* SPL Token minting and account creation is done via `spl-token` CLI.
* AMM pool tokens and user token accounts are created and funded before gameplay begins.

## 📜 Requirements

* Node.js (v18+)
* Solana CLI
* `spl-token` CLI
* Anchor CLI (`cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked`)
* Typescript for scripts

## 🙌 Contributors

* **@daniel** – Protocol and Game Architect
* Special thanks to on-chain game builders and Solana devs worldwide.

## 📄 License

MIT License
