#!/bin/bash

function deploy_kyt() {
  read -r -d '' argument <<CANDID
(variant {
    InitArg = record {
      minter_id = principal "mqygn-kiaaa-aaaar-qaadq-cai";
      maintainers = vec {
        principal "$(dfx identity get-principal)"
      };
      mode = variant { AcceptAll }
    }
  })
CANDID

  dfx deploy kyt --specified-id pjihx-aaaaa-aaaar-qaaka-cai --argument "$argument"

  dfx canister call kyt set_api_key '(record { api_key = "" })'
}

function deploy_minters() {

  #  Deploying ckBTC Minter Canister
  read -r -d '' argument <<CANDID
    (variant {
        Init = record {
            btc_network = variant { Regtest };
            min_confirmations=opt 1;
            ledger_id = principal "mxzaz-hqaaa-aaaar-qaada-cai";
            kyt_principal = opt principal "pjihx-aaaaa-aaaar-qaaka-cai";
            ecdsa_key_name = "dfx_test_key";
            retrieve_btc_min_amount = 5_000;
            max_time_in_queue_nanos = 420_000_000_000;
            mode = variant {GeneralAvailability}
        }
    })
CANDID

  dfx deploy ckbtc_minter --specified-id mqygn-kiaaa-aaaar-qaadq-cai --argument "$argument"

  # # Deploying ckETH Minter Canister

  # # Execute the curl command and capture the output
  # response=$(curl -X POST 'https://ethereum-sepolia.publicnode.com' \
  #   -H 'Content-Type: application/json' \
  #   -d '{
  #   "jsonrpc":"2.0",
  #   "method":"eth_getTransactionCount",
  #   "params":[
  #     "0x1789F79e95324A47c5Fd6693071188e82E9a3558",
  #     "latest"
  #   ],
  #   "id":1
  # }')

  # # Extract the "result" field using jq
  # NEXT_NONCE=$(echo "$response" | jq -r '.result')

  # # Print the extracted value
  # echo "Transaction count: $NEXT_NONCE"

  # dfx deploy cketh_minter --argument "(variant {InitArg = record { ethereum_network = variant {Sepolia} ;
  # ecdsa_key_name = \"key_1\"; ethereum_contract_address = opt \"0xb44B5e756A894775FC32EDdf3314Bb1B1944dC34\" ;
  # ledger_id = principal \"$(dfx canister id cketh_ledger)\"; ethereum_block_height = variant {Finalized} ;
  # minimum_withdrawal_amount = 10_000_000_000_000_000; next_transaction_nonce = $NEXT_NONCE ; last_scraped_block_number = 4_775_280}})" \
  #   --specified-id sv3dd-oaaaa-aaaar-qacoa-cai
}

function deploy_ledgers() {

  # Deploy ckBTC Ledger Canister
  read -r -d '' argument <<CANDID
    (variant {
        Init = record {
            minting_account = record {
                owner = principal "mqygn-kiaaa-aaaar-qaadq-cai"
            };
            transfer_fee = 0 ;
            token_symbol = "ckBTC";
            token_name = "ckBTC";
            metadata = vec {};
            initial_balances = vec {};
            archive_options = record {
                num_blocks_to_archive = 0 ;
                trigger_threshold = 0  ;
                controller_id = principal "aaaaa-aa"
            }
        }
    })
CANDID

  dfx deploy ckbtc_ledger --specified-id mxzaz-hqaaa-aaaar-qaada-cai --argument "$argument"

  #   # Deploy ckETH Ledger Canister

  #   dfx deploy cketh_ledger --argument \
  #     '(variant { Init = record { minting_account = record { owner = principal  "'"$(dfx identity get-principal)"'"};
  #  feature_flags  = opt record { icrc2 = true }; decimals = opt 18; max_memo_length = opt 80;
  #  transfer_fee = 10_000_000_000; token_symbol = "ckSepoliaETH"; token_name = "Chain key Sepolia Ethereum";
  #  metadata = vec {}; initial_balances = vec {}; archive_options = record { num_blocks_to_archive = 1000;
  #   trigger_threshold = 2000; max_message_size_bytes = null; cycles_for_archive_creation = opt 1_000_000_000_000;
  #    node_max_memory_size_bytes = opt 3_221_225_472; controller_id = principal "mf7xa-laaaa-aaaar-qaaaa-cai"; } }})' \
  #     --specified-id ss2fx-dyaaa-aaaar-qacoq-cai

}

function deploy_canisters() {
  bash scripts/candid.sh accounts
  dfx deploy accounts --argument='(record { bitcoin_network= variant { regtest }})' --specified-id 222qi-2qaaa-aaaao-anesa-cai
}

function deploy() {

  deploy_ledgers
  deploy_minters
  deploy_kyt
  deploy_canisters

}
function main() {

  case $1 in

  "deploy")
    deploy
    ;;

  *) echo "Invalid Arguments!" ;;

  esac
}

main "$1"
