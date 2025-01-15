import { execSync } from "child_process";
import { CANISTER_IDS_MAP_NO_PIC, CANISTERS_NAME_NO_PIC } from "./constants";
import {
  _INSURANCE,
  idlFactoryCkethLedger,
  idlFactoryIcpLedger,
  idlFactoryInsurance,
  idlFactoryOptions,
  SERVICES,
} from "../exports";
import { Actor, ActorSubclass, HttpAgent, Identity } from "@dfinity/agent";
export async function install_canister(args: CANISTERS_NAME_NO_PIC) {
  switch (args) {
    case CANISTERS_NAME_NO_PIC.INSURANCE:
      execSync(
        `  dfx deploy insurance --argument='(record {  } )' --specified-id suaf3-hqaaa-aaaaf-bfyoa-cai`,
        {
          stdio: "inherit",
        }
      );
  }
}

/// Install All Canisters and Ledger Canisters
export async function install_all_canisters() {
  execSync(` yarn deploy:all`, {
    stdio: "inherit",
  });
}

export async function delete_canisters(args: CANISTERS_NAME_NO_PIC[]) {
  args.forEach((canisterName) => {
    execSync(
      `dfx canister uninstall-code ${CANISTER_IDS_MAP_NO_PIC.get(
        canisterName
      )} || true`,
      {
        stdio: "inherit",
      }
    );
  });
}

export async function delete_all_canisters() {
  // console.log("Deleting all canisters");
  // execSync(`dfx  stop || true`, {
  //   stdio: "inherit",
  // });

  // execSync(`dfx start --clean --artificial-delay 0 || true`, {
  //   stdio: "inherit",
  // });
  
  execSync(`yarn uninstall:all || true`, {
    stdio: "inherit",
  });
}

/// Create Actor with an option to Actor with Identity
export function createCanisterActor(
  args: CANISTERS_NAME_NO_PIC,
  identity?: Identity
): ActorSubclass<SERVICES> {
  const agent = new HttpAgent({ host: "http://127.0.0.1:8080", identity });

  agent.fetchRootKey().catch((err) => {
    console.warn(
      "Unable to fetch root key. Check to ensure that your local replica is running"
    );
    console.error(err);
  });

  let canisterId = CANISTER_IDS_MAP_NO_PIC.get(args)!;

  switch (args) {
    case CANISTERS_NAME_NO_PIC.INSURANCE:
      return Actor.createActor(idlFactoryInsurance, {
        agent,
        canisterId,
        // ...options.actorOptions,
      }) as ActorSubclass<_INSURANCE>;

    case CANISTERS_NAME_NO_PIC.ICP_LEDGER:
      return Actor.createActor(idlFactoryIcpLedger, {
        agent,
        canisterId,
      });
    case CANISTERS_NAME_NO_PIC.CKBTC_LEDGER:
      return Actor.createActor(idlFactoryCkethLedger, {
        agent,
        canisterId,
      });
    case CANISTERS_NAME_NO_PIC.CKETH_LEDGER:
      return Actor.createActor(idlFactoryCkethLedger, {
        agent,
        canisterId,
      });
    case CANISTERS_NAME_NO_PIC.CKUSDT_LEDGER:
      return Actor.createActor(idlFactoryCkethLedger, {
        agent,
        canisterId,
      });
    case CANISTERS_NAME_NO_PIC.OPTIONS:
      return Actor.createActor(idlFactoryOptions, {
        agent,
        canisterId,
      });
  }
  throw new Error("Provide valid args");
}
