#!/usr/bin/bash

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
  dfx identity new minter || true

  export CKETH_MINTER_ID="sv3dd-oaaaa-aaaar-qacoa-cai"
  export CKSEPOPLIA_ETH_MINTER_ID="jzenf-aiaaa-aaaar-qaa7q-cai"

  #  Deploying ckBTC Minter Canister
  read -r -d '' ckbtc_argument <<CANDID
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

  dfx deploy ckbtc_minter --specified-id mqygn-kiaaa-aaaar-qaadq-cai --argument "$ckbtc_argument"

  # Deploying ckETH Minter Canister

  # Execute the curl command and capture the output
  response=$(curl -X POST 'https://ethereum-sepolia.publicnode.com' \
    -H 'Content-Type: application/json' \
    -d '{
    "jsonrpc":"2.0",
    "method":"eth_getTransactionCount",
    "params":[
      "0x1789F79e95324A47c5Fd6693071188e82E9a3558",
      "latest"
    ],
    "id":1
  }')

  # Extract the "result" field using jq
  NEXT_NONCE=$(echo "$response" | jq -r '.result')

  # Print the extracted value
  echo "Transaction count: $NEXT_NONCE"

  CONTRACT_ADDRESS="0xb44B5e756A894775FC32EDdf3314Bb1B1944dC34"
  CKETH_LEDGER_ID="ss2fx-dyaaa-aaaar-qacoq-cai"

  read -r -d '' cketh_argument <<CANDID
    (variant {
        InitArg = record {
            ethereum_network = variant {Sepolia} ; 
            ecdsa_key_name = "dfx_test_key"; 
            ethereum_contract_address = opt "$CONTRACT_ADDRESS" ; 
            ledger_id = principal "$CKETH_LEDGER_ID"; 
            ethereum_block_height = variant {Finalized} ; 
            minimum_withdrawal_amount = 10_000_000_000_000_000; 
            next_transaction_nonce = $NEXT_NONCE ;
            last_scraped_block_number = 5_533_180;
        }
    })
CANDID

  dfx deploy cketh_minter --specified-id $CKSEPOPLIA_ETH_MINTER_ID --argument "$cketh_argument"

}

