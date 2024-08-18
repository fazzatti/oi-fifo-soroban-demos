import { ContractEngine } from "stellar-plus/lib/stellar-plus/core/contract-engine";
import {
  GetMetadataPayload,
  GetMetadataResponse,
  InitializePayload,
  IsWrapperActivePayload,
  IsWrapperActiveResponse,
  OptionalClassicWrapperConstructorArgs,
  TransferPayload,
} from "./types";
import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { TransactionInvocation } from "stellar-plus/lib/stellar-plus/types";
import { Methods } from "./constants";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";
import { readTxInvocation } from "@src/utils/admin";

export class OptionalClassicWrapper extends ContractEngine {
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

  public async transfer(args: TransactionInvocation & TransferPayload) {
    const txInvocation = args as TransactionInvocation;
    return await this.invokeContract({
      method: Methods.Transfer,
      methodArgs: {
        from: args.from,
        to: args.to,
        amount: args.amount,
      } as TransferPayload,
      ...txInvocation,
    });
  }

  public async getMetadata(): Promise<GetMetadataResponse> {
    return (await this.readFromContract({
      method: Methods.GetMetadata,
      methodArgs: {} as GetMetadataPayload,
      ...readTxInvocation(),
    })) as unknown as GetMetadataResponse;
  }

  public async isWrapperActive(): Promise<IsWrapperActiveResponse> {
    return (await this.readFromContract({
      method: Methods.IsWrapperActive,
      methodArgs: {} as IsWrapperActivePayload,
      ...readTxInvocation(),
    })) as unknown as IsWrapperActiveResponse;
  }
}
