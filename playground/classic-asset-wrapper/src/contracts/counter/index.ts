import { ContractEngine } from "stellar-plus/lib/stellar-plus/core/contract-engine";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";
import { u64 } from "stellar-plus/lib/stellar-plus/types";
import {
  GetEvenCountPayload,
  GetEvenCountResponse,
  GetOddCountPayload,
  GetOddCountResponse,
} from "./types";
import { Methods } from "./constants";
import { readTxInvocation } from "@src/utils/admin";

export class Counter extends ContractEngine {
  constructor(args: ContractEngineConstructorArgs) {
    super(args);
  }

  public async getEvenCount(): Promise<GetEvenCountResponse> {
    return (await this.readFromContract({
      method: Methods.GetEvenCount,
      methodArgs: {} as GetEvenCountPayload,
      ...readTxInvocation(),
    })) as unknown as GetEvenCountResponse;
  }

  public async getOddCount(): Promise<GetOddCountResponse> {
    return (await this.readFromContract({
      method: Methods.GetOddCount,
      methodArgs: {} as GetOddCountPayload,
      ...readTxInvocation(),
    })) as unknown as GetOddCountResponse;
  }
}
