# AEGIS Accounts

The Account Canister acts as Exchange Wallet for User to Trade, Deposit & Withdraw Funds. The Canister supports mutliple asset from other chains such as BTC, ETH, etc

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
