import { ActorSubclass } from "@dfinity/agent";
import { _CKBTC_LEDGER, _CKETH_LEDGER, _ICP_LEDGER } from "../../exports";
import { CANISTERS_NAME_NO_PIC } from "../../non-pic/constants";
import { createCanisterActor } from "../../non-pic/setup-canister";

export async function fee(
  asset:
    | CANISTERS_NAME_NO_PIC.ICP_LEDGER
    | CANISTERS_NAME_NO_PIC.CKBTC_LEDGER
    | CANISTERS_NAME_NO_PIC.CKETH_LEDGER
): Promise<bigint> {
  let actor: ActorSubclass<_ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER> =
    createCanisterActor(asset) as unknown as ActorSubclass<
      _ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER
    >;

  return await actor.icrc1_fee();
}
