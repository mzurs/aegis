#!/usr/bin/bash

function setup() {

    # Fetch ledgers
    bash scripts/ledgers.sh fetch_ledgers
    # Fetch minters
    bash scripts/ledgers.sh fetch_minters

}

function main() {

    case $1 in

    "setup")
        setup

        ;;

    *) echo "Invalid arguments passed in ""$0" ;;
    esac

}

main "$1"
