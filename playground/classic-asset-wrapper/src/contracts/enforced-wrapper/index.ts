import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { ContractEngine } from "stellar-plus/lib/stellar-plus/core/contract-engine";
import { Methods } from "./constants";
import { TransactionInvocation } from "stellar-plus/lib/stellar-plus/types";
import {
  ActivateWrapperPayload,
  ActivateWrapperResponse,
  AuthorizedPayload,
  AuthorizedResponse,
  BalancePayload,
  BalanceResponse,
  BurnPayload,
  BurnResponse,
  ClassicWrapperConstructorArgs,
  DeactivateWrapperPayload,
  DeactivateWrapperResponse,
  GetAdminPayload,
  GetAdminResponse,
  GetMetadataPayload,
  GetMetadataResponse,
  InitializePayload,
  IsWrapperActivePayload,
  IsWrapperActiveResponse,
  MintPayload,
  MintResponse,
  SetAdminPayload,
  SetAdminResponse,
  SetAuthorizedPayload,
  TransferPayload,
} from "./types";
import { readTxInvocation } from "@src/utils/admin";
import { SorobanTransactionPipelineOutput } from "stellar-plus/lib/stellar-plus/core/pipelines/soroban-transaction/types";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";

export class EnforcedClassicWrapper extends ContractEngine {
  public sacHandler: SACHandler;

  public adminPublicKey: string = "";
  public asssetControllerContractId: string = "";

  constructor(args: ClassicWrapperConstructorArgs) {
    super(args);

    this.sacHandler = args.asset;
  }

  public async initialize(args: TransactionInvocation & InitializePayload) {
    const txInvocation = args as TransactionInvocation;
    this.adminPublicKey = args.admin;
    this.asssetControllerContractId = args.asset_controller;

    return await this.invokeContract({
      method: Methods.Initialize,
      methodArgs: {
        admin: this.adminPublicKey,
        asset: this.sacHandler.sorobanTokenHandler.getContractId(),
        asset_controller: this.asssetControllerContractId,
      } as InitializePayload,
      ...txInvocation,
    });
  }

  public async activateWrapper(
    args: ActivateWrapperPayload & TransactionInvocation
  ): Promise<ActivateWrapperResponse> {
    const txInvocation: TransactionInvocation = args;
    return (await this.invokeContract({
      method: Methods.ActivateWrapper,
      methodArgs: {} as ActivateWrapperPayload,
      ...txInvocation,
    })) as unknown as ActivateWrapperResponse;
  }

  public async deactivateWrapper(
    args: DeactivateWrapperPayload & TransactionInvocation
  ): Promise<DeactivateWrapperResponse> {
    const txInvocation: TransactionInvocation = args;
    return (await this.invokeContract({
      method: Methods.ActivateWrapper,
      methodArgs: {} as DeactivateWrapperPayload,
      ...txInvocation,
    })) as unknown as DeactivateWrapperResponse;
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

  public async setAdmin(
    args: SetAdminPayload & TransactionInvocation
  ): Promise<SetAdminResponse> {
    const txInvocation: TransactionInvocation = args;
    const res = (await this.invokeContract({
      method: Methods.SetAdmin,
      methodArgs: { new_admin: args.new_admin } as SetAdminPayload,
      ...txInvocation,
    })) as unknown as SetAdminResponse;

    this.adminPublicKey = args.new_admin;

    return res;
  }

  public async getAdmin(): Promise<GetAdminResponse> {
    return (await this.readFromContract({
      method: Methods.GetAdmin,
      methodArgs: {} as GetAdminPayload,
      ...readTxInvocation(),
    })) as unknown as GetAdminResponse;
  }

  public async balance(id: string): Promise<BalanceResponse> {
    return (await this.readFromContract({
      method: Methods.Balance,
      methodArgs: { id } as BalancePayload,
      ...readTxInvocation(),
    })) as BalanceResponse;
  }

  public async authorized(id: string): Promise<AuthorizedResponse> {
    return (await this.readFromContract({
      method: Methods.Authorized,
      methodArgs: { id } as AuthorizedPayload,
      ...readTxInvocation(),
    })) as unknown as AuthorizedResponse;
  }

  public async transfer(
    args: TransferPayload & TransactionInvocation
  ): Promise<SorobanTransactionPipelineOutput> {
    const txInvocation: TransactionInvocation = args;
    return await this.invokeContract({
      method: Methods.Transfer,
      methodArgs: {
        from: args.from,
        to: args.to,
        amount: args.amount,
      } as TransferPayload,
      ...txInvocation,
      options: {
        includeHashOutput: true,
      },
    });
  }

  public async mint(
    args: MintPayload & TransactionInvocation
  ): Promise<MintResponse> {
    const txInvocation: TransactionInvocation = args;
    return (await this.invokeContract({
      method: Methods.Mint,
      methodArgs: { to: args.to, amount: args.amount } as MintPayload,
      ...txInvocation,
    })) as unknown as MintResponse;
  }

  public async burn(
    args: BurnPayload & TransactionInvocation
  ): Promise<BurnResponse> {
    const txInvocation: TransactionInvocation = args;
    return (await this.invokeContract({
      method: Methods.Burn,
      methodArgs: { from: args.from, amount: args.amount } as BurnPayload,
      ...txInvocation,
    })) as unknown as BurnResponse;
  }

  public async setAuthorized(
    args: SetAuthorizedPayload & TransactionInvocation
  ): Promise<SorobanTransactionPipelineOutput> {
    const txInvocation: TransactionInvocation = args;
    return await this.invokeContract({
      method: Methods.SetAuthorized,
      methodArgs: {
        id: args.id,
        authorize: args.authorize,
      } as SetAuthorizedPayload,
      ...txInvocation,
      options: {
        includeHashOutput: true,
      },
    });
  }
}
