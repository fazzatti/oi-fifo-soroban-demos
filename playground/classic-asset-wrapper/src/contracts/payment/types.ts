import { i128 } from "stellar-plus/lib/stellar-plus/types";

export type InitializePayload = { asset: string };

export type PayPayload = { from: string; to: string; amount: i128 };
