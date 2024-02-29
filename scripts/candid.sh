#!/usr/bin/env bash

function generate_did() {
    local canister=$1
    canister_root="src/$canister"

    echo "$canister_root"
    cargo build --manifest-path="$canister_root/Cargo.toml" \
        --target wasm32-unknown-unknown \
        --release --package "$canister" -p "$canister" --locked

    candid-extractor "target/wasm32-unknown-unknown/release/$canister.wasm" >"$canister_root/$canister.did"
}

generate_did "$1"