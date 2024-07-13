import { ActorSubclass, Identity } from "@dfinity/agent";
import { _INSURANCE } from "../../exports";
import {
  BuyInsuranceRes,
  InsuranceAssets,
  InsuranceContractInitArgs,
  InsuranceInitRes,
  InsuranceRewardsMultiplier,
} from "../../../../declarations/insurance/insurance.did";
import { currentTimePlusExtraMinutesInNanoseconds } from "../../non-pic/utils";
import { createCanisterActor } from "../../non-pic/setup-canister";
import { CANISTERS_NAME_NO_PIC } from "../../non-pic/constants";

type CreateInsuranceArgs = {
  identity?: Identity;
  amount?: bigint;
  multiplier?: InsuranceRewardsMultiplier;
  expiry_in?: number;
  min_premium_amount?: bigint;
  is_muliple_seller_allowed?: boolean;
  insurance_asset?: InsuranceAssets;
  inflation_target?: number;
  min_share_amount?: [bigint];
};
export async function createInsuranceContract(
  crateArgs: CreateInsuranceArgs
): Promise<InsuranceInitRes> {
  let {
    identity,
    expiry_in,
    multiplier,
    min_premium_amount,
    inflation_target,
    insurance_asset,
    is_muliple_seller_allowed,
    amount,
    min_share_amount,
  } = crateArgs;
  let actor: ActorSubclass<_INSURANCE> = createCanisterActor(
    CANISTERS_NAME_NO_PIC.INSURANCE,
    identity
  ) as ActorSubclass<_INSURANCE>;

  let expiry_date = currentTimePlusExtraMinutesInNanoseconds(expiry_in ?? 2);
  // console.log("🚀 ~ expiry_date:", expiry_date);

  let args: InsuranceContractInitArgs = {
    multiplier: multiplier ?? {
      M2X: null,
    },
    title: "",
    min_premium_amount: min_premium_amount ?? 10_000_000n,
    description: "",
    is_muliple_seller_allowed: is_muliple_seller_allowed ?? true,
    expiry_date,
    category: {
      InflationBasedInsurance: {
        country: {
          US: null,
        },
        target_expiry: expiry_date,
        inflation_target: inflation_target ?? 3.0,
      },
    },
    insurance_asset: insurance_asset ?? {
      ICP: null,
    },
    amount: amount ?? 100_000_000n,
    min_share_amount: min_share_amount ?? [0n],
  };
  return await actor.create_insurance_contract(args);
}
