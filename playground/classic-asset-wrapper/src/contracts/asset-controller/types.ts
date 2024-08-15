import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";
import { i128, u32, u64 } from "stellar-plus/lib/stellar-plus/types";

export interface AllowanceValue {
  amount: i128;
  expiration_ledger: u32;
}

export interface AccountActivityData {
  inflow: Array<TxEntry>;
  outflow: Array<TxEntry>;
}

export interface TxEntry {
  amount: i128;
  timestamp: u64;
}

export interface AccountQuotaReleaseData {
  inflow: Array<TxReleaseEntry>;
  outflow: Array<TxReleaseEntry>;
}

export interface TxReleaseEntry {
  amount: i128;
  time_left: u64;
}

export type InitializePayload = {
  admin: string;
  wrapper: string;
  asset: string;
  probation_period: u64;
  quota_time_limit: u64;
  inflow_limit: i128;
  outflow_limit: i128;
};
export type InitializeResponse = null;

export type SetProbationStartPayload = {
  id: string;
  probation_start: u64;
  reset_quotas: boolean;
};
export type SetProbationStartResponse = null;

export type ReviewTransferPayload = { from: string; to: string; amount: i128 };
export type ReviewTransferResponse = null;

export type GetQuotaPayload = { id: string };
export type GetQuotaResponse = Array<i128>;

export type GetAccountProbationPeriodPayload = { id: string };
export type GetAccountProbationPeriodResponse = u64;

export type GetQuotaReleaseTimePayload = { id: string };
export type GetQuotaReleaseTimeResponse = AccountQuotaReleaseData;

export type GetQuotaTimeLimitPayload = {};
export type GetQuotaTimeLimitResponse = u64;

export type GetInflowLimitPayload = {};
export type GetInflowLimitResponse = i128;

export type GetOutflowLimitPayload = {};
export type GetOutflowLimitResponse = i128;

export type GetAssetPayload = {};
export type GetAssetResponse = string;

export type GetAdminPayload = {};
export type GetAdminResponse = string;

export type GetProbationPeriodPayload = {};
export type GetProbationPeriodResponse = u64;
