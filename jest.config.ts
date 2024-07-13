import type { Config } from "jest";

const config: Config = {
  watch: false,
  preset: "ts-jest/presets/js-with-ts",
  testEnvironment: "node",
  verbose: true,
  globalSetup: "<rootDir>/tests/global-setup.ts",
  globalTeardown: "<rootDir>/tests/global-teardown.ts",
  testTimeout: 30_000,
};

export default config;
