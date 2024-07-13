import { ActorSubclass, Identity } from "@dfinity/agent";
import {
  BuyInsuranceRes,
  SellInsuranceArgs,
} from "../../../../declarations/insurance/insurance.did";
import { _INSURANCE } from "../../exports";
import { createCanisterActor } from "../../non-pic/setup-canister";
import { CANISTERS_NAME_NO_PIC } from "../../non-pic/constants";

type SellInsuranceArgsTest = {
  identity?: Identity;
  insurance_id: number;
  amount?: bigint;
};
export async function sellInsuranceContract(
  sellInsuranceArgsTest: SellInsuranceArgsTest): Promise<BuyInsuranceRes> {
  let actor: ActorSubclass<_INSURANCE> = createCanisterActor(
    CANISTERS_NAME_NO_PIC.INSURANCE,
    sellInsuranceArgsTest.identity
  ) as ActorSubclass<_INSURANCE>;

  let args: SellInsuranceArgs = {
    insurance_id: sellInsuranceArgsTest.insurance_id,
    amount: sellInsuranceArgsTest.amount ?? 10_000_000n,
  };

  return actor.sell_insurance_contract(args);
}
