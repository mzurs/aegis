import { ActorSubclass } from "@dfinity/agent";
import { _CKBTC_LEDGER, _CKETH_LEDGER, _ICP_LEDGER, } from "../../exports";
import { CANISTERS_NAME_NO_PIC } from "../../non-pic/constants";
import { createCanisterActor } from "../../non-pic/setup-canister";
import { Principal } from "@dfinity/principal";
import { Account, SubAccount } from "../../../../declarations/icp_ledger/icp_ledger.did";

export async function balance(
    asset:
        | CANISTERS_NAME_NO_PIC.ICP_LEDGER
        | CANISTERS_NAME_NO_PIC.CKBTC_LEDGER
        | CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
    principal: Principal,
    subaccount?: SubAccount
): Promise<bigint> {
    let actor: ActorSubclass<_ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER> =
        createCanisterActor(asset) as unknown as ActorSubclass<
            _ICP_LEDGER | _CKBTC_LEDGER | _CKETH_LEDGER
        >;
    let args: Account = {
        owner: principal,
        subaccount:  [],
    };

    return await actor.icrc1_balance_of(args);
}
