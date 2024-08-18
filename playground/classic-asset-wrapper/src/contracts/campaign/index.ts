import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { ContractEngine } from "stellar-plus/lib/stellar-plus/core/contract-engine";
import {
  i128,
  TransactionInvocation,
  u64,
} from "stellar-plus/lib/stellar-plus/types";
import {
  AddFundsPayload,
  GetUserPayload,
  InitializePayload,
  UserData,
} from "./types";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";
import { Methods } from "./constants";
import { readTxInvocation } from "@src/utils/admin";

export class Campaign extends ContractEngine {
  constructor(args: ContractEngineConstructorArgs) {
    super(args);
  }

  public async initialize(args: TransactionInvocation & InitializePayload) {
    const txInvocation = args as TransactionInvocation;
    return await this.invokeContract({
      method: Methods.Initialize,
      methodArgs: {
        ...(args as InitializePayload),
      } as InitializePayload,
      ...txInvocation,
    });
  }

  public async addFunds(args: TransactionInvocation & AddFundsPayload) {
    const txInvocation = args as TransactionInvocation;
    return await this.invokeContract({
      method: Methods.AddFunds,
      methodArgs: {
        amount: args.amount,
      } as AddFundsPayload,
      ...txInvocation,
    });
  }

  public async getUser(args: GetUserPayload): Promise<UserData> {
    return (await this.readFromContract({
      method: Methods.GetUser,
      methodArgs: {
        ...(args as GetUserPayload),
      } as GetUserPayload,
      ...readTxInvocation(),
    })) as unknown as UserData;
  }
}
