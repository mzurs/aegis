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
import { createCanisterActor } from "../../utils/non-pic/setup-canister";
import { balance } from "../../utils/methods/ledgers/balance";
import { BASE_OF_XRC } from "../../utils/constants";

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
      CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
      user1.getPrincipal().toText(),
      parseEther("0.00002")
    );

    await mintTokens(
      CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
      user2.getPrincipal().toText(),
      parseEther("0.00002")
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
    // await delete_all_canisters();
  });

  describe("Options Main Methods 5", () => {
    let optionId_1: bigint;

    it("Check Balances of user 1 and user 2", async () => {
      let balances = await (async (): Promise<[bigint, bigint][]> => {
        const ledgers: (
          | CANISTERS_NAME_NO_PIC.CKETH_LEDGER
          | CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER
        )[] = [
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

            return balances as [bigint, bigint];
          })
        );

        return results;
      })();

      // For user 1
      expect(balances[0][0]).toBe(parseEther("0.00002"));
      expect(balances[0][1]).toBe(parseEther("2"));

      // For user 2
      expect(balances[0][0]).toBe(parseEther("0.00002"));
      expect(balances[0][1]).toBe(parseEther("2"));
    });

    it("Create PUT Option Contract of 0.00001 CkETH for user1 ", async () => {
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user1
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res = await actor.create_icrc_options(
        {
          CKETH: null,
        },
        {
          asset: { ICRC: { CKETH: null } },
          asset_amount: parseEther("0.00001"),
          strike_price: 2_000n * BASE_OF_XRC,
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

      expect(res).toMatchObject({
        Ok: expect.stringContaining("Option is successfully created with Id"),
      });

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKETH_LEDGER, user1.getPrincipal())
      ).toBe(parseEther((0.00002 - 0.00000001).toString()));

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER, user1.getPrincipal())
      ).toBeLessThan(parseEther("2"));

      // Use a regular expression to extract the number
      let string = res as { Ok: string };
      const match = string.Ok.match(/\d+/);

      if (match) {
        optionId_1 = BigInt(match[0]); // Convert the matched string to a number (BigInt if it's a large number)
        console.log("Extracted Number:", optionId_1);
      } else {
        console.log("No number found in the string.");
      }
    });

    it("Execute Option Manually from user 2 ID Should Fail because contract is in OFFER PHASE", async () => {
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;
      let res = await actor.execute_manual({ CKETH: null }, optionId_1);

      expect(res).toMatchObject({
        Err: expect.stringContaining(
          "Option Cannot Execute from Offer Phase because the caller is not the owner(seller)"
        ),
      });
    });

    it("Trade PUT Option Contract with user2 Id should succeed", async () => {
      let balanceCkUsdtUser1 = await balance(
        CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        user1.getPrincipal()
      );

      console.log(": --------");
      console.log(": optionId_1", optionId_1);
      console.log(": --------");

      // Seller Cannot be a Buyer
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;

      let res = await actor.trade_icrc_options({ CKETH: null }, optionId_1);
      console.log(": ----------");
      console.log(": res", res);
      console.log(": ----------");

      expect(res).toMatchObject({
        Ok: expect.stringContaining("Option Purchased!"),
      });

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER, user2.getPrincipal())
      ).toBeLessThan(parseEther("2"));

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER, user1.getPrincipal())
      ).toBeGreaterThan(balanceCkUsdtUser1);
    });

    it("Execute Option Manually from user 2 ID Should Fail because expiry not reached yet", async () => {
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;
      let res = await actor.execute_manual({ CKETH: null }, optionId_1);

      expect(res).toMatchObject({
        Err: expect.stringContaining("Option Contract Expiry Didn't reached"),
      });
    });

    it("Execute Option Manually from user 2 ID Should Fail because expiry not reached yet", async () => {
      let actor = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user2
      ) as unknown as ActorSubclass<_OPTIONS>;
      let res = await actor.execute_manual({ CKETH: null }, optionId_1);

      expect(res).toMatchObject({
        Err: expect.stringContaining("Option Contract Expiry Didn't reached"),
      });
    });

    it("Execute Option Automatically should succeeded with seller won", async () => {
      let balanceCkethUser1 = await balance(
        CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
        user1.getPrincipal()
      );
      let balanceCkethUser2 = await balance(
        CANISTERS_NAME_NO_PIC.CKETH_LEDGER,
        user2.getPrincipal()
      );
      let balanceCkUsdtUser1 = await balance(
        CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        user1.getPrincipal()
      );
      let balanceCkUsdtUser2 = await balance(
        CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER,
        user2.getPrincipal()
      );

      await wait(1);

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKETH_LEDGER, user1.getPrincipal())
      ).toBe(balanceCkethUser1);

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKETH_LEDGER, user2.getPrincipal())
      ).toBe(balanceCkethUser2);

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER, user1.getPrincipal())
      ).toBeGreaterThan(balanceCkUsdtUser1);

      expect(
        await balance(CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER, user2.getPrincipal())
      ).toBe(balanceCkUsdtUser2);
    }, 66000);
  });
});
