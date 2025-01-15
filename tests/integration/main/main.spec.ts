import { Actor, PocketIc } from "@hadronous/pic";
import {
  _CKBTC_LEDGER,
  _CKBTC_MINTER,
  _CKETH_LEDGER,
  _CKETH_MINTER,
  _ICP_LEDGER,
  _AEGIS_LEDGER,
  _MAIN,
} from "../../utils/exports";
import { Identity } from "@dfinity/agent";
import {
  createIdentityFromSeed,
  setupCanister,
  wait,
} from "../../utils/configs";
import {
  AEGIS_LEDGER_WASM_PATH,
  CANISTER_IDS_MAP,
  CANISTERS_NAME,
  MAIN_WASM_PATH,
} from "../../utils/constants";
import {
  ExecuteUnstakeAmountRes,
  StakeIcrcRes,
  TotalValueLockedRes,
} from "../../../declarations/main/main.did";
import { e8sToHuman, humanToE8s } from "../../utils/helpers";
import {
  approveTokens,
  balance,
  transferTokens,
} from "../../utils/methods/ledgers/pic_ledger";
import { principalToSubAccount } from "@dfinity/utils";

describe("======================================Main Canister Integration Testing======================================", () => {
  let pic: PocketIc;
  let minter: Identity;
  let user1: Identity;
  let user2: Identity;
  let user3: Identity;
  let user4: Identity;
  let user5: Identity;
  let user6: Identity;

  let aegisLedgerActor: Actor<_AEGIS_LEDGER>;
  let mainCanisterActor: Actor<_MAIN>;

  beforeAll(async () => {
    pic = await PocketIc.create(process.env.PIC_URL);
    await pic.resetTime();
    minter = createIdentityFromSeed("minter");
    user1 = createIdentityFromSeed("user1");
    user2 = createIdentityFromSeed("user2");
    user3 = createIdentityFromSeed("user3");
    user4 = createIdentityFromSeed("user4");
    user5 = createIdentityFromSeed("user5");
    user6 = createIdentityFromSeed("user6");

    aegisLedgerActor = (await setupCanister(
      pic,
      minter.getPrincipal(),
      AEGIS_LEDGER_WASM_PATH,
      CANISTERS_NAME.AEGIS_LEDGER
    )) as Actor<_AEGIS_LEDGER>;
    mainCanisterActor = (await setupCanister(
      pic,
      minter.getPrincipal(),
      MAIN_WASM_PATH,
      CANISTERS_NAME.MAIN
    )) as Actor<_MAIN>;
    mainCanisterActor.setIdentity(minter);
    mainCanisterActor.set_canister_id(
      { AEGIS: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.AEGIS_LEDGER)!
    );
    mainCanisterActor.set_min_staking_delay([60n]);
  });

  afterAll(async () => {
    await pic.tearDown();
  });

  describe("Aegis Transfer Test", () => {
    it("Balance should be zero user1,2,3", async () => {
      let res1 = await balance(aegisLedgerActor, user1.getPrincipal());
      let res2 = await balance(aegisLedgerActor, user2.getPrincipal());
      let res3 = await balance(aegisLedgerActor, user3.getPrincipal());
      let res4 = await balance(aegisLedgerActor, user4.getPrincipal());
      let res5 = await balance(aegisLedgerActor, user5.getPrincipal());
      let res6 = await balance(aegisLedgerActor, user6.getPrincipal());
      expect(e8sToHuman(res1)).toBe(0);
      expect(e8sToHuman(res2)).toBe(0);
      expect(e8sToHuman(res3)).toBe(0);
      expect(e8sToHuman(res4)).toBe(0);
      expect(e8sToHuman(res5)).toBe(0);
      expect(e8sToHuman(res6)).toBe(0);
    });

    it("Transfer 100 AEGIS tokens to user 1,2,3 wallet", async () => {
      aegisLedgerActor.setIdentity(minter);
      let res1 = await transferTokens(
        aegisLedgerActor,
        humanToE8s(100),
        user1.getPrincipal()
      );
      let res2 = await transferTokens(
        aegisLedgerActor,
        humanToE8s(100),
        user2.getPrincipal()
      );
      let res3 = await transferTokens(
        aegisLedgerActor,
        humanToE8s(100),
        user3.getPrincipal()
      );
      expect(res1).toHaveProperty("Ok");
      expect(res2).toHaveProperty("Ok");
      expect(res3).toHaveProperty("Ok");
    });

    it("Transfer 1000 AEGIS tokens to Main canister wallet", async () => {
      aegisLedgerActor.setIdentity(minter);
      let res = await transferTokens(
        aegisLedgerActor,
        humanToE8s(1000),
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        await mainCanisterActor.convert_u32_to_subaccount(1)
      );
      expect(res).toHaveProperty("Ok");
    });

    it("Balance of Main Wallet should be 1000 AEGIS", async () => {
      let res = await balance(
        aegisLedgerActor,
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        await mainCanisterActor.convert_u32_to_subaccount(1)
      );
      expect(res).toBe(humanToE8s(1000));
    });

    it("Balance should be 100 user1,2,3", async () => {
      let res1 = await balance(aegisLedgerActor, user1.getPrincipal());
      let res2 = await balance(aegisLedgerActor, user2.getPrincipal());
      let res3 = await balance(aegisLedgerActor, user3.getPrincipal());
      expect(e8sToHuman(res1)).toBe(100);
      expect(e8sToHuman(res2)).toBe(100);
      expect(e8sToHuman(res3)).toBe(100);
    });

    it("Approve Tokens", async () => {
      aegisLedgerActor.setIdentity(user1);
      await approveTokens(aegisLedgerActor, humanToE8s(100));
      aegisLedgerActor.setIdentity(user2);
      await approveTokens(aegisLedgerActor, humanToE8s(100));
      aegisLedgerActor.setIdentity(user3);
      await approveTokens(aegisLedgerActor, humanToE8s(100));
    });
  });

  describe("Staking Test", () => {
    it("Staking delay should be 60 sec", async () => {
      let delay = await mainCanisterActor.get_min_staking_delay();
      expect(delay).toBe(60n * 1000000000n);
    });

    it("TVL should be 0", async () => {
      let tvl: TotalValueLockedRes =
        await mainCanisterActor.get_total_value_locked({
          ICRC: {
            AEGIS: null,
          },
        });
      expect("ICRC" in tvl ? tvl.ICRC.valueOf() : 1n).toBe(0n);
    });

    it("Stake 10 tokens with user1 Identity", async () => {
      mainCanisterActor.setIdentity(user1);
      let res: StakeIcrcRes = await mainCanisterActor.icrc_stake_tokens(
        { AEGIS: null },
        { use_account: false, amount: humanToE8s(10) }
      );
      expect(res).toHaveProperty("Success");
    });

    it("Staked Amount of User1 should be 10 AEGIS", async () => {
      let res = await balance(
        aegisLedgerActor,
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        principalToSubAccount(user1.getPrincipal())
      );
      expect(res).toBe(humanToE8s(10));
    });

    it("TVL should be 10", async () => {
      let tvl: TotalValueLockedRes =
        await mainCanisterActor.get_total_value_locked({
          ICRC: {
            AEGIS: null,
          },
        });
      expect("ICRC" in tvl ? tvl.ICRC.valueOf() : 1n).toBe(humanToE8s(10));
    });

    it("Initiate Process to UnStake tokens with user1 Identity", async () => {
      mainCanisterActor.setIdentity(user1);
      let res: StakeIcrcRes = await mainCanisterActor.icrc_unstake_tokens(
        { AEGIS: null },
        { to_account: false, amount: humanToE8s(10) }
      );
      expect(res).toHaveProperty("Success");
      await wait(1);
      await pic.advanceTime(60 * 1_000_000);
      await pic.tick(100);
    }, 70000);

    it("Balance of Staked Amount of User1 should be 10 AEGIS", async () => {
      let res = await balance(
        aegisLedgerActor,
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        principalToSubAccount(user1.getPrincipal())
      );
      expect(res).toBe(humanToE8s(10));
    });

    it("Manual UnStake tokens with User1 Identity  ", async () => {
      mainCanisterActor.setIdentity(user1);
      let res: ExecuteUnstakeAmountRes =
        await mainCanisterActor.icrc_unstake_tokens_manual({
          AEGIS: null,
        });
      expect(res).toHaveProperty("Success");
    });

    it("Balance of Staked Amount of User1 should be 0 AEGIS", async () => {
      let res = await balance(
        aegisLedgerActor,
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        principalToSubAccount(user1.getPrincipal())
      );
      expect(res).toBe(humanToE8s(0));
    });

    it("TVL should be 0", async () => {
      let tvl: TotalValueLockedRes =
        await mainCanisterActor.get_total_value_locked({
          ICRC: {
            AEGIS: null,
          },
        });
      expect("ICRC" in tvl ? tvl.ICRC.valueOf() : 1n).toBe(humanToE8s(0));
    });

    it("Balance of user 1 should be >99 ", async () => {
      let res1 = await balance(aegisLedgerActor, user1.getPrincipal());
      expect(res1).toBe(humanToE8s(100) - 100_000n * 3n);
    });

    it("List Execution Logs ", async () => {
      let res1 = await mainCanisterActor.get_stake_execution_logs();
      expect(res1);
    });

    it("Stake 5 tokens with user1 Identity", async () => {
      mainCanisterActor.setIdentity(user1);
      let res: StakeIcrcRes = await mainCanisterActor.icrc_stake_tokens(
        { AEGIS: null },
        { use_account: false, amount: humanToE8s(5) }
      );
      expect(res).toHaveProperty("Success");
    });

    it("Staked Amount of User1 should be 5 AEGIS", async () => {
      let res = await balance(
        aegisLedgerActor,
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        principalToSubAccount(user1.getPrincipal())
      );
      expect(res).toBe(humanToE8s(5));
    });

    it("TVL should be 5", async () => {
      let tvl: TotalValueLockedRes =
        await mainCanisterActor.get_total_value_locked({
          ICRC: {
            AEGIS: null,
          },
        });
      expect("ICRC" in tvl ? tvl.ICRC.valueOf() : 1n).toBe(humanToE8s(5));
    });

    it("Set Rewards Duration", async () => {
      mainCanisterActor.set_rewards_duration({ ICRC: { AEGIS: null } }, 30n);
      await wait(0.5);
      await pic.advanceTime(30 * 1_000_000);
      await pic.tick(100);
    }, 40000);

    it("Balance of user 1 should be >1099 ", async () => {
      let res1 = await balance(aegisLedgerActor, user1.getPrincipal());
      expect(res1).toBe(
        humanToE8s(1000) + humanToE8s(100) - humanToE8s(5) - 100_000n * 5n
      );
    });

    it("Balance of Main Wallet should be 0 AEGIS", async () => {
      let res = await balance(
        aegisLedgerActor,
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        await mainCanisterActor.convert_u32_to_subaccount(1)
      );
      expect(res).toBe(humanToE8s(0));
    });

    it("Stake 5 AEGIS tokens with user2 Identity", async () => {
      mainCanisterActor.setIdentity(user2);
      let res: StakeIcrcRes = await mainCanisterActor.icrc_stake_tokens(
        { AEGIS: null },
        { use_account: false, amount: humanToE8s(5) }
      );
      expect(res).toHaveProperty("Success");
    });

    it("Stake 5 AEGIS tokens with user3 Identity", async () => {
      mainCanisterActor.setIdentity(user3);
      let res: StakeIcrcRes = await mainCanisterActor.icrc_stake_tokens(
        { AEGIS: null },
        { use_account: false, amount: humanToE8s(5) }
      );
      expect(res).toHaveProperty("Success");
    });

    it("TVL should be 15", async () => {
      let tvl: TotalValueLockedRes =
        await mainCanisterActor.get_total_value_locked({
          ICRC: {
            AEGIS: null,
          },
        });
      expect("ICRC" in tvl ? tvl.ICRC.valueOf() : 1n).toBe(humanToE8s(15));
    });

    it("Transfer 9000 AEGIS tokens to Main canister wallet", async () => {
      aegisLedgerActor.setIdentity(minter);
      let res = await transferTokens(
        aegisLedgerActor,
        humanToE8s(9000),
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        await mainCanisterActor.convert_u32_to_subaccount(1)
      );
      expect(res).toHaveProperty("Ok");
      await wait(0.5);
      await pic.advanceTime(30 * 1_000_000);
      await pic.tick(100);
    }, 32000);

    it("Balance of user1 should be >1399 ", async () => {
      let res1 = await balance(aegisLedgerActor, user1.getPrincipal());
      expect(res1).toBe(
        humanToE8s(3000) +
          humanToE8s(1000) +
          humanToE8s(100) -
          humanToE8s(5) -
          100_000n * 6n
      );
    });

    it("Balance of user2 should be >399 ", async () => {
      let res1 = await balance(aegisLedgerActor, user2.getPrincipal());
      expect(res1).toBe(
        humanToE8s(3000) + humanToE8s(100) - humanToE8s(5) - 100_000n * 3n
      );
    });

    it("Balance of user3 should be >399 ", async () => {
      let res1 = await balance(aegisLedgerActor, user3.getPrincipal());
      expect(res1).toBe(
        humanToE8s(3000) + humanToE8s(100) - humanToE8s(5) - 100_000n * 3n
      );
    });

    it("Balance of Main Wallet should be 0 AEGIS", async () => {
      let res = await balance(
        aegisLedgerActor,
        CANISTER_IDS_MAP.get(CANISTERS_NAME.MAIN)!,
        await mainCanisterActor.convert_u32_to_subaccount(1)
      );
      expect(res).toBe(humanToE8s(0));
    });

    it("List Execution Logs ", async () => {
      let res1 = await mainCanisterActor.get_stake_execution_logs();
      expect(res1);
    });
  });
});
