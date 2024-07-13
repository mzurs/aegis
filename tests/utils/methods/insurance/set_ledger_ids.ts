import { ActorSubclass, Identity } from "@dfinity/agent";
import { _INSURANCE } from "../../exports";
import {
  CANISTER_IDS_MAP_NO_PIC,
  CANISTERS_NAME_NO_PIC,
} from "../../non-pic/constants";
import { createCanisterActor } from "../../non-pic/setup-canister";

export async function set_ledger_ids(identity?: Identity) {
  let actor = createCanisterActor(
    CANISTERS_NAME_NO_PIC.INSURANCE,
    identity
  ) as ActorSubclass<_INSURANCE>;

  // set the ledger ids in the Insurance Canister
  actor.set_ledger_canister_id(
    { ICP: null },
    CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.ICP_LEDGER)!
  );

  actor.set_ledger_canister_id(
    { CKBTC: null },
    CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKBTC_LEDGER)!
  );

  actor.set_ledger_canister_id(
    { CKETH: null },

    CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKETH_LEDGER)!
  );
}
