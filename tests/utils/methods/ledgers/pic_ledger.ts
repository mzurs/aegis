import {
  _AEGIS_LEDGER,
  _CKBTC_LEDGER,
  _CKETH_LEDGER,
  _ICP_LEDGER,
  _INSURANCE,
} from "../../exports";
import { Account } from "@dfinity/ledger-icp";
import {
  ApproveArgs,
  TransferArg,
} from "@dfinity/ledger-icp/dist/candid/ledger";
import { Principal } from "@dfinity/principal";
import { Actor } from "@hadronous/pic";
import { TransferResult } from "../../../../declarations/aegis_ledger/aegis_ledger.did";
import { CANISTER_IDS_MAP, CANISTERS_NAME } from "../../constants";

export async function transferTokens(
  actor: Actor<_ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER | _AEGIS_LEDGER>,
  amount: bigint,
  to: Principal,
  subaccount?: Uint8Array | number[]
): Promise<TransferResult> {
  let args: TransferArg = {
    to: {
      owner: to,
      subaccount: subaccount ? [subaccount!] : [],
    },
    fee: [],
    memo: [],
    from_subaccount: [],
    created_at_time: [],
    amount,
  };

  return actor.icrc1_transfer(args);
}

export async function balance(
  actor: Actor<_ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER | _AEGIS_LEDGER>,
  principal: Principal,
  subaccount?: Uint8Array | number[]
): Promise<bigint> {
  let args: Account = {
    owner: principal,
    subaccount: subaccount ? [subaccount!] : [],
  };

  return await actor.icrc1_balance_of(args);
}

export async function approveTokens(
  actor: Actor<_ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER | _AEGIS_LEDGER>,
  amount?: bigint,
  spender?: Account
) {
  let args: ApproveArgs = {
    fee: [],
    memo: [],
    from_subaccount: [],
    created_at_time: [],
    amount: amount ?? 1_000_000_000n,
    expected_allowance: [],
    expires_at: [],
    spender: spender ?? {
      owner: CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
      subaccount: [],
    },
  };

  actor.icrc2_approve(args);
}
