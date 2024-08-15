import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";
import { i128, u64 } from "stellar-plus/lib/stellar-plus/types";

export type InitializePayload = {
  admin: string;
  asset: string;
  wrapper: string;
  prize_amount: i128;
  inflow_points: i128;
  outflow_points: i128;
  target_points: i128;
  wait_interval: u64;
  end_date: u64;
};

export interface UserData {
  points: i128;
  wait_until: u64;
}

export type InitializeResponse = null;

export type AddFundsPayload = { amount: i128 };
export type AddFundsResponse = null;

export type ReviewTransferPayload = { from: string; to: string; amount: i128 };
export type ReviewTransferResponse = null;

export type GetUserPayload = { user: string };
export type GetUserResponse = UserData;
