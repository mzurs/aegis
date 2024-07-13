import { ActorSubclass, Identity } from "@dfinity/agent";
import {
  BuyInsuranceArgs,
  BuyInsuranceRes,
} from "../../../../declarations/insurance/insurance.did";
import { _INSURANCE } from "../../exports";
import { CANISTERS_NAME_NO_PIC } from "../../non-pic/constants";
import { createCanisterActor } from "../../non-pic/setup-canister";

export async function buyInsuranceContract(
  identity: Identity,
  insurance_id: number,
  premium?: bigint
): Promise<BuyInsuranceRes> {
  let actor: ActorSubclass<_INSURANCE> = createCanisterActor(
    CANISTERS_NAME_NO_PIC.INSURANCE,
    identity
  ) as ActorSubclass<_INSURANCE>;

  let args: BuyInsuranceArgs = {
    premium: premium ?? 10_000_000n,
    insurance_id,
  };
  return actor.buy_insurance_contract(args);
}
