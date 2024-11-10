import { Actor, PocketIc } from "@hadronous/pic";
import {
  _CKBTC_LEDGER,
  _CKBTC_MINTER,
  _CKETH_LEDGER,
  _CKETH_MINTER,
  _ICP_LEDGER,
  _AEGIS_LEDGER,
  _MAIN,
  _OPTIONS,
} from "../../utils/exports";
import { Identity } from "@dfinity/agent";
import { createIdentityFromSeed, setupCanister } from "../../utils/configs";
import {
  CANISTER_IDS_MAP,
  CANISTERS_NAME,
  OPTIONS_WASM_PATH,
} from "../../utils/constants";
import {
  OptionsAssetsIcrc,
  Result,
} from "../../../declarations/options/options.did";

describe("======================================Options Canister Unit Testing======================================", () => {
  let pic: PocketIc;
  let minter: Identity;
  let user1: Identity;
  let user2: Identity;
  let user3: Identity;

  let optionsActor: Actor<_OPTIONS>;

  let icrcAssets: OptionsAssetsIcrc[] = [
    {
      CKBTC: null,
    },
    {
      CKETH: null,
    },
  ];
  beforeAll(async () => {
    pic = await PocketIc.create(process.env.PIC_URL, {
      nns: true,
      fiduciary: true,
      bitcoin: true,
      system: 1,
    });

    await pic.resetTime();

    minter = createIdentityFromSeed("minter");
    user1 = createIdentityFromSeed("user1");
    user2 = createIdentityFromSeed("user2");
    user3 = createIdentityFromSeed("user3");

    optionsActor = (await setupCanister(
      pic,
      minter.getPrincipal(),
      OPTIONS_WASM_PATH,
      CANISTERS_NAME.OPTIONS
    )) as Actor<_OPTIONS>;

    optionsActor.setIdentity(minter);

    optionsActor.set_ledger_canister_id(
      { CKBTC: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_LEDGER)!
    );

    optionsActor.set_ledger_canister_id(
      { CKETH: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_LEDGER)!
    );
  });

  afterAll(async () => {
    await pic.tearDown();
  });

  it("List of all Option Contracts should be 0", async () => {
    let res = await optionsActor.get_all_options();

    expect(res.length).toBe(0);
  });

  it("Create an ICRC 2 Call Option Contract of 1 CKBTC", async () => {
    let res: Result = await optionsActor.create_icrc_options(
      { CKBTC: null },
      {
        asset: {
          ICRC: {
            CKBTC: null,
          },
        },
        options_type: {
          CALL: null,
        },
        contract_state: {
          OPEN: null,
        },
        asset_amount: 1n,
        contract_expiry: BigInt(Date.now() + 30 * 60000),
      }
    );
    let res1: Result = await optionsActor.create_icrc_options(
      { CKBTC: null },
      {
        asset: {
          ICRC: {
            CKBTC: null,
          },
        },
        options_type: {
          CALL: null,
        },
        contract_state: {
          OPEN: null,
        },
        asset_amount: 1n,
        contract_expiry: BigInt(Date.now() + 30 * 60000),
      }
    );

    expect(res).toMatchObject({
      Ok: expect.stringContaining("Option is successfully created with Id"),
    });

    expect(res1).toMatchObject({
      Ok: expect.stringContaining("Option is successfully created with Id"),
    });
  });

  it("Create an ICRC Call Option Contract of 1 CKETH", async () => {
    let res: Result = await optionsActor.create_icrc_options(
      { CKETH: null },
      {
        asset: {
          ICRC: {
            CKETH: null,
          },
        },
        options_type: {
          CALL: null,
        },
        contract_state: {
          OPEN: null,
        },
        asset_amount: 1n,
        contract_expiry: BigInt(Date.now() + 30 * 60000),
      }
    );

    expect(res).toMatchObject({
      Ok: expect.stringContaining("Option is successfully created with Id"),
    });
  });

  it("List of all Option Contracts should be 2", async () => {
    let res = await optionsActor.get_all_options();
    expect(res.length).toBe(3);
  });

  it("Create 5 ICRC PUT Option Contracts of 1 CKETH and 1 CKBTC", async () => {
    for (const i of icrcAssets) {
      let j = 0;

      while (j < 5) {
        let res: Result = await optionsActor.create_icrc_options(i, {
          asset: {
            ICRC: i,
          },
          options_type: {
            PUT: null,
          },
          contract_state: {
            OPEN: null,
          },
          asset_amount: 1n,
          contract_expiry: BigInt(Date.now() + 30 * 60000),
        });

        expect(res).toMatchObject({
          Ok: expect.stringContaining("Option is successfully created with Id"),
        });

        j++;
      }
    }
  });

  it("List of all Option Contracts should be 13", async () => {
    let res = await optionsActor.get_all_options();
    expect(res.length).toBe(13);
  });

  it("List of all Call Option Contracts should be 2", async () => {
    let res = await optionsActor.get_call_options_by_asset({ CKBTC: null });
    console.log(": ----------");
    console.dir(res, { depth: null });
    console.log(": ----------");
    expect(res.length).toBe(2);
  });
});
