import { execSync } from "child_process";
import { CANISTER_IDS_MAP_NO_PIC, CANISTERS_NAME_NO_PIC } from "./constants";
import { Principal } from "@dfinity/principal";

export async function mintTokens(
  args: CANISTERS_NAME_NO_PIC,
  to: string,
  amount: bigint,
  subaccount?: Uint8Array
) {
  execSync(`   dfx identity use minter  `, {
    stdio: "inherit",
  });
  execSync(
    `   dfx identity use minter && dfx canister call ${CANISTER_IDS_MAP_NO_PIC.get(
      args
    )} icrc1_transfer  '(record {  to = record {owner=principal "${to}" ;}; amount= ${amount} })'`,
    {
      stdio: "inherit",
    }
  );
  execSync(`   dfx identity use default  `, {
    stdio: "inherit",
  });
}
