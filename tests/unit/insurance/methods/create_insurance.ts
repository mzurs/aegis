import { ActorSubclass } from "@dfinity/agent";
import { _INSURANCE } from "../../../utils/exports";
import {
  BuyInsuranceRes,
  InsuranceAssets,
  InsuranceContractInitArgs,
  InsuranceRewardsMultiplier,
} from "../../../../declarations/insurance/insurance.did";
import { currentTimePlusExtraMinutesInNanoseconds } from "../../../utils/non-pic/utils";

export async function createInsuranceContract(
  actor: ActorSubclass<_INSURANCE>,
  multiplier?: InsuranceRewardsMultiplier,
  expiry_in?: number,
  min_premium_amount?: bigint,
  is_muliple_seller_allowed?: boolean,
  insurance_asset?: InsuranceAssets,
  amount?: bigint,
  inflation_target?: number
): Promise<BuyInsuranceRes> {
  let expiry_date = currentTimePlusExtraMinutesInNanoseconds(expiry_in ?? 1);

  let args: InsuranceContractInitArgs = {
    multiplier: multiplier ?? {
      M2X: null,
    },
    title: "",
    min_premium_amount: min_premium_amount ?? 0n,
    description: "",
    is_muliple_seller_allowed: is_muliple_seller_allowed ?? false,
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
    amount: amount ?? 10_000_000n,
  };
  return await actor.create_insurance_contract(args);
}
