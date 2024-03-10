#!/usr/bin/env bash

function install_bitcoin_node() {

    echo "Installing Bitcoin Node .....!!!!"

    echo "Removing Bitcoin Dir.."
    rm -r .bitcoin/data
    rm -r .bitcoin

    echo "Creating Bitcoin Dir ..."
    mkdir .bitcoin
    mkdir .bitcoin/data

    if [[ $1 == "download" ]]; then

        echo "Downloading Bitcoin Core..."
        curl https://bitcoincore.org/bin/bitcoin-core-23.0/bitcoin-23.0-x86_64-linux-gnu.tar.gz -o bitcoin.tar.gz

    fi

    tar xzf bitcoin.tar.gz --overwrite --strip-components=1 --directory=.bitcoin/ bitcoin-23.0/bin/

    # rm -rf bitcoin.tar.gz

    echo "Bitcoin Node Installed!"

}

function start_bitcoin_node() {

    echo "Starting Bitcoin Node......."

    .bitcoin/bin/bitcoind -conf="$(pwd)"/.bitcoin.conf -datadir="$(pwd)"/.bitcoin/data --port=18444

}

function mine_to_address() {

    .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf generatetoaddress "$1" "$2"
}

function bitcoin_wallet() {

    echo "$1"
    echo "$2"
    if [[ $2 == "create" ]]; then

        echo "Creating Bitcoin Wallet on Regtest Network..."

        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf createwallet "testwallet"
        # .bitcoin/bin/bitcoin-cli --conf="$(pwd)"/.bitcoin.conf loadwallet testwallet

        # create a new deposit address after creating a new wallet
        NEW_ADDRESS="$(.bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet getnewaddress "" legacy)"
        echo "New Address of Regtest Wallet = $NEW_ADDRESS"
        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet generatetoaddress 110 "$NEW_ADDRESS"

    elif [[ $2 == "wallet_info" ]]; then

        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet getwalletinfo

    elif [[ $2 == "new_address" ]]; then

        NEW_ADDRESS="$(.bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet getnewaddress "" legacy)"
        echo "New Address of Regtest Wallet = $NEW_ADDRESS"
        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet generatetoaddress 101 "$NEW_ADDRESS"

    elif [[ $2 == "wallet_list" ]]; then

        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet listaddressgroupings

    elif [[ $2 == "send" ]]; then

        echo "$3"

        echo "$4"
        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet sendtoaddress "$3" "$4" "drinks" "room77" true true null "unset" null 1.1

        # after transfer mine 1 block to attach tx in a next block
        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet generatetoaddress 1 mq42nonp2JVeVN6RCRCgfrEutDSxij4cqR

    elif [[ $2 == "mine" ]]; then

        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet generatetoaddress 101 "$3"

    elif [[ $2 == "mine_one_block" ]]; then

        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet generatetoaddress 1 "$3"

    elif [[ $2 == "balance" ]]; then

        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet getbalance

    elif [[ $2 == "mine_block" ]]; then

        .bitcoin/bin/bitcoin-cli -conf="$(pwd)"/.bitcoin.conf -rpcwallet=testwallet generatetoaddress 1 mq42nonp2JVeVN6RCRCgfrEutDSxij4cqR

    else
        echo "Invalid Function Call"
    fi
}

function main() {

    case $1 in

    "download")
        install_bitcoin_node "download"
        ;;
    "install")
        install_bitcoin_node
        ;;
    "start")
        start_bitcoin_node
        ;;
    "wallet")
        bitcoin_wallet "$1" "$2" "$3" "$4"
        ;;
    *) echo "Invalid Argument!" ;;
    esac
}
main "$1" "$2" "$3" "$4"
