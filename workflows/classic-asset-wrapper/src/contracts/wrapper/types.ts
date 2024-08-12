import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";
import { i128 } from "stellar-plus/lib/stellar-plus/types";

export type InitializePayload = {
  admin: string;
  asset: string;
  asset_controller: string;
};
export type InitializeResponse = null;

export type ActivateWrapperPayload = {};
export type ActivateWrapperResponse = null;

export type DeactivateWrapperPayload = {};
export type DeactivateWrapperResponse = null;

export type SetAdminPayload = { new_admin: string };
export type SetAdminResponse = null;

export type GetAdminPayload = {};
export type GetAdminResponse = boolean;

export type IsWrapperActivePayload = {};
export type IsWrapperActiveResponse = boolean;

export type BalancePayload = { id: string };
export type BalanceResponse = i128;

export type AuthorizedPayload = { id: string };
export type AuthorizedResponse = boolean;

export type TransferPayload = { from: string; to: string; amount: i128 };
export type TransferResponse = null;

export type MintPayload = { to: string; amount: i128 };
export type MintResponse = null;

export type BurnPayload = { from: string; amount: i128 };
export type BurnResponse = null;

export type SetAuthorizedPayload = { id: string; authorize: boolean };
export type SetAuthorizedResponse = null;

export type ClassicWrapperConstructorArgs = ContractEngineConstructorArgs & {
  asset: SACHandler;
};
