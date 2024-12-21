import { ActorSubclass } from "@dfinity/agent";
import { parseEther } from "ethers";
import { createIdentityFromSeed, wait } from "../../utils/configs";
import { _OPTIONS } from "../../utils/exports";
import { approveTokens } from "../../utils/methods/ledgers/approve";
import {
  CANISTERS_NAME_NO_PIC,
  CANISTER_IDS_MAP_NO_PIC,
} from "../../utils/non-pic/constants";
import { mintTokens } from "../../utils/non-pic/mint_to_account";
import {
  createCanisterActor,
  delete_all_canisters,
  install_all_canisters,
} from "../../utils/non-pic/setup-canister";
import { balance } from "../../utils/methods/ledgers/balance";
import { BASE_OF_XRC } from "../../utils/constants";
import { humanToE8s } from "../../utils/helpers";

describe("Options Canister Integration Testing", () => {
  let user1 = createIdentityFromSeed(
    (Math.random() * Math.random() + Math.random()).toString()
  );
  let user2 = createIdentityFromSeed(
    (Math.random() * Math.random() + Math.random()).toString()
  );

  let user3 = createIdentityFromSeed(
    (Math.random() * Math.random() + Math.random()).toString()
  );

  beforeAll(async () => {
    await install_all_canisters();

    [user1, user2, user3].forEach(async (user) => {
      await mintTokens(
        CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
        user.getPrincipal().toText(),
        humanToE8s(2)
      );

      await mintTokens(
        CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
        user.getPrincipal().toText(),
        parseEther("0.00002")
      );

      await mintTokens(
        CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        user.getPrincipal().toText(),
        parseEther("2")
      );
    });

    [user1, user2, user3].forEach(async (id) => {
      await approveTokens(
        id,
        CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
        humanToE8s(10),
        {
          owner: CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.OPTIONS)!,
          subaccount: [],
        }
      );

      await approveTokens(
        id,
        CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
        parseEther("10"),
        {
          owner: CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.OPTIONS)!,
          subaccount: [],
        }
      );
      await approveTokens(
        id,
        CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        parseEther("10"),
        {
          owner: CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.OPTIONS)!,
          subaccount: [],
        }
      );
    });

    let actor = createCanisterActor(
      CANISTERS_NAME_NO_PIC.OPTIONS,
      user1
    ) as unknown as ActorSubclass<_OPTIONS>;

    actor.set_ledger_canister_id(
      { CKUSDT: null },
      CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER)!
    );

    actor.set_ledger_canister_id(
      { CKETH: null },
      CANISTER_IDS_MAP_NO_PIC.get(CANISTERS_NAME_NO_PIC.CKETH_LEDGER)!
    );
  });

  afterAll(async () => {
    await delete_all_canisters();
  });

  describe("Options Main Methods 6", () => {
    let optionId_1: bigint;
    let optionId_2: bigint;

    it("Check Balances of user 1 and user 2", async () => {
      let balances = await (async (): Promise<[bigint, bigint, bigint][]> => {
        const ledgers: (
          | CANISTERS_NAME_NO_PIC.CKBTC_LEDGER
          | CANISTERS_NAME_NO_PIC.CKETH_LEDGER
          | CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER
        )[] = [
          CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
          CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
          CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        ];

        const results = await Promise.all(
          [user1, user2].map(async (user) => {
            const balances = await Promise.all(
              ledgers.map(
                async (ledger) => await balance(ledger, user.getPrincipal())
              )
            );

            return balances as [bigint, bigint, bigint];
          })
        );

        return results;
      })();

      // For user 1
      expect(balances[0][0]).toBe(humanToE8s(2));
      expect(balances[0][1]).toBe(parseEther("0.00002"));
      expect(balances[0][2]).toBe(parseEther("2"));

      // For user 2
      expect(balances[1][0]).toBe(humanToE8s(2));
      expect(balances[1][1]).toBe(parseEther("0.00002"));
      expect(balances[1][2]).toBe(parseEther("2"));
    });

    it("Create CALL Option Contract of  and 0.00002 CKBTC from user1 ", async () => {
      let actor1 = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user1
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res1 = await actor1.create_icrc_options(
        {
          CKBTC: null,
        },
        {
          asset: { ICRC: { CKBTC: null } },
          asset_amount: humanToE8s(0.00002),
          strike_price: 160_000n * BASE_OF_XRC,
          contract_expiry: BigInt(
            (Date.now() + 1 * 60 * 1000) * 1000000 // 1 minute days after
          ),
          use_exchange_account: false,
          offer_duration: BigInt(
            (Date.now() + 0.5 * 60 * 1000) * 1000000 // 1/2 minutes days after
          ),
          contract_state: { OFFER: null },
          options_type: { CALL: null },
        }
      );

      expect(res1).toMatchObject({
        Ok: expect.stringContaining("Option is successfully created with Id"),
      });

      // Use a regular expression to extract the number
      let string1 = res1 as { Ok: string };
      const match1 = string1.Ok.match(/\d+/);

      if (match1) {
        optionId_1 = BigInt(match1[0]); // Convert the matched string to a number (BigInt if it's a large number)
        console.log("Extracted Number:", optionId_1);
      } else {
        console.log("No number found in the string.");
      }
    });
    it("Create CALL Option Contract of 0.00001 CkETH and 0.00002 CKBTC from user1 ", async () => {
      let actor2 = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res2 = await actor2.create_icrc_options(
        {
          CKETH: null,
        },
        {
          asset: { ICRC: { CKETH: null } },
          asset_amount: parseEther("0.00001"),
          strike_price: 5000n * BASE_OF_XRC,
          contract_expiry: BigInt(
            (Date.now() + 1.1 * 60 * 1000) * 1000000 // 1 minute days after
          ),
          use_exchange_account: false,
          offer_duration: BigInt(
            (Date.now() + 0.6 * 60 * 1000) * 1000000 // 1/2 minutes days after
          ),
          contract_state: { OFFER: null },
          options_type: { CALL: null },
        }
      );

      expect(res2).toMatchObject({
        Ok: expect.stringContaining("Option is successfully created with Id"),
      });

      // Use a regular expression to extract the number
      let string2 = res2 as { Ok: string };
      const match2 = string2.Ok.match(/\d+/);

      if (match2) {
        optionId_2 = BigInt(match2[0]); // Convert the matched string to a number (BigInt if it's a large number)
        console.log("Extracted Number:", optionId_2);
      } else {
        console.log("No number found in the string.");
      }
    });

    it("Create 2 PUT Option Contracts 0.000001 CKBTC from user1 ", async () => {
      [1, 2].forEach(async () => {
        let actor1 = createCanisterActor(
          CANISTERS_NAME_NO_PIC.OPTIONS,
          user1
        ) as unknown as ActorSubclass<_OPTIONS>;

        let res1 = await actor1.create_icrc_options(
          {
            CKBTC: null,
          },
          {
            asset: { ICRC: { CKBTC: null } },
            asset_amount: humanToE8s(0.000001),
            strike_price: 160_000n * BASE_OF_XRC,
            contract_expiry: BigInt(
              (Date.now() + 1 * 60 * 1000) * 1000000 // 1 minute days after
            ),
            use_exchange_account: false,
            offer_duration: BigInt(
              (Date.now() + 0.5 * 60 * 1000) * 1000000 // 1/2 minutes days after
            ),
            contract_state: { OFFER: null },
            options_type: { PUT: null },
          }
        );

        expect(res1).toMatchObject({
          Ok: expect.stringContaining("Option is successfully created with Id"),
        });
      });
    });

    it("Create 2 PUT Option Contracts 0.000001 CKBTC from user3 ", async () => {
      [1, 2].forEach(async () => {
        let actor1 = createCanisterActor(
          CANISTERS_NAME_NO_PIC.OPTIONS,
          user3
        ) as unknown as ActorSubclass<_OPTIONS>;

        let res1 = await actor1.create_icrc_options(
          {
            CKBTC: null,
          },
          {
            asset: { ICRC: { CKBTC: null } },
            asset_amount: humanToE8s(0.000001),
            strike_price: 160_000n * BASE_OF_XRC,
            contract_expiry: BigInt(
              (Date.now() + 1 * 60 * 1000) * 1000000 // 1 minute days after
            ),
            use_exchange_account: false,
            offer_duration: BigInt(
              (Date.now() + 0.5 * 60 * 1000) * 1000000 // 1/2 minutes days after
            ),
            contract_state: { OFFER: null },
            options_type: { PUT: null },
          }
        );

        expect(res1).toMatchObject({
          Ok: expect.stringContaining("Option is successfully created with Id"),
        });
      });
    });

    it("Trade CALL Option Contract with optionId_1 with user2 Id should succeed", async () => {
      console.log(": --------");
      console.log(": optionId_1", optionId_1);
      console.log(": --------");
      await wait(0.1);
      // Seller Cannot be a Buyer
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res = await actor.trade_icrc_options({ CKBTC: null }, optionId_1);
      console.log(": ----------");
      console.log(": res", res);
      console.log(": ----------");

      expect(res).toMatchObject({
        Ok: expect.stringContaining("CALL Option Purchased!"),
      });
    });

    it("Trade History of User 1, 2 and 3 should be in correct  contract state", async () => {
      let actor1 = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user1
      ) as unknown as ActorSubclass<_OPTIONS>;
      let actor2 = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;
      let actor3 = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user3
      ) as unknown as ActorSubclass<_OPTIONS>;

      // expect(
      //   (await actor1.get_options_trade_history_by_principal({ OPEN: null }))
      //     .length
      // ).toBe(1);

      // expect(
      //   (await actor2.get_options_trade_history_by_principal({ OPEN: null }))
      //     .length
      // ).toBe(1);

      //     expect(
      //       (await actor3.get_options_trade_history_by_principal({ OFFER: null }))
      //         .length
      //     ).toBe(2);

      //     expect(
      //       (await actor1.get_options_trade_history_by_principal({ OFFER: null }))
      //         .length
      //     ).toBe(2);

      //     await wait(1.3);

      //     expect(
      //       (await actor1.get_options_trade_history_by_principal({ OPEN: null }))
      //         .length
      //     ).toBe(0);
      //     expect(
      //       (await actor2.get_options_trade_history_by_principal({ OPEN: null }))
      //         .length
      //     ).toBe(0);
      //     expect(
      //       (await actor3.get_options_trade_history_by_principal({ OPEN: null }))
      //         .length
      //     ).toBe(0);

      //     expect(
      //       (
      //         await actor1.get_options_trade_history_by_principal({
      //           EXECUTED: null,
      //         })
      //       ).length
      //     ).toBe(1);
      //     expect(
      //       (
      //         await actor2.get_options_trade_history_by_principal({
      //           EXECUTED: null,
      //         })
      //       ).length
      //     ).toBe(1);

      //     expect(
      //       (
      //         await actor1.get_options_trade_history_by_principal({
      //           CLOSED: null,
      //         })
      //       ).length
      //     ).toBe(2);
      //     expect(
      //       (
      //         await actor2.get_options_trade_history_by_principal({
      //           CLOSED: null,
      //         })
      //       ).length
      //     ).toBe(0);
      //     expect(
      //       (
      //         await actor3.get_options_trade_history_by_principal({
      //           CLOSED: null,
      //         })
      //       ).length
      //     ).toBe(2);
    });
  });
});
