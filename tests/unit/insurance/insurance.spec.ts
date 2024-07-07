import { Actor, PocketIc, PocketIcServer } from "@hadronous/pic";
import {
  _ACCOUNTS,
  _CKBTC_LEDGER,
  _CKBTC_MINTER,
  _CKETH_LEDGER,
  _CKETH_MINTER,
  _ICP_LEDGER,
  _INSURANCE,
  _KYT,
} from "../../utils/exports";
import { ActorSubclass, Identity } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import {
  CANISTER_IDS_MAP,
  CANISTERS_NAME,
  CKBTC_LEDGER_WASM_PATH,
  CKBTC_MINTER_WASM_PATH,
  CKETH_LEDGER_WASM_PATH,
  ICP_LEDGER_WASM_PATH,
  INSURANCE_WASM_PATH,
  KYT_WASM_PATH,
} from "../../utils/constants";
import { createIdentityFromSeed, setupCanister } from "../../utils/configs";
import {
  createCanisterActor,
  delete_canisters,
  install_canister,
} from "../../utils/non-pic/setup-canister";
import {
  CANISTER_IDS_MAP_NO_PIC,
  CANISTERS_NAME_NO_PIC,
} from "../../utils/non-pic/constants";
import { InsuranceContractInitArgs } from "../../../declarations/insurance/insurance.did";
import { currentTimePlusExtraMinutesInNanoseconds } from "../../utils/non-pic/utils";
import { createInsuranceContract } from "./methods/create_insurance";

describe("\n================================= Insurance Canister Unit Testing =================================\n", () => {
  let minter: Identity;
  let user: Identity;
  let insuranceActor: ActorSubclass<_INSURANCE>;
  let icpLedgerActor: Actor<_ICP_LEDGER>;
  let ckethLedgerActor: Actor<_CKETH_LEDGER>;
  let ckbtcLedgerActor: Actor<_CKBTC_LEDGER>;
  let ckbtcMinterActor: Actor<_CKBTC_MINTER>;
  //   let ckethMinterActor: Actor<_CKETH_MINTER>;

  beforeAll(async () => {
    await install_canister(CANISTERS_NAME_NO_PIC.INSURANCE);

    // Generate new Identities
    minter = createIdentityFromSeed("minter");
    user = createIdentityFromSeed("user");

    insuranceActor = createCanisterActor(
      CANISTERS_NAME_NO_PIC.INSURANCE
    ) as ActorSubclass<_INSURANCE>;

    // insuranceActor.setIdentity(minter);

    // set the ledger ids in the Account Canister
    insuranceActor.set_ledger_canister_id(
      { ICP: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.ICP_LEDGER)!
    );

    insuranceActor.set_ledger_canister_id(
      { CKBTC: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_LEDGER)!
    );

    insuranceActor.set_ledger_canister_id(
      { CKETH: null },

      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_LEDGER)!
    );
  }, 30_000);

  describe("Inflation Data Testing", () => {
    it("Inflation should be greater than 1", async () => {
      let inflation_point = await insuranceActor.get_inflation_data(
        { US: null },

        ["2024-06-16"]
      );

      let point: number =
        "Ok" in inflation_point
          ? parseFloat(inflation_point.Ok.toFixed(2))
          : 0.0;

      expect(point).toBe(3.25);
    }, 30_000);
  });

  describe("Insurance Contract Testing", () => {
    it("List of Insurance Active Contract List Should be 0", async () => {
      let list = await insuranceActor.list_insurance_contract();
      expect(list.length).toBe(0);
    });

    it("Insurance Contract With Anonymous Principal can not be created", async () => {
      let res;
      try {
        res = await createInsuranceContract(insuranceActor);
      } catch (error) {
        expect(error);
      }
    });

    it("Failed to create Insurance using User Identity because of Transfer Error", async () => {
      insuranceActor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.INSURANCE,
        user
      ) as ActorSubclass<_INSURANCE>;

      let res = await createInsuranceContract(insuranceActor);
      expect(res).toHaveProperty("TransferError");
    });
  });
  afterAll(async () => {
    console.log("Deleting Canister...");
    let delCanisters: CANISTERS_NAME_NO_PIC[] = [
      CANISTERS_NAME_NO_PIC.INSURANCE,
    ];

    await delete_canisters(delCanisters);
  });
});
