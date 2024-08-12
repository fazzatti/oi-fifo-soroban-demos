import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { Core } from "stellar-plus/lib/stellar-plus/core";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";

export const wasmPath =
  "../../target/wasm32-unknown-unknown/release/wrapper_interface.wasm";

export enum Methods {
  Initialize = "initialize",
  ActivateWrapper = "activate_wrapper",
  DeactivateWrapper = "deactivate_wrapper",
  IsWrapperActive = "is_wrapper_active",
  SetAdmin = "set_admin",
  GetAdmin = "get_admin",
  Balance = "balance",
  Authorized = "authorized",
  Transfer = "transfer",
  Mint = "mint",
  Burn = "burn",
  SetAuthorized = "set_authorized",
}

export const spec = new Core.Spec([
  "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAAAwAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAAVhc3NldAAAAAAAABMAAAAAAAAAEGFzc2V0X2NvbnRyb2xsZXIAAAATAAAAAA==",
  "AAAAAAAAAAAAAAAQYWN0aXZhdGVfd3JhcHBlcgAAAAAAAAAA",
  "AAAAAAAAAAAAAAASZGVhY3RpdmF0ZV93cmFwcGVyAAAAAAAAAAAAAA==",
  "AAAAAAAAAAAAAAAJc2V0X2FkbWluAAAAAAAAAQAAAAAAAAAJbmV3X2FkbWluAAAAAAAAEwAAAAA=",
  "AAAAAAAAAAAAAAAOc2V0X2F1dGhvcml6ZWQAAAAAAAIAAAAAAAAAAmlkAAAAAAATAAAAAAAAAAlhdXRob3JpemUAAAAAAAABAAAAAA==",
  "AAAAAAAAAAAAAAAEbWludAAAAAIAAAAAAAAAAnRvAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
  "AAAAAAAAAAAAAAAIdHJhbnNmZXIAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAACdG8AAAAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAA=",
  "AAAAAAAAAAAAAAARaXNfd3JhcHBlcl9hY3RpdmUAAAAAAAAAAAAAAQAAAAE=",
  "AAAAAAAAAAAAAAAJZ2V0X2FkbWluAAAAAAAAAAAAAAEAAAAT",
  "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAAAwAAAAAAAAAAAAAABUFkbWluAAAAAAAAAAAAAAAAAAAFQXNzZXQAAAAAAAAAAAAAAAAAAA9Bc3NldENvbnRyb2xsZXIA",
]);
