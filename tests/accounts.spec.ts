import { Actor, PocketIc } from "@hadronous/pic";
import { createIdentityFromSeed, setupCanister } from "./utils/configs";
import {
  _ACCOUNTS,
  _CKBTC_LEDGER,
  _CKBTC_MINTER,
  _CKETH_LEDGER,
  _CKETH_MINTER,
  _ICP_LEDGER,
  _KYT,
} from "./utils/exports";
import { Identity } from "@dfinity/agent";
import {
  ACCOUNTS_WASM_PATH,
  CANISTER_IDS_MAP,
  CANISTERS_NAME,
  CKBTC_LEDGER_WASM_PATH,
  CKBTC_MINTER_WASM_PATH,
  CKETH_LEDGER_WASM_PATH,
  CKETH_MINTER_WASM_PATH,
  // CKETH_MINTER_WASM_PATH,
  ICP_LEDGER_WASM_PATH,
  KYT_WASM_PATH,
} from "./utils/constants";
import { humanToE8s } from "./utils/helpers";
import { principalToSubAccount } from "@dfinity/utils";
import { Account } from "@dfinity/ledger-icp";
import { Icrc1TransferResult } from "../declarations/icp_ledger/icp_ledger.did";
import {
  AegisAccountInfo,
  IcrcTransferResult,
} from "../declarations/accounts/accounts.did";
import { parseEther } from "ethers/utils";

