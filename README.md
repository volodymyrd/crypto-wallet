# crypto-wallet

Crypto Wallet

## Bitcoin

- Install [bitcoincore](https://bitcoincore.org/en/download/)
  For MacOS (brew install bitcoin)

```
bitcoin-cli --version
```



## Solana

### Clusters

* Mainnet - https://api.mainnet-beta.solana.com
* Devnet - https://api.devnet.solana.com
* Testnet - https://api.testnet.solana.com

### Development

- install/update Solana CLI

```
agave-install update
solana --version
```

- current config

```
solana config get
```

- update the Solana CLI cluster

```
solana config set -um    # For mainnet-beta
solana config set -ud    # For devnet
solana config set -ul    # For localhost
solana config set -ut    # For testnet
```

```solana address```

- request an airdrop of devnet SOL

```
solana airdrop 2 <your_public_key>
```

solana transfer <target_public_key> 0.1 --from <source_keypair_file>
