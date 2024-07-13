import { ActorSubclass, Identity } from "@dfinity/agent";
import { createIdentityFromSeed } from "../../utils/configs";
import {
  _CKBTC_LEDGER,
  _CKBTC_MINTER,
  _CKETH_LEDGER,
  _ICP_LEDGER,
  _INSURANCE,
} from "../../utils/exports";
import {
  CANISTER_IDS_MAP_NO_PIC,
  CANISTERS_NAME_NO_PIC,
} from "../../utils/non-pic/constants";
import {
  createCanisterActor,
  delete_all_canisters,
  install_all_canisters,
} from "../../utils/non-pic/setup-canister";
import { createInsuranceContract } from "../../utils/methods/insurance/create_insurance";
import { set_ledger_ids } from "../../utils/methods/insurance/set_ledger_ids";
import { mintTokens } from "../../utils/non-pic/mint_to_account";
import { getFormattedTime } from "../../utils/helpers";
import { sellInsuranceContract } from "../../utils/methods/insurance/sell_insurance_contract";
import { approveTokens } from "../../utils/methods/ledgers/approve";
import { error } from "console";
import { buyInsuranceContract } from "../../utils/methods/insurance/buy_insurance";
import { currentTimePlusExtraMinutesInNanoseconds } from "../../utils/non-pic/utils";

