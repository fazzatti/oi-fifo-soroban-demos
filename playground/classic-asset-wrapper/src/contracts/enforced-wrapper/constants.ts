import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { Core } from "stellar-plus/lib/stellar-plus/core";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";

export const wasmPath =
  "../../target/wasm32-unknown-unknown/release/enforced_classic_asset_wrapper.wasm";

export enum Methods {
  Initialize = "initialize",
  GetMetadata = "get_metadata",
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
  "AAAAAAAAAAAAAAAQYWN0aXZhdGVfd3JhcHBlcgAAAAAAAAAA",
  "AAAAAAAAAAAAAAASZGVhY3RpdmF0ZV93cmFwcGVyAAAAAAAAAAAAAA==",
  "AAAAAAAAAAAAAAAJc2V0X2FkbWluAAAAAAAAAQAAAAAAAAAJbmV3X2FkbWluAAAAAAAAEwAAAAA=",
  "AAAAAAAAAAAAAAAOc2V0X2F1dGhvcml6ZWQAAAAAAAIAAAAAAAAAAmlkAAAAAAATAAAAAAAAAAlhdXRob3JpemUAAAAAAAABAAAAAA==",
  "AAAAAAAAAAAAAAAIdHJhbnNmZXIAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAACdG8AAAAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAA=",
  "AAAAAAAAAAAAAAAMZ2V0X21ldGFkYXRhAAAAAAAAAAEAAAfQAAAAD1dyYXBwZXJNZXRhZGF0YQA=",
  "AAAAAAAAAAAAAAARaXNfd3JhcHBlcl9hY3RpdmUAAAAAAAAAAAAAAQAAAAE=",
  "AAAAAAAAAAAAAAAJZ2V0X2FkbWluAAAAAAAAAAAAAAEAAAAT",
  "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAAAwAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAAVhc3NldAAAAAAAABMAAAAAAAAAEGFzc2V0X2NvbnRyb2xsZXIAAAATAAAAAA==",
  "AAAAAAAAAAAAAAAEbWludAAAAAIAAAAAAAAAAnRvAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
  "AAAAAgAAAAAAAAAAAAAAC01ldGFkYXRha2V5AAAAAAEAAAAAAAAAAAAAAA9XcmFwcGVyTWV0YWRhZGEA",
  "AAAAAQAAAAAAAAAAAAAAD1dyYXBwZXJNZXRhZGF0YQAAAAAFAAAAAAAAAAVhZG1pbgAAAAAAABMAAAAAAAAABWFzc2V0AAAAAAAAEwAAAAAAAAAQYXNzZXRfY29udHJvbGxlcgAAABMAAAAAAAAACGVuZm9yY2VkAAAAAQAAAAAAAAAJaXNfYWN0aXZlAAAAAAAAAQ==",
]);
