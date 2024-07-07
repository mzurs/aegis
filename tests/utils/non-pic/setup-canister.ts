import { execSync } from "child_process";
import { CANISTER_IDS_MAP_NO_PIC, CANISTERS_NAME_NO_PIC } from "./constants";
import { _INSURANCE, SERVICES } from "../exports";
import { createActor, idlFactory } from "../../../declarations/insurance/index";
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
export async function getCanisterId(
  args: CANISTERS_NAME_NO_PIC
): Promise<string> {
  let id = execSync(
    `dfx canister id ${CANISTER_IDS_MAP_NO_PIC.get(args)} || true`,
    {
      stdio: "inherit",
    }
  );
  return id as unknown as string;
}
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
  //   switch (args) {
  // case CANISTERS_NAME_NO_PIC.INSURANCE:
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
    // ...options.actorOptions,
  }) as ActorSubclass<_INSURANCE>;
  //   }
}
