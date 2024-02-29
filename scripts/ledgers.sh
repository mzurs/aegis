#!/bin/bash

function fetch_ledgers() {

    rm src/ledgers/*

    cd src/ledgers || bash

    IC_VERSION=ac04d772c23ff771eaf526bee9ca9e9b411e129d

    echo "Fetching ICP ledger wasm..."
    curl -o icp_ledger.wasm.gz https://download.dfinity.systems/ic/$IC_VERSION/canisters/ledger-canister.wasm.gz
    curl -o icp_ledger.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icp_ledger/ledger.did"

    echo -e "\nFetching ckBTC ledger wasm..."
    curl -o ckbtc_ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz"
    curl -o ckbtc_ledger.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/ledger.did"

    echo -e "\nFetching ckETH ledger wasm..."
    curl -o cketh_ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz"
    curl -o cketh_ledger.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/ledger.did"

    gunzip icp_ledger.wasm.gz
    gunzip ckbtc_ledger.wasm.gz
    gunzip cketh_ledger.wasm.gz

    cd ../..

}

function fetch_minters() {

    rm src/minter/*.wasm

    cd src/minter || bash

    IC_VERSION=ac04d772c23ff771eaf526bee9ca9e9b411e129d

    echo -e "\nFetching ckBTC Minter..."
    curl -o ckbtc_minter.wasm.gz https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-ckbtc-minter.wasm.gz

    echo -e "\nFetching ckETH Minter..."
    curl -o cketh_minter.wasm.gz https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-cketh-minter.wasm.gz

    gunzip ckbtc_minter.wasm.gz
    gunzip cketh_minter.wasm.gz

    cd ../..
}
function main() {

    case $1 in

    "fetch_ledgers")
        fetch_ledgers
        ;;

    "fetch_minters")

        fetch_minters
        ;;

    *) echo "Invalid argument pass in " "$0" ;;

    esac

}

main "$1"