describe("\n================================= Insurance Canister Integration Testing =================================\n", () => {
  let minter: Identity;
  let user: Identity;
  let buyer: Identity;
  let seller: Identity;
 
  beforeAll(async () => {
    // await delete_all_canisters();
    await install_all_canisters();

    // Generate new Identities
    minter = createIdentityFromSeed("minter");
    user = createIdentityFromSeed("user");
    buyer = createIdentityFromSeed("buyer");
    seller = createIdentityFromSeed("seller");

    //set ledger ids
    await set_ledger_ids();
  });

  describe("Insurance Contract Creation Testing", () => {
    it("Create Insurance Contract with Anonymous Identity should failed", async () => {
      await createInsuranceContract({}).catch((error) => expect(error));
    });

    it("create insurance contract with 1 ICP with user Identity ", async () => {
      await mintTokens(
        CANISTERS_NAME_NO_PIC.ICP_LEDGER,
        user.getPrincipal().toText(),
        1_000_000_000n
      );

      approveTokens(user, CANISTERS_NAME_NO_PIC.ICP_LEDGER);

      let resUser = await createInsuranceContract({ identity: user });

      expect(resUser).toHaveProperty("Success");
    });

    it("Create Insurance Contract with user Identity should be rejected because of less allowance ", async () => {
      let resUser = await createInsuranceContract({
        identity: user,
        amount: 100_000_000_000n,
      });

      expect(resUser).toHaveProperty("TransferError");
    });
  });

  describe("Buy Insurance Contract Testing", () => {
    it("Buy Insurance Contract with Anonymous Identity should failed", async () => {
      await createInsuranceContract({ identity: user }).catch((error) =>
        expect(error)
      );
    });

    it("Buy Insurance Contract with Fake Insurance ID should failed", async () => {
      expect(
        await buyInsuranceContract(buyer, 100001, 10_000_000n)
      ).toMatchObject({
        ErrorMessage: "Insurance with Id 100001 not found",
      });
    });

    it("Buy insurance contract with 1 ICP with buyer Identity ", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
      });

      expect(res).toHaveProperty("Success");

      let id = "Success" in res ? res.Success : 0;

      await mintTokens(
        CANISTERS_NAME_NO_PIC.ICP_LEDGER,
        buyer.getPrincipal().toText(),
        100_000_000n
      );

      approveTokens(buyer, CANISTERS_NAME_NO_PIC.ICP_LEDGER);

      let userBuyer = await buyInsuranceContract(buyer, id, 1_000_000n);
      expect(userBuyer).toHaveProperty("Success");
    });

    it("Buy Insurance with greater than Max Premium Amount should failed ", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
      });

      expect(res).toHaveProperty("Success");

      let id = "Success" in res ? res.Success : 0;

      approveTokens(buyer, CANISTERS_NAME_NO_PIC.ICP_LEDGER);

      let userBuyer = await buyInsuranceContract(buyer, id, 5_000_001n);
      console.log("🚀 ~ it ~ userBuyer:", userBuyer);
      expect(userBuyer).toMatchObject({
        ErrorMessage:
          "Premium amount 5_000_001 exceeds the current premium amount 5_000_000 or premium pool threshold reached",
      });
    });

    it("Buy Insurance with less than Min Premium Amount should failed ", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
      });

      expect(res).toHaveProperty("Success");

      let id = "Success" in res ? res.Success : 0;

      approveTokens(buyer, CANISTERS_NAME_NO_PIC.ICP_LEDGER);

      let userBuyer = await buyInsuranceContract(buyer, id, 900_000n);

      console.log("🚀 ~ it ~ userBuyer:", userBuyer);

      expect(userBuyer).toMatchObject({
        ErrorMessage:
          "Min Premium Amount to Participate in Inusrance is 1_000_000 ",
      });
    });

    it("Buy Insurance with Max Premium Amount should succeed ", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
      });

      expect(res).toHaveProperty("Success");

      let id = "Success" in res ? res.Success : 0;

      approveTokens(buyer, CANISTERS_NAME_NO_PIC.ICP_LEDGER);

      let userBuyer = await buyInsuranceContract(buyer, id, 5_000_000n);

      console.log("🚀 ~ it ~ userBuyer:", userBuyer);

      expect(res).toHaveProperty("Success");
    });

    it("Buy Insurance with less than Max Premium Amount should succeed ", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
      });

      expect(res).toHaveProperty("Success");

      let id = "Success" in res ? res.Success : 0;

      approveTokens(buyer, CANISTERS_NAME_NO_PIC.ICP_LEDGER);

      let userBuyer = await buyInsuranceContract(buyer, id, 4_999_999n);

      console.log("🚀 ~ it ~ userBuyer:", userBuyer);

      expect(res).toHaveProperty("Success");
    });
  });

  describe("Sell Insurance Contract Testing", () => {
    it("Sell Insurance Contract with Anonymous Identity should failed", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
      });

      expect(res).toHaveProperty("Success");

      let insurance_id = "Success" in res ? res.Success : 0;

      await sellInsuranceContract({
        insurance_id,
      }).catch((error) => expect(error));
    });

    it("Sell Insurance Contract with Fake Insurance ID should failed", async () => {
      expect(
        await sellInsuranceContract({
          insurance_id: 100095,
          identity: seller,
          amount: 10_000_000n,
        })
      ).toMatchObject({
        ErrorMessage: "Insurance with Id 100095 not found",
      });
    });

    it("Sell insurance contract with 1 ICP with seller Identity ", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
      });

      expect(res).toHaveProperty("Success");

      let id = "Success" in res ? res.Success : 0;

      await mintTokens(
        CANISTERS_NAME_NO_PIC.ICP_LEDGER,
        seller.getPrincipal().toText(),
        100_000_000n
      );

      approveTokens(seller, CANISTERS_NAME_NO_PIC.ICP_LEDGER);

      let userSeller = await sellInsuranceContract({
        identity: seller,
        insurance_id: id,
        amount: 10_000_000n,
      });

      expect(userSeller).toHaveProperty("Success");
    });

    it("Sell Insurance Contract with less than min share amount should failed", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
        min_share_amount: [5_000_000n],
      });

      expect(res).toHaveProperty("Success");

      let id = "Success" in res ? res.Success : 0;

      let userSeller = await sellInsuranceContract({
        identity: seller,
        insurance_id: id,
        amount: 4_999_999n,
      });

      expect(userSeller).toMatchObject({
        ErrorMessage: "Insurance Minimum Share Amount Some(Nat(5000000))",
      });
    });

    it("Sell Insurance Contract should failed because no participation are allowed for sellers", async () => {
      let res = await createInsuranceContract({
        identity: user,
        amount: 10_000_000n,
        expiry_in: 1,
        min_premium_amount: 1_000_000n,
        min_share_amount: [5_000_000n],
        is_muliple_seller_allowed: false,
      });

      expect(res).toHaveProperty("Success");

      let id = "Success" in res ? res.Success : 0;

      let userSeller = await sellInsuranceContract({
        identity: seller,
        insurance_id: id,
        amount: 9_999_999n,
      });

      expect(userSeller).toMatchObject({
        ErrorMessage: "Participation Not Allowed for Sellers",
      });
    });
  });

  afterAll(async () => {
    console.log("Deleting All Canister...");

    await delete_all_canisters();
  });
});
