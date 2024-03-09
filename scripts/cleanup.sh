#!/usr/bin/env bash

command=$1

if [[ $command == "all" ]]; then

    rm -rf .dfx
    rm -rf .bitcoin
    rm -rf target

else

    rm -rf .dfx
    rm -rf target

fi
