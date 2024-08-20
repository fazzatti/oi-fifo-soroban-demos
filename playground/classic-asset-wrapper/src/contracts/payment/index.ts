import { ContractEngine } from "stellar-plus/lib/stellar-plus/core/contract-engine";
import {
  BaseInvocation,
  ContractEngineConstructorArgs,
} from "stellar-plus/lib/stellar-plus/core/contract-engine/types";
import { Methods } from "./constants.";
import { InitializePayload, PayPayload } from "./types";

export class PaymentContract extends ContractEngine {
  constructor(args: ContractEngineConstructorArgs) {
    super(args);
  }

  async initialize(args: BaseInvocation & InitializePayload) {
    return this.invokeContract({
      method: Methods.Initialize,
      methodArgs: args as InitializePayload,
      ...(args as BaseInvocation),
    });
  }

  async pay(args: BaseInvocation & PayPayload) {
    return this.invokeContract({
      method: Methods.Pay,
      methodArgs: args as PayPayload,
      ...(args as BaseInvocation),
    });
  }
}