function deploy_ledgers() {
  CKETH_LEDGER_ID="ss2fx-dyaaa-aaaar-qacoq-cai"
  CKSEPOPLIA_ETH_LEDGER_ID="apia6-jaaaa-aaaar-qabma-cai"
  CKUSDT_LEDGER_ID="cngnf-vqaaa-aaaar-qag4q-cai"
  CKSEPOPLIA_USDT_LEDGER_ID="yfumr-cyaaa-aaaar-qaela-cai"

  # Deploy ICP Ledger Canister
  dfx identity use minter
  MINT_ACC=$(dfx ledger account-id)
  MINTER_PRINCIPAL=$(dfx identity get-principal)

  dfx identity use default
  LEDGER_PRINCIPAL=$(dfx identity get-principal)
  
  PRE_MINTED_CKETH_TOKENS=10_000_000_000_000_000_000
  PRE_MINTED_CKBTC_TOKENS=1_000_000_000_000
  
  read -r -d '' icp_argument <<CANDID
    (variant 
        {Init = record {
          minting_account = "$MINT_ACC";
          initial_values = vec { } ; 
          archive_options = opt record {
            num_blocks_to_archive = 1000000;
             trigger_threshold = 1000000;
             controller_id = principal  "$LEDGER_PRINCIPAL"; };
              send_whitelist = vec {}
              }
        }
    )
CANDID

  dfx deploy icp_ledger --specified-id ryjl3-tyaaa-aaaaa-aaaba-cai --argument "$icp_argument"

  # Deploy ckBTC Ledger Canister
  read -r -d '' ckbtc_argument <<CANDID
    (variant {
        Init = record {
            minting_account = record {
                owner = principal "mqygn-kiaaa-aaaar-qaadq-cai"
            };
            
            transfer_fee = 10 ;
            token_symbol = "ckBTC";
            token_name = "ckBTC";
            metadata = vec {};
            initial_balances = vec {record { record { owner = principal "$MINTER_PRINCIPAL"; }; $PRE_MINTED_CKBTC_TOKENS; };};
            archive_options = record {
                num_blocks_to_archive = 0 ;
                trigger_threshold = 0  ;
                controller_id = principal "aaaaa-aa"
            }
        }
    })
CANDID

  dfx deploy ckbtc_ledger --specified-id mxzaz-hqaaa-aaaar-qaada-cai --argument "$ckbtc_argument"

  # Deploy ckETH Ledger Canister

  CKETH_MINTING_ACCOUNT="sv3dd-oaaaa-aaaar-qacoa-cai"

  read -r -d '' cketh_argument <<CANDID
    (variant {
        Init = record {
            minting_account = record {
                owner = principal "$CKETH_MINTING_ACCOUNT"
            };
            feature_flags  = opt record { icrc2 = true };
            decimals = opt 18; 
            max_memo_length = opt 80;
            transfer_fee = 10_000_000_000 ;
            token_symbol = "ckSepoliaETH";
            token_name = "ckSepoliaETH";
            metadata = vec {};
            initial_balances = vec {record { record { owner = principal "$MINTER_PRINCIPAL"; }; $PRE_MINTED_CKETH_TOKENS; };};
            archive_options = record {
            num_blocks_to_archive = 1000; 
            trigger_threshold = 2000; 
            max_message_size_bytes = null; 
            cycles_for_archive_creation = opt 1_000_000_000_000; 
            node_max_memory_size_bytes = opt 3_221_225_472; 
            controller_id = principal "$CKETH_MINTING_ACCOUNT";
            }
        }
    })
CANDID

  dfx deploy cketh_ledger --specified-id $CKSEPOPLIA_ETH_LEDGER_ID --argument "$cketh_argument"



    # Deploy ckUSDT Ledger Canister

  # CKETH_MINTING_ACCOUNT="sv3dd-oaaaa-aaaar-qacoa-cai"

  read -r -d '' ckusdt_argument <<CANDID
    (variant {
        Init = record {
            minting_account = record {
                owner = principal "$CKETH_MINTING_ACCOUNT"
            };
            feature_flags  = opt record { icrc2 = true };
            decimals = opt 18; 
            max_memo_length = opt 80;
            transfer_fee = 10_000_000_000 ;
            token_symbol = "ckSepoliaETH";
            token_name = "ckSepoliaETH";
            metadata = vec {};
            initial_balances = vec {record { record { owner = principal "$MINTER_PRINCIPAL"; }; $PRE_MINTED_CKETH_TOKENS; };};
            archive_options = record {
            num_blocks_to_archive = 1000; 
            trigger_threshold = 2000; 
            max_message_size_bytes = null; 
            cycles_for_archive_creation = opt 1_000_000_000_000; 
            node_max_memory_size_bytes = opt 3_221_225_472; 
            controller_id = principal "$CKETH_MINTING_ACCOUNT";
            }
        }
    })
CANDID

  dfx deploy ckusdt_ledger --specified-id $CKSEPOPLIA_USDT_LEDGER_ID --argument "$ckusdt_argument"

}

function deploy_aegis() {
  # Deploy ICP Ledger Canister
  dfx identity use minter
  MINTING_PRINCIPAL=$(dfx identity get-principal)

  dfx identity use default
  LEDGER_PRINCIPAL=$(dfx identity get-principal)

  AEGIS_LEDGER_ID="2jymc-fyaaa-aaaar-qad2q-cai"
  AEGIS_INDEX_ID="2dm64-liaaa-aaaar-qaega-cai"

  # echo "Fetching Aegis Ledger Canister Wasm..."
  # curl -o aegis_ledger.wasm.gz "https://download.dfinity.systems/ic/$IC_VERSION/canisters/ic-icrc1-ledger.wasm.gz",

  # echo "Fetching Aegis Ledger Canister Candid..."
  # curl -o aegis_ledger.did "https://raw.githubusercontent.com/dfinity/ic/$IC_VERSION/rs/rosetta-api/icrc1/ledger/ledger.did",

  read -r -d '' aegis_ledger_argument <<CANDID
    (variant {
        Init = record {
            minting_account = record {
                owner = principal "$MINTING_PRINCIPAL"
            };
            feature_flags  = opt record { icrc2 = true };
            decimals = opt 8; 
            max_memo_length = opt 80;
            transfer_fee = 100_000 ;
            token_symbol = "AEGIS";
            token_name = "AEGIS FINANCE";
            metadata = vec {};
            initial_balances = vec {};
            archive_options = record {
            num_blocks_to_archive = 1000; 
            trigger_threshold = 2000; 
            max_message_size_bytes = null; 
            cycles_for_archive_creation = opt 1_000_000_000_000; 
            node_max_memory_size_bytes = opt 3_221_225_472; 
            controller_id = principal "$LEDGER_PRINCIPAL";
            }
        }
    })
CANDID

  dfx deploy aegis_ledger --specified-id $AEGIS_LEDGER_ID --argument "$aegis_ledger_argument"

  read -r -d '' aegis_index_argument <<CANDID
    (opt variant {
        Init = record {
        ledger_id = principal "$AEGIS_LEDGER_ID"
           
             }
    })
CANDID

  dfx deploy aegis_index --argument "$aegis_index_argument" --specified-id $AEGIS_INDEX_ID

  #
  #
  #
  dfx canister call aegis_index ledger_id '()'
  dfx canister call aegis_index status '()'
}

