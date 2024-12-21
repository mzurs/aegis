import { ActorSubclass, Identity } from "@dfinity/agent";
import { mintTokens } from "../../utils/non-pic/mint_to_account";
import { createIdentityFromSeed, wait } from "../../utils/configs";
import { humanToE8s } from "../../utils/helpers";
import {
  CANISTER_IDS_MAP_NO_PIC,
  CANISTERS_NAME_NO_PIC,
} from "../../utils/non-pic/constants";
import { parseEther } from "ethers";
import { balance } from "../../utils/methods/ledgers/balance";
import { createCanisterActor } from "../../utils/non-pic/setup-canister";
import { _OPTIONS } from "../../utils/exports";
import { approveTokens } from "../../utils/methods/ledgers/approve";

describe("Options Canister Integration Testing", () => {
  let user1 = createIdentityFromSeed(
    (Math.random() * Math.random() + Math.random()).toString()
  );
  let user2 = createIdentityFromSeed(
    (Math.random() * Math.random() + Math.random()).toString()
  );

  beforeAll(async () => {
    // await install_all_canisters();

    await mintTokens(
      CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
      user1.getPrincipal().toText(),
      humanToE8s(5)
    );

    await mintTokens(
      CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
      user2.getPrincipal().toText(),
      humanToE8s(5)
    );

    await mintTokens(
      CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
      user1.getPrincipal().toText(),
      parseEther("2")
    );

    await mintTokens(
      CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
      user2.getPrincipal().toText(),
      parseEther("2")
    );

    [user1, user2].forEach(async (id) => {
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
    // await delete_all_canisters();
  });

  describe("Options Main Methods 2", () => {
    let id: bigint;

    it("Check Balances of user 1 and user 2", async () => {
      let balances = await (async (): Promise<[bigint, bigint][]> => {
        const ledgers: (
          | CANISTERS_NAME_NO_PIC.CKBTC_LEDGER
          | CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER
        )[] = [
          CANISTERS_NAME_NO_PIC.CKBTC_LEDGER,
          CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        ];

        const results = await Promise.all(
          [user1, user2].map(async (user) => {
            const balances = await Promise.all(
              ledgers.map(
                async (ledger) => await balance(ledger, user.getPrincipal())
              )
            );

            return balances as [bigint, bigint];
          })
        );

        return results;
      })();

      // For user 1
      expect(balances[0][0]).toBeGreaterThanOrEqual(humanToE8s(5));
      expect(balances[0][1]).toBeGreaterThanOrEqual(parseEther("2"));

      // For user 2
      expect(balances[0][0]).toBeGreaterThanOrEqual(humanToE8s(5));
      expect(balances[0][1]).toBeGreaterThanOrEqual(parseEther("2"));
    });

    it("Create CALL Option Contract with 0.000001 BTC from user 1", async () => {
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user1
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res = await actor.create_icrc_options(
        {
          CKBTC: null,
        },
        {
          asset: { ICRC: { CKBTC: null } },
          asset_amount: humanToE8s(0.000001),
          strike_price: 110_000n,
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

      expect(res).toMatchObject({
        Ok: expect.stringContaining("Option is successfully created with Id"),
      });

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKBTC_LEDGER, user1.getPrincipal())
      ).toBeLessThan(humanToE8s(5));

      // Use a regular expression to extract the number
      let string = res as { Ok: string };
      const match = string.Ok.match(/\d+/);

      if (match) {
        id = BigInt(match[0]); // Convert the matched string to a number (BigInt if it's a large number)
        console.log("Extracted Number:", id);
      } else {
        console.log("No number found in the string.");
      }
    });

    it("Trade the same option with same user ID should fail", async () => {
      console.log(": --------");
      console.log(": id", id);
      console.log(": --------");

      // Seller Cannot be a Buyer
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user1
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res = await actor.trade_icrc_options({ CKBTC: null }, id);

      expect(res).toMatchObject({
        Err: expect.stringContaining("Seller Cannot be a Buyer"),
      });
    });

    it("Trade Option should fail because offer is completed", async () => {
      await wait(0.6);

      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res = await actor.trade_icrc_options({ CKBTC: null }, id);

      expect(res).toMatchObject({
        Err: expect.stringContaining("Contract State is not in Offer phase"),
      });
    }, 65000);

    it("Create CALL Option Contract with 0.000002 BTC from user 1", async () => {
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user1
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res = await actor.create_icrc_options(
        {
          CKBTC: null,
        },
        {
          asset: { ICRC: { CKBTC: null } },
          asset_amount: humanToE8s(0.000002),
          strike_price: 110_000n,
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

      expect(res).toMatchObject({
        Ok: expect.stringContaining("Option is successfully created with Id"),
      });

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKBTC_LEDGER, user1.getPrincipal())
      ).toBeLessThan(humanToE8s(5 - 0.000002));

      // Use a regular expression to extract the number
      let string = res as { Ok: string };
      const match = string.Ok.match(/\d+/);

      if (match) {
        id = BigInt(match[0]); // Convert the matched string to a number (BigInt if it's a large number)
        console.log("Extracted Number:", id);
      } else {
        console.log("No number found in the string.");
      }
    });

    it("Trade Option should succeed", async () => {
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res = await actor.trade_icrc_options({ CKBTC: null }, id);
      console.log(": ----------");
      console.log(": res", res);
      console.log(": ----------");

      expect(res).toMatchObject({
        Ok: expect.stringContaining("Option Purchased!"),
      });
    });
  });
});
