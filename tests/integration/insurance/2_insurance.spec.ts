import { Identity } from "@dfinity/agent";
import { createIdentityFromSeed, wait } from "../../utils/configs";
import {
  delete_all_canisters,
  install_all_canisters,
} from "../../utils/non-pic/setup-canister";
import { set_ledger_ids } from "../../utils/methods/insurance/set_ledger_ids";
import { mintTokens } from "../../utils/non-pic/mint_to_account";
import { CANISTERS_NAME_NO_PIC } from "../../utils/non-pic/constants";
import { e8sToHuman, humanToE8s } from "../../utils/helpers";
import { approveTokens } from "../../utils/methods/ledgers/approve";
import { createInsuranceContract } from "../../utils/methods/insurance/create_insurance";
import { buyInsuranceContract } from "../../utils/methods/insurance/buy_insurance";
import { sellInsuranceContract } from "../../utils/methods/insurance/sell_insurance_contract";
import { balance } from "../../utils/methods/ledgers/balance";
import { fee } from "../../utils/methods/ledgers/fee";
import {
  getPoolBalanceByInsuranceId,
  getPremiumBalanceByInsuranceId,
} from "../../utils/methods/insurance/balances";

describe("\n================================= Insurance Canister Integration Testing =================================\n", () => {
  const BUYERS = 10;
  const SELLERS = 10;
  const ISSUERS = 10;

  let issuers: Identity[] = Array(ISSUERS).fill(null);
  let buyers: Identity[] = Array(BUYERS).fill(null);
  let sellers: Identity[] = Array(SELLERS).fill(null);

  beforeAll(async () => {
    await install_all_canisters();

    //set ledger ids
    await set_ledger_ids();

    for (let i = 0; i < ISSUERS; i++) {
      issuers[i] = createIdentityFromSeed(`issuer${i}`);
    }

    for (let i = 0; i < BUYERS; i++) {
      buyers[i] = createIdentityFromSeed(`buyer${i}`);
    }

    for (let i = 0; i < SELLERS; i++) {
      sellers[i] = createIdentityFromSeed(`seller${i}`);
    }

    [issuers, sellers, buyers].forEach((p) => {
      p.forEach(async (identity) => {
        //mint tokens
        await mintTokens(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          identity.getPrincipal().toText(),
          humanToE8s(100)
        );

        //approve tokens
        await approveTokens(
          identity,
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          humanToE8s(100)
        );
      });
    });
  });

  afterAll(async () => {
    console.log("Deleting All Canister...");

    await delete_all_canisters();
  });

  describe("Buy,Sell,Issue Insurance Contract Testing in favor of sellers", () => {
      let insuranceId: number[] = Array(10).fill(null);

      it("Create Insurance Contract of 10 ICP with Issuer [0]", async () => {
          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  issuers[0].getPrincipal()
              )
          ).toBe(humanToE8s(100));

          const response = await fetch(
              "https://api.stlouisfed.org/fred/series/observations?series_id=CPIAUCSL&api_key=15e62224856a1ef86749639d67a04aea&units=pc1&file_type=json&sort_order=desc&limit=1"
          );
          if (!response.ok) {
              throw new Error(`Network response was not ok (${response.status})`);
          }

          let inflation_data: any = await response.json();
          let inflation_point = parseFloat(
              inflation_data?.observations[0].value as unknown as string
          );
          console.log("🚀 ~ it ~ inflation_point:", inflation_point);

          let resIssuer = await createInsuranceContract({
              identity: issuers[0],
              amount: humanToE8s(10),
              inflation_target: inflation_point ?? 100,
              min_premium_amount: humanToE8s(1),
              min_share_amount: [humanToE8s(1)],
              multiplier: { M4X: null },
              is_muliple_seller_allowed: true,
              expiry_in: 1,
          });

          expect(resIssuer).toHaveProperty("Success");

          insuranceId[0] = "Success" in resIssuer ? resIssuer.Success : 0;
          console.log("🚀 ~ it ~ insuranceId:", insuranceId[0])

          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  issuers[0].getPrincipal()
              )
          ).toBe(humanToE8s(90) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n);
      });

      it(`Buy Insurance for insurance Id [0] with buyer[0] should succeed`, async () => {
          console.log("🚀 ~ it ~ insuranceId:", insuranceId[0])

          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  buyers[0].getPrincipal()
              )
          ).toBe(humanToE8s(100) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 1n);

          let buyRes = await buyInsuranceContract(
              buyers[0],
              insuranceId[0],
              humanToE8s(1)
          );
          expect(buyRes).toHaveProperty("Success");

          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  buyers[0].getPrincipal()
              )
          ).toBe(humanToE8s(99) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n);
      });

      it(`Buy Insurance for insurance Id [0] with buyer[1] should succeed`, async () => {
          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  buyers[1].getPrincipal()
              )
          ).toBe(humanToE8s(100) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 1n);

          let buyRes = await buyInsuranceContract(
              buyers[1],
              insuranceId[0],
              humanToE8s(1)
          );
          expect(buyRes).toHaveProperty("Success");

          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  buyers[1].getPrincipal()
              )
          ).toBe(humanToE8s(99) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n);
      });

      it(`Buy Insurance for insurance Id [0] with buyer[2] should failed`, async () => {
          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  buyers[2].getPrincipal()
              )
          ).toBe(humanToE8s(100) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 1n);

          let buyRes = await buyInsuranceContract(
              buyers[2],
              insuranceId[0],
              humanToE8s(1)
          );

          const regex =
              /Premium amount (\d+) exceeds the current premium amount (\d+) or premium pool threshold reached/;

          expect(buyRes).toMatchObject({ ErrorMessage: regex });

          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  buyers[2].getPrincipal()
              )
          ).toBe(humanToE8s(100) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER));
      });

      it("Sell Insurance Contract with Id [0] with Seller[0] should succeed", async () => {
          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  sellers[0].getPrincipal()
              )
          ).toBe(humanToE8s(100) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 1n);

          let sellRes = await sellInsuranceContract({
              identity: sellers[0],
              amount: humanToE8s(2),
              insurance_id: insuranceId[0],
          });

          expect(sellRes).toHaveProperty("Success");

          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  sellers[0].getPrincipal()
              )
          ).toBe(humanToE8s(98) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n);
      });

      it(`Buy Insurance for insurance Id [0] with buyer[2] should succeed`, async () => {
          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  buyers[2].getPrincipal()
              )
          ).toBe(humanToE8s(100) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 1n);

          let buyRes = await buyInsuranceContract(
              buyers[2],
              insuranceId[0],
              humanToE8s(1)
          );

          expect(buyRes).toHaveProperty("Success");

          expect(
              await balance(
                  CANISTERS_NAME_NO_PIC.ICP_LEDGER,
                  buyers[2].getPrincipal()
              )
          ).toBe(humanToE8s(99) - await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n);
      });

      it("Executing Insurance Contract Automatically within 1 minute in favor of Seller And Verify Balance", async () => {
          await wait(1)

          let issuer0Amount = humanToE8s(90) + humanToE8s(10) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n)
          let issuer0ActualAmont = await balance(CANISTERS_NAME_NO_PIC.ICP_LEDGER, issuers[0].getPrincipal());
          console.log("🚀 ~ it ~ issuer0ActualAmont:", e8sToHuman(issuer0ActualAmont))

          expect(
              issuer0ActualAmont
          ).toBeGreaterThan(issuer0Amount);

          let seller0Amount = humanToE8s(98) + humanToE8s(2) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n)
          let seller0ActualAmont = await balance(CANISTERS_NAME_NO_PIC.ICP_LEDGER, sellers[0].getPrincipal());
          console.log("🚀 ~ it ~ seller0ActualAmont:", e8sToHuman(seller0ActualAmont))

          expect(
              seller0ActualAmont
          ).toBeGreaterThan(seller0Amount);

          let buyer0Amount = humanToE8s(99) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n)
          let buyer0ActualAmont = await balance(CANISTERS_NAME_NO_PIC.ICP_LEDGER, buyers[0].getPrincipal());
          console.log("🚀 ~ it ~ buyer0ActualAmont:", e8sToHuman(buyer0ActualAmont))

          expect(
              buyer0ActualAmont
          ).toBe(buyer0Amount);

          let buyer1Amount = humanToE8s(99) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n)
          let buyer1ActualAmont = await balance(CANISTERS_NAME_NO_PIC.ICP_LEDGER, buyers[1].getPrincipal());
          console.log("🚀 ~ it ~ buyer1ActualAmont:", e8sToHuman(buyer1ActualAmont))

          expect(
              buyer1ActualAmont
          ).toBe(buyer1Amount);

          let buyer2Amount = humanToE8s(99) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER) * 2n)
          let buyer2ActualAmont = await balance(CANISTERS_NAME_NO_PIC.ICP_LEDGER, buyers[2].getPrincipal());
          console.log("🚀 ~ it ~ buyer2ActualAmont:", e8sToHuman(buyer2ActualAmont))

          expect(
              buyer2ActualAmont
          ).toBe(buyer2Amount);
      }, 70000)

  });
  ///===============================================================================================================================
  ///===============================================================================================================================

  ///===============================================================================================================================
  ///===============================================================================================================================

  describe("Buy,Sell,Issue Insurance Contract Testing in favor of Buyers", () => {
    let insuranceId: number[] = Array(10).fill(null);

    it("Create Insurance Contract of 15 ICP with Issuer [1]", async () => {
      //   expect(
      //     await balance(
      //       CANISTERS_NAME_NO_PIC.ICP_LEDGER,
      //       issuers[1].getPrincipal()
      //     )
      //   ).toBe(
      //     humanToE8s(100) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 1n
      //   );

      let resIssuer = await createInsuranceContract({
        identity: issuers[1],
        amount: humanToE8s(15),
        inflation_target: 0,
        min_premium_amount: humanToE8s(2),
        min_share_amount: [humanToE8s(5)],
        multiplier: { M4X: null },
        is_muliple_seller_allowed: true,
        expiry_in: 1,
      });
      console.log("🚀 ~ it ~ resIssuer:", resIssuer);

      expect(resIssuer).toHaveProperty("Success");

      insuranceId[0] = "Success" in resIssuer ? resIssuer.Success : 0;
      console.log("🚀 ~ it ~ insuranceId[0]:", insuranceId[0]);

      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          issuers[1].getPrincipal()
        )
      ).toBe(
        humanToE8s(85) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n
      );

      let pool_balance = await getPoolBalanceByInsuranceId(insuranceId[0]);
      console.log("🚀 ~ it ~ pool_balance:", e8sToHuman(pool_balance));
      expect(pool_balance).toBe(humanToE8s(15));
    });

    it(`Buy Insurance for insurance Id [0] with buyer[3] should succeed`, async () => {
      console.log("🚀 ~ it ~ insuranceId:", insuranceId[0]);

      let premium_balance = await getPremiumBalanceByInsuranceId(
        insuranceId[0]
      );
      console.log("🚀 ~ it ~ premium_balance:", e8sToHuman(premium_balance));
      expect(premium_balance).toBe(0n);

      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          buyers[3].getPrincipal()
        )
      ).toBe(
        humanToE8s(100) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 1n
      );

      let buyRes = await buyInsuranceContract(
        buyers[3],
        insuranceId[0],
        humanToE8s(2)
      );
      expect(buyRes).toHaveProperty("Success");

      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          buyers[3].getPrincipal()
        )
      ).toBe(
        humanToE8s(98) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n
      );

      premium_balance = await getPremiumBalanceByInsuranceId(insuranceId[0]);
      console.log("🚀 ~ it ~ premium_balance:", e8sToHuman(premium_balance));
      expect(premium_balance).toBe(humanToE8s(2));
    });

    it(`Buy Insurance for insurance Id [0] with buyer[4] should failed because of max threshold`, async () => {
      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          buyers[4].getPrincipal()
        )
      ).toBe(
        humanToE8s(100) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 1n
      );

      let buyRes = await buyInsuranceContract(
        buyers[4],
        insuranceId[0],
        humanToE8s(2)
      );

      const regex =
        /Premium amount (\d+) exceeds the current premium amount (\d+) or premium pool threshold reached/;

      expect(buyRes).toMatchObject({ ErrorMessage: regex });

      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          buyers[4].getPrincipal()
        )
      ).toBe(
        humanToE8s(100) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 1n
      );

      let premium_balance = await getPremiumBalanceByInsuranceId(
        insuranceId[0]
      );
      console.log("🚀 ~ it ~ premium_balance:", e8sToHuman(premium_balance));
      expect(premium_balance).toBe(humanToE8s(2));
    });

    it("Sell Insurance Contract with Id [0] with Seller[1] should succeed", async () => {
      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          sellers[1].getPrincipal()
        )
      ).toBe(
        humanToE8s(100) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 1n
      );

      let sellRes = await sellInsuranceContract({
        identity: sellers[1],
        amount: humanToE8s(5),
        insurance_id: insuranceId[0],
      });

      expect(sellRes).toHaveProperty("Success");

      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          sellers[1].getPrincipal()
        )
      ).toBe(
        humanToE8s(95) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n
      );

      let pool_balance = await getPoolBalanceByInsuranceId(insuranceId[0]);
      console.log("🚀 ~ it ~ pool_balance:", e8sToHuman(pool_balance));
      expect(pool_balance).toBe(humanToE8s(20));
    });

    it("Sell Insurance Contract with Id [0] with Seller[2] should succeed", async () => {
      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          sellers[2].getPrincipal()
        )
      ).toBe(
        humanToE8s(100) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 1n
      );

      let sellRes = await sellInsuranceContract({
        identity: sellers[2],
        amount: humanToE8s(5),
        insurance_id: insuranceId[0],
      });

      expect(sellRes).toHaveProperty("Success");

      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          sellers[2].getPrincipal()
        )
      ).toBe(
        humanToE8s(95) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n
      );

      let pool_balance = await getPoolBalanceByInsuranceId(insuranceId[0]);
      console.log("🚀 ~ it ~ pool_balance:", e8sToHuman(pool_balance));
      expect(pool_balance).toBe(humanToE8s(25));
    });

    it(`Buy Insurance for insurance Id [0] with buyer[4] should succeed`, async () => {
      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          buyers[4].getPrincipal()
        )
      ).toBe(
        humanToE8s(100) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 1n
      );

      let buyRes = await buyInsuranceContract(
        buyers[4],
        insuranceId[0],
        humanToE8s(2)
      );

      expect(buyRes).toHaveProperty("Success");

      expect(
        await balance(
          CANISTERS_NAME_NO_PIC.ICP_LEDGER,
          buyers[4].getPrincipal()
        )
      ).toBe(
        humanToE8s(98) - (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n
      );

      let premium_balance = await getPremiumBalanceByInsuranceId(
        insuranceId[0]
      );
      console.log("🚀 ~ it ~ premium_balance:", e8sToHuman(premium_balance));
      expect(premium_balance).toBe(humanToE8s(4));
    });

    it("Executing Insurance Contract Automatically within 1 minute in favor of Seller And Verify Balance", async () => {
      await wait(1);

      let premium_balance = await getPremiumBalanceByInsuranceId(
        insuranceId[0]
      );
      console.log("🚀 ~ it ~ premium_balance:", e8sToHuman(premium_balance));
      expect(premium_balance).toBe(humanToE8s(0));

      let pool_balance = await getPoolBalanceByInsuranceId(insuranceId[0]);
      console.log("🚀 ~ it ~ pool_balance:", e8sToHuman(pool_balance));
      expect(pool_balance).toBe(humanToE8s(0));

      let issuer1Amount =
        humanToE8s(100) -
        humanToE8s(15) +
        humanToE8s((15 / 25) * (25 - 2 * (2 * 4))) +
        humanToE8s((15 / 25) * 4) -
        (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n;

      let issuer1ActualAmont = await balance(
        CANISTERS_NAME_NO_PIC.ICP_LEDGER,
        issuers[1].getPrincipal()
      );
      console.log(
        "🚀 ~ it ~ issuer1ActualAmont:",
        e8sToHuman(issuer1ActualAmont)
      );

      expect(issuer1ActualAmont).toBeLessThanOrEqual(issuer1Amount);

      let seller1Amount =
        humanToE8s(100) -
        humanToE8s(5) +
        humanToE8s((5 / 25) * (25 - 2 * (2 * 4))) +
        humanToE8s((5 / 25) * 4) -
        (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n;
      let seller1ActualAmont = await balance(
        CANISTERS_NAME_NO_PIC.ICP_LEDGER,
        sellers[1].getPrincipal()
      );
      console.log(
        "🚀 ~ it ~ seller1ActualAmont:",
        e8sToHuman(seller1ActualAmont)
      );

      expect(seller1ActualAmont).toBeLessThanOrEqual(seller1Amount);

      let seller2Amount =
        humanToE8s(100) -
        humanToE8s(5) +
        humanToE8s((5 / 25) * (25 - 2 * (2 * 4))) +
        humanToE8s((5 / 25) * 4) -
        (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n;
      let seller2ActualAmont = await balance(
        CANISTERS_NAME_NO_PIC.ICP_LEDGER,
        sellers[2].getPrincipal()
      );
      console.log(
        "🚀 ~ it ~ seller2ActualAmont:",
        e8sToHuman(seller2ActualAmont)
      );

      expect(seller2ActualAmont).toBeLessThanOrEqual(seller2Amount);

      let buyer3Amount =
        humanToE8s(98) +
        humanToE8s(2) -
        (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n;
      let buyer3ActualAmont = await balance(
        CANISTERS_NAME_NO_PIC.ICP_LEDGER,
        buyers[3].getPrincipal()
      );
      console.log(
        "🚀 ~ it ~ buyer3ActualAmont:",
        e8sToHuman(buyer3ActualAmont)
      );

      expect(buyer3ActualAmont).toBeGreaterThan(buyer3Amount);

      let buyer4Amount =
        humanToE8s(98) +
        humanToE8s(2) -
        (await fee(CANISTERS_NAME_NO_PIC.ICP_LEDGER)) * 2n;
      let buyer4ActualAmont = await balance(
        CANISTERS_NAME_NO_PIC.ICP_LEDGER,
        buyers[4].getPrincipal()
      );

      console.log(
        "🚀 ~ it ~ buyer4ActualAmont:",
        e8sToHuman(buyer4ActualAmont)
      );

      expect(buyer4ActualAmont).toBeGreaterThan(buyer4Amount);
    }, 70000);
  });
});
