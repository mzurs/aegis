#!/usr/bin/env bash

dfx canister call accounts set_canister_id '(variant { CKETH }, principal "apia6-jaaaa-aaaar-qabma-cai")'

dfx canister call insurance set_ledger_canister_id '(variant { CKETH }, principal "apia6-jaaaa-aaaar-qabma-cai")'

dfx canister call options set_ledger_canister_id '(variant { CKETH }, principal "apia6-jaaaa-aaaar-qabma-cai")'

dfx canister call options set_ledger_canister_id '(variant { CKUSDT }, principal "yfumr-cyaaa-aaaar-qaela-cai")'
