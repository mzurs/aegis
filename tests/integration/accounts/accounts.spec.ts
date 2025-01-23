import { Identity } from "@dfinity/agent";
import { Actor, PocketIc } from "@hadronous/pic";
import {
    AegisAccountInfo
} from "../../../declarations/accounts/accounts.did";
import { createIdentityFromSeed, setupCanister } from "../../utils/configs";
import {
    ACCOUNTS_WASM_PATH,
    CANISTER_IDS_MAP,
    CANISTERS_NAME,
    CKBTC_LEDGER_WASM_PATH,
    CKBTC_MINTER_WASM_PATH,
    CKETH_LEDGER_WASM_PATH,
    CKETH_MINTER_WASM_PATH,
    ICP_LEDGER_WASM_PATH,
    KYT_WASM_PATH,
} from "../../utils/constants";
import {
    _ACCOUNTS,
    _CKBTC_LEDGER,
    _CKBTC_MINTER,
    _CKETH_LEDGER,
    _CKETH_MINTER,
    _ICP_LEDGER
} from "../../utils/exports";

describe(" Account Management System", () => {
  let pocketIC: PocketIc;
  let systemAdmin: Identity;
  let customerAccount: Identity;
  let accountManagerCanister: Actor<_ACCOUNTS>;
  let icpLedgerCanister: Actor<_ICP_LEDGER>;
  let ckethLedgerCanister: Actor<_CKETH_LEDGER>;
  let ckbtcLedgerCanister: Actor<_CKBTC_LEDGER>;
  let ckbtcMintingCanister: Actor<_CKBTC_MINTER>;
  let ckethMintingCanister: Actor<_CKETH_MINTER>;

  beforeAll(async () => {
    // Initialize the test environment with required features
    pocketIC = await PocketIc.create(process.env.PIC_URL, {
      nns: true,
      fiduciary: true,
      bitcoin: true,
    });

    await pocketIC.resetTime();

    // Create system identities
    systemAdmin = createIdentityFromSeed("system-admin");
    customerAccount = createIdentityFromSeed("customer-1");

    // Initialize system canisters
    accountManagerCanister = (await setupCanister(
      pocketIC,
      systemAdmin.getPrincipal(),
      ACCOUNTS_WASM_PATH,
      CANISTERS_NAME.ACCOUNTS
    )) as Actor<_ACCOUNTS>;

    icpLedgerCanister = (await setupCanister(
      pocketIC,
      systemAdmin.getPrincipal(),
      ICP_LEDGER_WASM_PATH,
      CANISTERS_NAME.ICP_LEDGER
    )) as Actor<_ICP_LEDGER>;

    ckbtcLedgerCanister = (await setupCanister(
      pocketIC,
      systemAdmin.getPrincipal(),
      CKBTC_LEDGER_WASM_PATH,
      CANISTERS_NAME.CKBTC_LEDGER
    )) as Actor<_CKBTC_LEDGER>;

    ckethLedgerCanister = (await setupCanister(
      pocketIC,
      systemAdmin.getPrincipal(),
      CKETH_LEDGER_WASM_PATH,
      CANISTERS_NAME.CKETH_LEDGER
    )) as Actor<_CKETH_LEDGER>;

    ckbtcMintingCanister = (await setupCanister(
      pocketIC,
      systemAdmin.getPrincipal(),
      CKBTC_MINTER_WASM_PATH,
      CANISTERS_NAME.CKBTC_MINTER
    )) as Actor<_CKBTC_MINTER>;

    // Initialize KYT (Know Your Transaction) service
    await setupCanister(
      pocketIC,
      systemAdmin.getPrincipal(),
      KYT_WASM_PATH,
      CANISTERS_NAME.KYT
    );

    // Configure account manager with token ledger addresses
    accountManagerCanister.setIdentity(systemAdmin);
    await configureTokenLedgers();
  }, 10000);

  const configureTokenLedgers = async () => {
    // Register ledger canister IDs
    await accountManagerCanister.set_canister_id(
      { ICP: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.ICP_LEDGER)!
    );
    await accountManagerCanister.set_canister_id(
      { CKBTC: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_LEDGER)!
    );
    await accountManagerCanister.set_canister_id(
      { CKETH: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_LEDGER)!
    );

    // Register minter canister IDs
    await accountManagerCanister.set_canister_id(
      { CKBTCMINTER: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKBTC_MINTER)!
    );
    await accountManagerCanister.set_canister_id(
      { CKETHMINTER: null },
      CANISTER_IDS_MAP.get(CANISTERS_NAME.CKETH_MINTER)!
    );
  };

  afterAll(async () => {
    await pocketIC.tearDown();
  });

  describe("System Initialization and Setup", () => {
    it("should verify correct deployment of token ledgers", async () => {
      const icpTokenName = await icpLedgerCanister.icrc1_name();
      const ckBTCTokenName = await ckbtcLedgerCanister.icrc1_name();
      const ckETHTokenName = await ckethLedgerCanister.icrc1_name();

      expect(icpTokenName).toBe("ICP");
      expect(ckBTCTokenName).toBe("ckBTC");
      expect(ckETHTokenName).toBe("ckETH");
    });

    it("should confirm minting services are operational", async () => {
      ckethMintingCanister = (await setupCanister(
        pocketIC,
        systemAdmin.getPrincipal(),
        CKETH_MINTER_WASM_PATH,
        CANISTERS_NAME.CKETH_MINTER
      )) as Actor<_CKETH_MINTER>;

      ckbtcMintingCanister.setIdentity(customerAccount);

      const btcMintingAddress = await ckbtcMintingCanister.get_btc_address({
        owner: [],
        subaccount: [],
      });
      const ethMintingAddress = await ckethMintingCanister.minter_address();

      expect(btcMintingAddress).toBeDefined();
      expect(ethMintingAddress).toBeDefined();
      expect(true).toBe(true);
    });
  });

  describe("Customer Account Lifecycle Management", () => {
    it("should successfully create new customer account", async () => {
      accountManagerCanister.setIdentity(customerAccount);
      icpLedgerCanister.setIdentity(customerAccount);
      ckbtcLedgerCanister.setIdentity(customerAccount);
      ckethLedgerCanister.setIdentity(customerAccount);

      const accountCreationResult =
        await accountManagerCanister.create_account();
      expect(accountCreationResult).toMatchObject({ Ok: true });
    });

    it("should update customer profile with username", async () => {
      accountManagerCanister.setIdentity(customerAccount);
      const profileUpdateResult =
        await accountManagerCanister.update_account_user_name("customer_john");
      expect(profileUpdateResult).toMatchObject({ Ok: null });
    });

    it("should retrieve customer account details", async () => {
      accountManagerCanister.setIdentity(customerAccount);
      const accountDetails = await accountManagerCanister.get_account();

      type AccountInfoSubset = Omit<AegisAccountInfo, "user_id">;
      const profileData: AccountInfoSubset = {
        user_name: accountDetails[0]?.user_name!,
      };

      expect(profileData).toMatchObject<AccountInfoSubset>({
        user_name: ["customer_john"],
      });
    });
  });
});
