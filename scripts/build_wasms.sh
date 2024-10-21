#!/usr/bin/bash

function move_to_wasm_folder() {
    # Define source and destination directories
    source_dir=".dfx/local/canisters"
    dest_dir="wasms"

    # Check if arguments are provided
    if [ -z "$source_dir" ] || [ -z "$dest_dir" ]; then
        echo "Usage: $0 <source_dir> <destination_dir>"
        exit 1
    fi

    # Check if source directory exists
    if [ ! -d "$source_dir" ]; then
        echo "Error: Source directory '$source_dir' does not exist."
        exit 1
    fi

    # Check if destination directory exists (and create if not)
    if [ ! -d "$dest_dir" ]; then
        mkdir -p "$dest_dir"
    fi

    # Find all .wasm files recursively and copy them to the destination directory
    find "$source_dir" -type f -name "*.wasm" -exec cp -v {} "$dest_dir" \;

    echo "Successfully copied .wasm files from '$source_dir' to '$dest_dir'"

}

function main() {
    if [[ $1 == "local" ]]; then

        # Build all canister wasms
        bash scripts/dev.sh deploy

        # Copies the Wasm from .dfx to wasm dir
        move_to_wasm_folder

        dfx generate
    fi

}

main "$1"
