import { ActorSubclass, Identity } from "@dfinity/agent";
import { parseEther } from "ethers";
import { CreateOptionArgs } from "../../../declarations/options/options.did";
import { createIdentityFromSeed } from "../../utils/configs";
import {
  _OPTIONS
} from "../../utils/exports";
import { humanToE8s } from "../../utils/helpers";
import { approveTokens } from "../../utils/methods/ledgers/approve";
import { balance } from "../../utils/methods/ledgers/balance";
import {
  CANISTER_IDS_MAP_NO_PIC,
  CANISTERS_NAME_NO_PIC,
} from "../../utils/non-pic/constants";
import { mintTokens } from "../../utils/non-pic/mint_to_account";
import { createCanisterActor, delete_all_canisters, install_all_canisters } from "../../utils/non-pic/setup-canister";

describe("Options Canister Integration Testing", () => {
  let user: Identity;

  user = createIdentityFromSeed(
    (Math.random() * Math.random() + Math.random()).toString()
  );

  beforeAll(async () => {
    await install_all_canisters();

    await mintTokens(
      CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
      user.getPrincipal().toText(),
      humanToE8s(5)
    );

    await mintTokens(
      CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
      user.getPrincipal().toText(),
      parseEther("1")
    );

    await mintTokens(
      CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
      user.getPrincipal().toText(),
      parseEther("5")
    );

    let bal = await balance(
      CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
      user.getPrincipal()
    );

    console.log(": ----------");
    console.log(": bal", bal);
    console.log(": ----------");
  },60000);

  afterAll(async () => {
    await delete_all_canisters();
  });

  describe("Options Main Methods 1", () => {
    it("Option Creation with greater timestamp of offer duration should fail", async () => {
      let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user
      ) as unknown as ActorSubclass<_OPTIONS>;

      actor.set_ledger_canister_id(
        { CKUSDT: null },
        CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER)!
      );

      let res = await actor.create_icrc_options(
        {
          CKBTC: null,
        },
        {
          asset: { ICRC: { CKBTC: null } },
          asset_amount: humanToE8s(0.000001),
          strike_price: 110_000n,
          contract_expiry: BigInt(
            (Date.now() + 10 * 24 * 60 * 60 * 1000) * 1000000 // 10 days after
          ),
          use_exchange_account: false,
          offer_duration: BigInt(
            (Date.now() + 11 * 24 * 60 * 60 * 1000) * 1000000 // 11 days after
          ),
          contract_state: { OPEN: null },
          options_type: { CALL: null },
        } as CreateOptionArgs
      );

      console.log(": ----------");
      console.log(": res", res);
      console.log(": ----------");

      expect(res).toMatchObject({
        Err: expect.stringContaining(
          "Offer Duration should be within Contract Expiry"
        ),
      });
    });

    it("Create CALL Option Contract of 0.000001 BTC", async () => {
      let approveRes = await approveTokens(
        user,
        CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
        humanToE8s(100),
        {
          owner: CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.OPTIONS)!,
          subaccount: [],
        }
      );

      let bal = await balance(
        CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
        user.getPrincipal()
      );

      console.log(": ----------");
      console.log(": bal", bal);
      console.log(": ----------");

      let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user
      ) as unknown as ActorSubclass<_OPTIONS>;

      actor.set_ledger_canister_id(
        { CKUSDT: null },
        CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER)!
      );

      let res = await actor.create_icrc_options(
        {
          CKBTC: null,
        },
        {
          asset: { ICRC: { CKBTC: null } },
          asset_amount: humanToE8s(0.000001),
          strike_price: 110_000n,
          contract_expiry: BigInt(
            (Date.now() + 10 * 24 * 60 * 60 * 1000) * 1000000 // 10 days after
          ),
          use_exchange_account: false,
          offer_duration: BigInt(
            (Date.now() + 5 * 24 * 60 * 60 * 1000) * 1000000 // 5 days after
          ),
          contract_state: { OPEN: null },
          options_type: { CALL: null },
        } as CreateOptionArgs
      );

      console.log(": ----------");
      console.log(": res", res);
      console.log(": ----------");

      expect(res).toMatchObject({
        Ok: expect.stringContaining("Option is successfully created with Id"),
      });
    });

    it("Create PUT Option Contract of 0.000001 BTC", async () => {
      let approveRes = await approveTokens(
        user,
        CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        parseEther("10"),
        {
          owner: CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.OPTIONS)!,
          subaccount: [],
        }
      );

      let bal = await balance(
        CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        user.getPrincipal()
      );

      console.log(": ----------");
      console.log(": bal", bal);
      console.log(": ----------");

      let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user
      ) as unknown as ActorSubclass<_OPTIONS>;

      let id = CANISTER_IDS_MAP_NO_PIC.get(
        CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER
      )!.toText();
      console.log(": --------");
      console.log(": id", id);
      console.log(": --------");
      actor.set_ledger_canister_id(
        { CKUSDT: null },
        CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER)!
      );

      let res = await actor.create_icrc_options(
        {
          CKBTC: null,
        },
        {
          asset: { ICRC: { CKBTC: null } },
          asset_amount: humanToE8s(0.000001),
          strike_price: 110_000n,
          contract_expiry: BigInt(
            (Date.now() + 10 * 24 * 60 * 60 * 1000) * 1000000 // 10 days after
          ),
          use_exchange_account: false,
          offer_duration: BigInt(
            (Date.now() + 5 * 24 * 60 * 60 * 1000) * 1000000 // 5 days after
          ),
          contract_state: { OPEN: null },
          options_type: { PUT: null },
        } as CreateOptionArgs
      );

      expect(res).toMatchObject({
        Ok: expect.stringContaining("Option is successfully created with Id"),
      });
    });

    //   it("Create CALL Option Contract of 0.0001 ETH", async () => {
    //     let approveRes = await approveTokens(
    //       user,
    //       CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
    //       parseEther("1"),
    //       {
    //         owner: CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.OPTIONS)!,
    //         subaccount: [],
    //       }
    //     );

    //     let bal = await balance(
    //       CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
    //       user.getPrincipal()
    //     );

    //     console.log(": ----------");
    //     console.log(": eth bal", bal);
    //     console.log(": ----------");

    //     let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
    //       CANISTERS_NAME_NO_PIC.OPTIONS,
    //       user
    //     ) as unknown as ActorSubclass<_OPTIONS>;

    //     actor.set_ledger_canister_id(
    //       { CKETH: null },
    //       CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKETH_LEDGER)!
    //     );

    //     let res = await actor.create_icrc_options(
    //       {
    //         CKETH: null,
    //       },
    //       {
    //         asset: { ICRC: { CKETH: null } },
    //         asset_amount: parseEther("0.0001"),
    //         strike_price: 110_000n,
    //         contract_expiry: BigInt(
    //           (Date.now() + 10 * 24 * 60 * 60 * 1000) * 1000000 // 10 days after
    //         ),
    //         use_exchange_account: false,
    //         offer_duration: BigInt(
    //           (Date.now() + 5 * 24 * 60 * 60 * 1000) * 1000000 // 5 days after
    //         ),
    //         contract_state: { OFFER: null },
    //         options_type: { CALL: null },
    //       } as CreateOptionArgs
    //     );

    //     console.log(": ----------");
    //     console.log(": res", res);
    //     console.log(": ----------");

    //     expect(res).toMatchObject({
    //       Ok: expect.stringContaining("Option is successfully created with Id"),
    //     });
    //   });

    //   it("Create PUT Option Contract of 0.0001 ETH", async () => {
    //     let approveRes = await approveTokens(
    //       user,
    //       CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
    //       parseEther("10"),
    //       {
    //         owner: CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.OPTIONS)!,
    //         subaccount: [],
    //       }
    //     );

    //     let bal = await balance(
    //       CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
    //       user.getPrincipal()
    //     );

    //     console.log(": ----------");
    //     console.log(": eth bal", bal);
    //     console.log(": ----------");

    //     let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
    //       CANISTERS_NAME_NO_PIC.OPTIONS,
    //       user
    //     ) as unknown as ActorSubclass<_OPTIONS>;

    //     let id = CANISTER_IDS_MAP_NO_PIC.get(
    //       CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER
    //     )!.toText();
    //     console.log(": --------");
    //     console.log(": id", id);
    //     console.log(": --------");
    //     actor.set_ledger_canister_id(
    //       { CKUSDT: null },
    //       CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER)!
    //     );

    //     let res = await actor.create_icrc_options(
    //       {
    //         CKETH: null,
    //       },
    //       {
    //         asset: { ICRC: { CKETH: null } },
    //         asset_amount: parseEther("0.0001"),
    //         strike_price: 110_000n,
    //         contract_expiry: BigInt(
    //           (Date.now() + 10 * 24 * 60 * 60 * 1000) * 1000000 // 10 days after
    //         ),
    //         use_exchange_account: false,
    //         offer_duration: BigInt(
    //           (Date.now() + 5 * 24 * 60 * 60 * 1000) * 1000000 // 5 days after
    //         ),
    //         contract_state: { OPEN: null },
    //         options_type: { PUT: null },
    //       } as CreateOptionArgs
    //     );

    //     expect(res).toMatchObject({
    //       Ok: expect.stringContaining("Option is successfully created with Id"),
    //     });
    //   });
  });
});
