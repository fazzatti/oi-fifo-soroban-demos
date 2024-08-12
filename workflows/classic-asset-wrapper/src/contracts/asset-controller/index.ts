import { ContractEngine } from "stellar-plus/lib/stellar-plus/core/contract-engine";
import {
  GetAccountProbationPeriodPayload,
  GetAccountProbationPeriodResponse,
  GetAdminPayload,
  GetAdminResponse,
  GetAssetPayload,
  GetAssetResponse,
  GetInflowLimitPayload,
  GetInflowLimitResponse,
  GetOutflowLimitPayload,
  GetOutflowLimitResponse,
  GetProbationPeriodPayload,
  GetProbationPeriodResponse,
  GetQuotaPayload,
  GetQuotaReleaseTimePayload,
  GetQuotaReleaseTimeResponse,
  GetQuotaResponse,
  GetQuotaTimeLimitPayload,
  GetQuotaTimeLimitResponse,
  InitializePayload,
  SetProbationStartPayload,
} from "./types";
import {
  i128,
  TransactionInvocation,
  u64,
} from "stellar-plus/lib/stellar-plus/types";
import { Methods } from "./constants";
import { readTxInvocation } from "@src/utils/admin";
import { ContractEngineConstructorArgs } from "stellar-plus/lib/stellar-plus/core/contract-engine/types";

export class AssetController extends ContractEngine {
  public adminPublicKey: string = "";
  public wrapperContractId: string = "";
  public assetContractId: string = "";
  public probationPeriod: u64 = 0n;
  public quotaTimeLimit: u64 = 0n;
  public inflowLimit: i128 = 0n;
  public outflowLimit: i128 = 0n;

  constructor(args: ContractEngineConstructorArgs) {
    super(args);
  }

  public async initialize(args: TransactionInvocation & InitializePayload) {
    this.adminPublicKey = args.admin;
    this.wrapperContractId = args.wrapper;
    this.assetContractId = args.asset;
    this.probationPeriod = args.probation_period;
    this.quotaTimeLimit = args.quota_time_limit;
    this.inflowLimit = args.inflow_limit;
    this.outflowLimit = args.outflow_limit;

    const txInvocation = args as TransactionInvocation;

    return await this.invokeContract({
      ...txInvocation,
      method: Methods.Initialize,
      methodArgs: {
        admin: this.adminPublicKey,
        wrapper: this.wrapperContractId,
        asset: this.assetContractId,
        probation_period: this.probationPeriod,
        quota_time_limit: this.quotaTimeLimit,
        inflow_limit: this.inflowLimit,
        outflow_limit: this.outflowLimit,
      },
    });
  }

  public async setProbationStart(
    args: TransactionInvocation & SetProbationStartPayload
  ) {
    const txInvocation = args as TransactionInvocation;
    const methodArgs = args as SetProbationStartPayload;
    return await this.invokeContract({
      ...txInvocation,
      method: Methods.SetProbationStart,
      methodArgs,
    });
  }

  public async getQuota(id: string): Promise<GetQuotaResponse> {
    return (await this.readFromContract({
      method: Methods.GetQuota,
      methodArgs: { id } as GetQuotaPayload,
      ...readTxInvocation(),
    })) as GetQuotaResponse;
  }

  public async getQuotaReleaseTime(
    id: string
  ): Promise<GetQuotaReleaseTimeResponse> {
    return (await this.readFromContract({
      method: Methods.GetQuotaReleaseTime,
      methodArgs: { id } as GetQuotaReleaseTimePayload,
      ...readTxInvocation(),
    })) as GetQuotaReleaseTimeResponse;
  }

  public async getAccountProbationPeriod(
    id: string
  ): Promise<GetAccountProbationPeriodResponse> {
    return (await this.readFromContract({
      method: Methods.GetAccountProbationPeriod,
      methodArgs: { id } as GetAccountProbationPeriodPayload,
      ...readTxInvocation(),
    })) as GetAccountProbationPeriodResponse;
  }

  public async getProbationPeriod(): Promise<GetProbationPeriodResponse> {
    return (await this.readFromContract({
      method: Methods.GetProbationPeriod,
      methodArgs: {} as GetProbationPeriodPayload,
      ...readTxInvocation(),
    })) as GetProbationPeriodResponse;
  }

  public async getQuotaTimeLimit(): Promise<GetQuotaTimeLimitResponse> {
    return (await this.readFromContract({
      method: Methods.GetQuotaTimeLimit,
      methodArgs: {} as GetQuotaTimeLimitPayload,
      ...readTxInvocation(),
    })) as GetQuotaTimeLimitResponse;
  }

  public async getInflowLimit(): Promise<GetInflowLimitResponse> {
    return (await this.readFromContract({
      method: Methods.GetInflowLimit,
      methodArgs: {} as GetInflowLimitPayload,
      ...readTxInvocation(),
    })) as GetInflowLimitResponse;
  }

  public async getOutflowLimit(): Promise<GetOutflowLimitResponse> {
    return (await this.readFromContract({
      method: Methods.GetOutflowLimit,
      methodArgs: {} as GetOutflowLimitPayload,
      ...readTxInvocation(),
    })) as GetOutflowLimitResponse;
  }

  public async getAsset(): Promise<GetAssetResponse> {
    return (await this.readFromContract({
      method: Methods.GetAsset,
      methodArgs: {} as GetAssetPayload,
      ...readTxInvocation(),
    })) as GetAssetResponse;
  }

  public async getAdmin(): Promise<GetAdminResponse> {
    return (await this.readFromContract({
      method: Methods.GetAdmin,
      methodArgs: {} as GetAdminPayload,
      ...readTxInvocation(),
    })) as GetAdminResponse;
  }
}
