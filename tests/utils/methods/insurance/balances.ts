import { ActorSubclass, Identity } from "@dfinity/agent";
import { _INSURANCE } from "../../exports";
import { CANISTERS_NAME_NO_PIC } from "../../non-pic/constants";
import { createCanisterActor } from "../../non-pic/setup-canister";

export async function getPoolBalanceByInsuranceId(insurance_id: number, identity?: Identity): Promise<bigint> {
    let actor: ActorSubclass<_INSURANCE> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.INSURANCE,
        identity
    ) as ActorSubclass<_INSURANCE>;

    return await actor.get_pool_balance_by_insurance_id(insurance_id)

}

export async function getPremiumBalanceByInsuranceId(insurance_id: number, identity?: Identity): Promise<bigint> {
    let actor: ActorSubclass<_INSURANCE> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.INSURANCE,
        identity
    ) as ActorSubclass<_INSURANCE>;

    return await actor.get_premium_pool_balance_by_insurance_id(insurance_id)

}