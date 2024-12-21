import { Actor, ActorSubclass, Identity } from "@dfinity/agent";
import { createIdentityFromSeed } from "../../utils/configs";
import {
  _OPTIONS,
  _CKBTC_LEDGER,
  _CKETH_LEDGER,
  _XRC,
} from "../../utils/exports";
import {
  createCanisterActor,
  delete_all_canisters,
  install_all_canisters,
} from "../../utils/non-pic/setup-canister";
import { CANISTERS_NAME_NO_PIC } from "../../utils/non-pic/constants";

describe("Options Canister Integration Testing", () => {
  let minter: Identity;
  let user: Identity;

  minter = createIdentityFromSeed("minter");
  user = createIdentityFromSeed(
    (Math.random() * Math.random() + Math.random()).toString()
  );

  beforeAll(async () => {
    await install_all_canisters();
  });

  afterAll(async () => {
    delete_all_canisters();
  });

  describe("Exchange Rate Canister", () => {
    it("Get the current price of BTC/USDT using XRC", async () => {
      let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user
      ) as unknown as ActorSubclass<_OPTIONS>;

      let rate = await actor.get_exchange_rate({ BTC: null });
      console.log(": ------------");
      console.log(": rate", rate);
      console.log(": ------------");

      expect(rate).toBeDefined();
    });

    it("Get the current price of ETH/USDT using XRC", async () => {
      let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user
      ) as unknown as ActorSubclass<_OPTIONS>;

      let rate = await actor.get_exchange_rate({ ETH: null });
      console.log(": ------------");
      console.log(": rate", rate);
      console.log(": ------------");

      expect(rate).toBeDefined();
    });
  });

  describe("Premium Calculation", () => {
    it("Calculating PUT Premium for BTC", async () => {
      let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user
      ) as unknown as ActorSubclass<_OPTIONS>;

      let premium = await actor.calculate_premium(
        99000n,
        { PUT: null },
        BigInt((Date.now() + 10 * 24 * 60 * 60 * 1_000) * 1_000_000),
        { BTC: null }
      );
      console.log(": ------------------");
      console.log(": premium", premium);
      console.log(": ------------------");
      expect(premium).toBeDefined();
    });

    it("Calculating CALL Premium for ETH", async () => {
      let actor: ActorSubclass<_OPTIONS> = createCanisterActor(
        CANISTERS_NAME_NO_PIC.OPTIONS,
        user
      ) as unknown as ActorSubclass<_OPTIONS>;

      let premium = await actor.calculate_premium(
        4000n,
        { CALL: null },
        BigInt((Date.now() + 10 * 24 * 60 * 60 * 1_000) * 1_000_000),
        { ETH: null }
      );
      console.log(": ------------------");
      console.log(": premium", premium);
      console.log(": ------------------");
      expect(premium).toBeDefined();
    });
  });
});
