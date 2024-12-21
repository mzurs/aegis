import { ActorSubclass, Identity } from "@dfinity/agent";
import {
  _CKBTC_LEDGER,
  _CKETH_LEDGER,
  _CKUSDT_LEDGER,
  _ICP_LEDGER,
  _INSURANCE,
} from "../../exports";
import { Account } from "@dfinity/ledger-icp";
import {
  CANISTER_IDS_MAP_NO_PIC,
  CANISTERS_NAME_NO_PIC,
} from "../../non-pic/constants";
import { ApproveArgs } from "@dfinity/ledger-icp/dist/candid/ledger";
import { createCanisterActor } from "../../non-pic/setup-canister";

export async function approveTokens(
  identity: Identity,
  asset:
    | CANISTERS_NAME_NO_PIC.ICP_LEDGER
    | CANISTERS_NAME_NO_PIC.CKBTC_LEDGER
    | CANISTERS_NAME_NO_PIC.CKETH_LEDGER
    | CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
  amount?: bigint,
  spender?: Account
) {
  let actor: ActorSubclass<
    _ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER | _CKUSDT_LEDGER
  > = createCanisterActor(asset, identity) as unknown as ActorSubclass<
    _ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER | _CKUSDT_LEDGER
  >;

  let args: ApproveArgs = {
    fee: [],
    memo: [],
    from_subaccount: [],
    created_at_time: [],
    amount: amount ?? 1_000_000_000n,
    expected_allowance: [],
    expires_at: [],
    spender: spender ?? {
      owner: CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.INSURANCE)!,
      subaccount: [],
    },
  };

  console.log("Approve Args", args);

  actor.icrc2_approve(args);
}
