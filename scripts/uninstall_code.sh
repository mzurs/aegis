#!/usr/bin/bash

function uninstall_minters() {

    CKBTC_MINTER_ID="mqygn-kiaaa-aaaar-qaadq-cai"
    export CKETH_MINTER_ID="sv3dd-oaaaa-aaaar-qacoa-cai"
    export CKSEPOPLIA_ETH_MINTER_ID="jzenf-aiaaa-aaaar-qaa7q-cai"

    dfx canister uninstall-code $CKBTC_MINTER_ID
    dfx canister uninstall-code $CKSEPOPLIA_ETH_MINTER_ID

}

function uninstall_kyt() {
    KYT_ID="pjihx-aaaaa-aaaar-qaaka-cai"

    dfx canister uninstall-code $KYT_ID
}
function uninstall_ledgers() {

    dfx identity use default

    ICP_LEDGER="ryjl3-tyaaa-aaaaa-aaaba-cai"
    CKBTC_LEDGER_ID="mxzaz-hqaaa-aaaar-qaada-cai"
    export CKETH_LEDGER_ID="ss2fx-dyaaa-aaaar-qacoq-cai"
    CKSEPOPLIA_ETH_LEDGER_ID="apia6-jaaaa-aaaar-qabma-cai"

    # uninstall ICP Ledger Canister

    dfx canister uninstall-code $ICP_LEDGER
    dfx canister uninstall-code $CKBTC_LEDGER_ID
    dfx canister uninstall-code $CKSEPOPLIA_ETH_LEDGER_ID

}

function uninstall_canisters() {

    INSURANCE_ID="suaf3-hqaaa-aaaaf-bfyoa-cai"
    ACCOUNTS_ID="222qi-2qaaa-aaaao-anesa-cai"
    dfx canister uninstall-code $INSURANCE_ID
    dfx canister uninstall-code $ACCOUNTS_ID

}

function uninstall() {

    uninstall_ledgers
    uninstall_minters
    uninstall_kyt
    uninstall_canisters

}

function main() {

    case $1 in

    "uninstall")
        uninstall
        ;;

    *) echo "Invalid Arguments!" ;;

    esac
}

main "$1"
