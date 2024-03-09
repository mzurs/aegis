#!/usr/bin/bash

function setup() {

    # Fetch ledgers
    bash scripts/ledgers.sh fetch_ledgers
    # Fetch minters
    bash scripts/ledgers.sh fetch_minters
    # Fetch KYT
    bash scripts/ledgers.sh fetch_kyt

}

function main() {

    case $1 in

    "setup")
        setup

        ;;
    "deploy")
        bash scripts/deploy.sh deploy
        ;;

    *) echo "Invalid arguments passed in ""$0" ;;
    esac

}

main "$1"
