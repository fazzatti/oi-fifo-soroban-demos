import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";
import { i128 } from "stellar-plus/lib/stellar-plus/types";

export interface WrapperMetadata {
  admin: string;
  asset: string;
  asset_controller: string;
  enforced: boolean;
  is_active: boolean;
}

export type InitializePayload = {
  admin: string;
  asset: string;
  asset_controller: string;
};
export type InitializeResponse = null;

export type GetMetadataPayload = {};
export type GetMetadataResponse = WrapperMetadata;

export type IsWrapperActivePayload = {};
export type IsWrapperActiveResponse = boolean;

export type TransferPayload = { from: string; to: string; amount: i128 };
export type TransferResponse = null;

export type OptionalClassicWrapperConstructorArgs =
  ContractEngineConstructorArgs & {
    asset: SACHandler;
    asssetControllerContractId: string;
    adminPublicKey: string;
  };