function deploy_canisters() {

  bash scripts/candid.sh accounts

  bash scripts/candid.sh insurance
  
  bash scripts/candid.sh main

  dfx deploy accounts --argument='(record { bitcoin_network= variant { regtest }})' --specified-id 222qi-2qaaa-aaaao-anesa-cai

  dfx deploy insurance --argument='(record { bitcoin_network= variant { regtest }})' --specified-id suaf3-hqaaa-aaaaf-bfyoa-cai

  dfx deploy main --argument='(record { bitcoin_network= variant { regtest }})'  --specified-id 23633-jiaaa-aaaar-qadzq-cai

  dfx deploy options --argument='(record {})'  --specified-id 222iv-iiaaa-aaaak-qdyla-cai

  dfx deploy xrc --specified-id uf6dk-hyaaa-aaaaq-qaaaq-cai

  dfx generate
}

function deploy() {

  deploy_aegis
  deploy_ledgers
  # deploy_minters
  # deploy_kyt
  deploy_canisters

}

function topUp(){

dfx identity use minter 

dfx canister call icp_ledger icrc1_transfer  '(record {  to = record {owner=principal "up5qv-6itp6-z5fuj-kfq2a-qohj4-ckibb-lq6tt-34j2c-i2d27-3gqlm-pqe";}; amount= 10_000_000_000 })' 
                                         
dfx canister call icp_ledger icrc1_transfer  '(record {  to = record {owner=principal "akm3b-xt34z-vnaos-o667b-jrxjr-3a4ao-juwz5-7qdpz-hxnks-yfh2i-fae";}; amount= 10_000_000_000 })' 



dfx canister call ckbtc_ledger icrc1_transfer  '(record {  to = record {owner=principal "up5qv-6itp6-z5fuj-kfq2a-qohj4-ckibb-lq6tt-34j2c-i2d27-3gqlm-pqe";}; amount= 10_000_000_000 })' 
                                         
dfx canister call ckbtc_ledger icrc1_transfer  '(record {  to = record {owner=principal "akm3b-xt34z-vnaos-o667b-jrxjr-3a4ao-juwz5-7qdpz-hxnks-yfh2i-fae";}; amount= 10_000_000_000 })' 




dfx canister call cketh_ledger icrc1_transfer  '(record {  to = record {owner=principal "up5qv-6itp6-z5fuj-kfq2a-qohj4-ckibb-lq6tt-34j2c-i2d27-3gqlm-pqe";}; amount= 1_000_000_000_000_000_000 })' 
                                         
dfx canister call cketh_ledger icrc1_transfer  '(record {  to = record {owner=principal "akm3b-xt34z-vnaos-o667b-jrxjr-3a4ao-juwz5-7qdpz-hxnks-yfh2i-fae";}; amount= 1_000_000_000_000_000_000 })' 


dfx canister call aegis_ledger icrc1_transfer  '(record {  to = record {owner=principal "up5qv-6itp6-z5fuj-kfq2a-qohj4-ckibb-lq6tt-34j2c-i2d27-3gqlm-pqe";}; amount= 10_000_000_000 })' 
                                         
dfx canister call aegis_ledger icrc1_transfer  '(record {  to = record {owner=principal "akm3b-xt34z-vnaos-o667b-jrxjr-3a4ao-juwz5-7qdpz-hxnks-yfh2i-fae";}; amount= 10_000_000_000 })' 


dfx identity use default 

}
function main() {

  case $1 in

  "deploy")
    deploy
    
    topUp
    ;;

  *) echo "Invalid Arguments!" ;;

  esac
}

main "$1"