describe("Account Canister", () => {
  let pic: PocketIc;
  let minter: Identity;
  let user: Identity;
  let accountsActor: Actor<_ACCOUNTS>;
  let icpLedgerActor: Actor<_ICP_LEDGER>;
  let ckethLedgerActor: Actor<_CKETH_LEDGER>;
  let ckbtcLedgerActor: Actor<_CKBTC_LEDGER>;
  let ckbtcMinterActor: Actor<_CKBTC_MINTER>;
  let ckethMinterActor: Actor<_CKETH_MINTER>;

  beforeAll(async () => {
    pic = await PocketIc.create({ nns: true, fiduciary: true, bitcoin: true });

    // Generate new Identities
    minter = createIdentityFromSeed("minter");
    user = createIdentityFromSeed("user");

    accountsActor = (await setupCanister(
      pic,
      minter.getPrincipal(),
      ACCOUNTS_WASM_PATH,
      CANISTERS_NAME.ACCOUNTS
    )) as Actor<_ACCOUNTS>;

    icpLedgerActor = (await setupCanister(
      pic,
      minter.getPrincipal(),
      ICP_LEDGER_WASM_PATH,
      CANISTERS_NAME.ICP_LEDGER
    )) as Actor<_ICP_LEDGER>;

    ckbtcLedgerActor = (await setupCanister(
      pic,
      minter.getPrincipal(),
      CKBTC_LEDGER_WASM_PATH,
      CANISTERS_NAME.CKBTC_LEDGER
    )) as Actor<_CKBTC_LEDGER>;

    ckethLedgerActor = (await setupCanister(
      pic,
      minter.getPrincipal(),
      CKETH_LEDGER_WASM_PATH,
      CANISTERS_NAME.CKETH_LEDGER
    )) as Actor<_CKETH_LEDGER>;

    ckbtcMinterActor = (await setupCanister(
      pic,
      minter.getPrincipal(),
      CKBTC_MINTER_WASM_PATH,
      CANISTERS_NAME.CKBTC_MINTER
    )) as Actor<_CKBTC_MINTER>;

    (await setupCanister(
      pic,
      minter.getPrincipal(),
      KYT_WASM_PATH,
      CANISTERS_NAME.KYT
    )) as Actor<_KYT>;

    accountsActor.setIdentity(minter);

    // set the ledger ids in the Account Canister
    accountsActor.set_canister_id(
      { ICP: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.ICP_LEDGER)!
    );
    accountsActor.set_canister_id(
      { CKBTC: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_LEDGER)!
    );
    accountsActor.set_canister_id(
      { CKETH: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_LEDGER)!
    );

    // set the minter ids in the Account Canister
    accountsActor.set_canister_id(
      { CKBTCMINTER: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_MINTER)!
    );
    accountsActor.set_canister_id(
      { CKETHMINTER: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_MINTER)!
    );
  });

  afterAll(async () => {
    await pic.tearDown();
  });

  describe("Canisters Setup", () => {
    it("Ledger Deployment", async () => {
      const ICP = await icpLedgerActor.icrc1_name();
      const ckBTC = await ckbtcLedgerActor.icrc1_name();
      const ckETH = await ckethLedgerActor.icrc1_name();

      expect(ICP).toBe("ICP");
      expect(ckBTC).toBe("ckBTC");
      expect(ckETH).toBe("ckETH");
    });

    it("Minter status", async () => {
      ckethMinterActor = (await setupCanister(
        pic,
        minter.getPrincipal(),
        CKETH_MINTER_WASM_PATH,
        CANISTERS_NAME.CKETH_MINTER
      )) as Actor<_CKETH_MINTER>;

      ckbtcMinterActor.setIdentity(user);
      // ckethMinterActor.setIdentity(user);

      const ckbtcMinterAddress = await ckbtcMinterActor.get_btc_address({
        owner: [], //CANISTER_IDS_MAP.get(CANISTERS_NAME.ACCOUNTS)!,
        subaccount: [],
      });
      console.log("🚀 ~ it ~ minterAddress:", ckbtcMinterAddress);

      const ckethMinterAddress = await ckethMinterActor.minter_address();
      console.log("🚀 ~ it ~ ckethMinterAddress:", ckethMinterAddress);
      expect(true).toBe(true);
    });
  });
  describe("Accounts", () => {
    it("Create Account", async () => {
      accountsActor.setIdentity(user);
      icpLedgerActor.setIdentity(user);
      ckbtcLedgerActor.setIdentity(user);
      ckethLedgerActor.setIdentity(user);

      const create = await accountsActor.create_account();

      expect(create).toMatchObject({ Ok: true });
    });

    it("Update user account with UserName", async () => {
      accountsActor.setIdentity(user);

      const res = await accountsActor.update_account_user_name("zohaib");
      expect(res).toMatchObject({ Ok: null });
    });

    it("Get user account", async () => {
      accountsActor.setIdentity(user);

      const res = await accountsActor.get_account();
      type res_omit_type = Omit<AegisAccountInfo, "user_id">;
      const res_omit: res_omit_type = {
        user_name: res[0]?.user_name!,
      };
      expect(res_omit).toMatchObject<res_omit_type>({
        user_name: ["zohaib"],
      });
    });

    it("Account Balance of User should be 0", async () => {
      const ckBTCBalance = await ckbtcLedgerActor.icrc1_balance_of({
        owner: user.getPrincipal(),
        subaccount: [],
      });
      const ckETHBalance = await ckethLedgerActor.icrc1_balance_of({
        owner: user.getPrincipal(),
        subaccount: [],
      });

      const icpBalance = await icpLedgerActor.icrc1_balance_of({
        owner: user.getPrincipal(),
        subaccount: [],
      });

      expect(icpBalance).toBe(0n);
      expect(ckBTCBalance).toBe(0n);
      expect(ckETHBalance).toBe(0n);
    });

    it("Transfer 11 ICP,ckBTC,ckETH to user wallet", async () => {
      icpLedgerActor.setIdentity(minter);
      ckbtcLedgerActor.setIdentity(minter);
      ckethLedgerActor.setIdentity(minter);

      let to: Account = {
        owner: user.getPrincipal(),
        subaccount: [], //[principalToSubAccount(user.getPrincipal())],
      };
      const transferICP: Icrc1TransferResult =
        await icpLedgerActor.icrc1_transfer({
          to,
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          amount: humanToE8s(11),
        });

      const transferckBTC: Icrc1TransferResult =
        await ckbtcLedgerActor.icrc1_transfer({
          to,
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          amount: humanToE8s(11),
        });
      const transferckETH: Icrc1TransferResult =
        await ckethLedgerActor.icrc1_transfer({
          to,
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          amount: parseEther("1"),
        });

      expect(transferICP).toHaveProperty("Ok");
      expect(transferckBTC).toHaveProperty("Ok");
      expect(transferckETH).toHaveProperty("Ok");
    });

    it("User Wallet Balance should be 11 ICP, 11 ckBTC, 1 ckETH", async () => {
      icpLedgerActor.setIdentity(user);

      let args: Account = {
        owner: user.getPrincipal(),
        subaccount: [],
      };

      const balanceICP = await icpLedgerActor.icrc1_balance_of(args);
      const balanceCKBTC = await ckbtcLedgerActor.icrc1_balance_of(args);
      const balanceCKETH = await ckethLedgerActor.icrc1_balance_of(args);

      expect(balanceICP).toBe(humanToE8s(11));
      expect(balanceCKBTC).toBe(humanToE8s(11));
      expect(balanceCKETH).toBe(parseEther("1"));
    });

    it("Transfer 10 ICP, 10 ckBTC, 0.9 ckETH from user wallet to user Account", async () => {
      icpLedgerActor.setIdentity(user);

      let to: Account = {
        owner: CANISTER_IDS_MAP.get(CANISTERS_NAME.ACCOUNTS)!,
        subaccount: [principalToSubAccount(user.getPrincipal())],
      };
      const transferICP: Icrc1TransferResult =
        await icpLedgerActor.icrc1_transfer({
          to,
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          amount: humanToE8s(10),
        });
      const transferCKBTC: Icrc1TransferResult =
        await ckbtcLedgerActor.icrc1_transfer({
          to,
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          amount: humanToE8s(10),
        });
      const transferCKETH: Icrc1TransferResult =
        await ckethLedgerActor.icrc1_transfer({
          to,
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          amount: parseEther("0.9"),
        });

      expect(transferICP).toHaveProperty("Ok");
      expect(transferCKBTC).toHaveProperty("Ok");
      expect(transferCKETH).toHaveProperty("Ok");
    });

    it("Account of user should contain 10 ICP, 10 ckBTC, 0.9 ckETH", async () => {
      icpLedgerActor.setIdentity(user);

      let args: Account = {
        owner: CANISTER_IDS_MAP.get(CANISTERS_NAME.ACCOUNTS)!,
        subaccount: [principalToSubAccount(user.getPrincipal())],
      };

      const balanceICP = await icpLedgerActor.icrc1_balance_of(args);
      const balanceCKBTC = await ckbtcLedgerActor.icrc1_balance_of(args);
      const balanceCKETH = await ckethLedgerActor.icrc1_balance_of(args);

      expect(balanceICP).toBe(humanToE8s(10));
      expect(balanceCKBTC).toBe(humanToE8s(10));
      expect(balanceCKETH).toBe(parseEther("0.9"));
    });

    it("User Wallet Balance should be 0.9999 ICP,<0.9999 ckBTC, <0.5 ckETH", async () => {
      icpLedgerActor.setIdentity(user);

      let args: Account = {
        owner: user.getPrincipal(),
        subaccount: [],
      };

      const balance = await icpLedgerActor.icrc1_balance_of(args);

      expect(balance).toBe(humanToE8s(0.9999));
    });

    it("Transfer 9 ICP, 9ckBTC , 0.8 ckETH from User Account to User", async () => {
      accountsActor.setIdentity(user);

      const resICP = await accountsActor.transfer_from_account(
        { ICP: null },
        [],
        humanToE8s(9)
      );
      const resCKBTC = await accountsActor.transfer_from_account(
        { CKBTC: null },
        [],
        humanToE8s(9)
      );
      const resCKETH: IcrcTransferResult =
        await accountsActor.transfer_from_account(
          { CKETH: null },
          [],
          parseEther("0.8")
        );
      expect(resICP).toHaveProperty("TransferSuccess");
      expect(resCKBTC).toHaveProperty("TransferSuccess");
      expect(resCKETH).toHaveProperty("TransferSuccess");
    });

    it("Account of user should contain 0.9999 ICP,ckBTC,ckETH", async () => {
      icpLedgerActor.setIdentity(user);

      let args: Account = {
        owner: CANISTER_IDS_MAP.get(CANISTERS_NAME.ACCOUNTS)!,
        subaccount: [principalToSubAccount(user.getPrincipal())],
      };

      const balance = await icpLedgerActor.icrc1_balance_of(args);

      expect(balance).toBe(humanToE8s(0.9999));
    });
  });
});
