# AEGIS FINANCE

Aegis Finance leverages the IC blockchain to power its decentralized exchange platform, enabling a range of use cases.

1. It acts as Exchange Wallet for User to Trade, Deposit & Withdraw Funds. The Canister supports mutliple asset from other chains such as BTC, ETH, etc.

2. Allow users to trade inusrance contracts.

3. Allow users to trade options contracts

#### Note

This repo contains Backend Canisters code, for Frontend Canister code, please visit [Aegis-Dapp](https://github.com/AegisFinance/aegis-dapp)

## Start IC Replica

```bash
yarn replica:start
```

## Bitcoin Node

- Download Bitcoin Node

```bash
yarn bitcoind:download
```

- Start Bitcoin Node

```bash
yarn bitcoind:start
```

## Deploy Canisters

```bash
yarn  deploy:all
```

## Testing

[PocketIC](https://github.com/dfinity/pocketic) is being used for testing Aegis Canisters, [tests](tests/accounts.spec.ts)

```bash
yarn deploy:all
```

```bash
dfx generate
```

```bash
yarn test:all
```

### Features


- [x] Deposit & Withdraw ICRCs Tokens (ICP, ckBTC, ckETH).
- [x] Convert BTC <-> ckBTC.
- [x] Convert ETH <-> ckETH.
- [x] Insurance Contracts (ICP, ckBTC, ckETH).
- [x] Stake ICRC tokens (AEGIS).
- [x] Options Contracts (ckBTC & ckETH).
 