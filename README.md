# AEGIS FINANCE

Aegis Finance leverages the IC blockchain to power its decentralized exchange platform, enabling a range of use cases.

1. It acts as Exchange Wallet for User to Trade, Deposit & Withdraw Funds. The Canister supports mutliple asset from other chains such as BTC, ETH, etc.

2. Allow users to trade inusrance contracts.

3. Allow users to trade options contracts

#### Note

This repo contains Backend Canisters code, for Frontend Canister code, please visit [Aegis-Dapp](https://github.com/AegisFinance/aegis-dapp)

#### Aegis Finance Canisters

- accounts (It manages user tokens such as ICP, ckBTC, ckETH.)
- main (It supports the staking of $AEGIS tokens.)
- insurance (It supports the buying/selling of insurance contracts.)
- options (It supports the buying/selling of options contracts.)

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

```bash
yarn deploy:all
```

```bash
dfx generate
```

- <b>Unit Testing</b>
  ```bash
  yarn test:unit:(canister-name)
  ```
- <b>Integration Testing</b>
  ```bash
  yarn test:integration:(canister-name)
  ```

### Features

- [x] Deposit & Withdraw ICRCs Tokens (ICP, ckBTC, ckETH).
- [x] Convert BTC <-> ckBTC.
- [x] Convert ETH <-> ckETH.
- [x] Insurance Contracts (ICP, ckBTC, ckETH).
- [x] Stake ICRC tokens (AEGIS).
- [x] Options Contracts (ckBTC & ckETH).
