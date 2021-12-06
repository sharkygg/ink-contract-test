## ink-contract-test

### how to run
- start apron node
```bash
git clone -b upgrade_contract https://github.com/Apron-Network/apron-node.git
cd apron-node
cargo build --release
./target/release/apron-node --dev
```

- compile contract and deploy
```bash
git clone -b 3.0.0-rc6 https://github.com/Apron-Network/apron-contracts.git
cd apron-contracts
./build.sh
```
then deploy market and statistic, set market owner is `5F7Xv7RaJe8BBNULSuRTXWtfn68njP1NqQL5LLf41piRcEJJ`, and transfer some token to the account.

- modify config
in ./src/main.rs modify const
```rust

// substrate node rpc
const WS_ENDPOINT: &str = "ws://127.0.0.1:9944";

const MARKET_CONTRACT_ADDR: &str = "5DVJQ7rK6L5fgRvDQpjQ7CMweyXAaYXwRBKMQY7yKhGo5hqk";
const MARKET_ABI_PATH: &str = "./release/services_market.json";

const STAT_CONTRACT_ADDR: &str = "5CGW7GKo13RdxwMFuYuUEw5eAq3eTM7G9fGk16p3EzKuGi3r";
const STAT_ABI_PATH: &str = "./release/services_statistics.json";

// execute contract private key
const SURI: &str = "0xe40891ed4fa2eb6b8b89b1d641ae72e8c1ba383d809eeba64131b37bf0aa3898";
```  
