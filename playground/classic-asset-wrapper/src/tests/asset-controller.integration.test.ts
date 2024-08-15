import { AssetController } from "@src/contracts/asset-controller";
import {
  wasmPath as assetControllerWasmPath,
  spec as acSpec,
} from "@src/contracts/asset-controller/constants";
import { EnforcedClassicWrapper as ClassicWrapper } from "@src/contracts/enforced-wrapper";
import {
  spec as wrSpec,
  wasmPath as wrapperWasmPath,
} from "@src/contracts/enforced-wrapper/constants";
import {
  adminTxInvocation,
  getAdmin,
  simpleTxInvocation,
} from "@src/utils/admin";
import { loadWasmFile } from "@src/utils/wasm";
import { DefaultAccountHandler } from "stellar-plus/lib/stellar-plus/account";
import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { StellarPlusError } from "stellar-plus/lib/stellar-plus/error";
import { NetworkConfig, TestNet } from "stellar-plus/lib/stellar-plus/network";
import {
  contractIdRegex,
  wasmHashRegex,
} from "stellar-plus/lib/stellar-plus/utils/regex";

describe("End-to-end Classic Wrapper Test with Asset Controller", () => {
  let sac: SACHandler;
  let wrapper: ClassicWrapper;
  let assetController: AssetController;
  let admin: DefaultAccountHandler;
  let networkConfig: NetworkConfig;

  beforeAll(async () => {
    const wrapperWasm = await loadWasmFile(wrapperWasmPath);
    const assetControllerWasm = await loadWasmFile(assetControllerWasmPath);

    networkConfig = TestNet();
    admin = await getAdmin();
    sac = new SACHandler({ networkConfig, code: "FIFO", issuerAccount: admin });
    wrapper = new ClassicWrapper({
      networkConfig,
      contractParameters: {
        wasm: wrapperWasm,
        spec: wrSpec,
      },
      asset: sac,
    });
    assetController = new AssetController({
      networkConfig,
      contractParameters: {
        wasm: assetControllerWasm,
        spec: acSpec,
      },
    });
  });
  describe("Before the contracts Setup", () => {
    it("should wrap the asset in an SAC(Stellar Asset Contract)", async () => {
      await expect(
        sac.wrapAndDeploy(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(sac.sorobanTokenHandler.getContractId()).toMatch(contractIdRegex);
    });

    it("should set the revocable and auth required flags for the asset", async () => {
      await expect(
        sac.classicHandler.setFlags({
          ...adminTxInvocation(),
          controlFlags: {
            authorizationRequired: true,
            authorizationRevocable: true,
          },
        })
      ).resolves.toBeDefined();
    });
  });

  describe("During Setup", () => {
    it("should upload the Asset Controller contract", async () => {
      await expect(
        assetController.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(assetController.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should upload the Classic Wrapper contract", async () => {
      await expect(
        wrapper.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(wrapper.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should deploy a new instance of the Asset Controller contract", async () => {
      await expect(
        assetController.deploy(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(assetController.getContractId()).toMatch(contractIdRegex);
    });

    it("should deploy a new instance of the Classic Wrapper contract", async () => {
      await expect(wrapper.deploy(adminTxInvocation())).resolves.toBeDefined();

      expect(wrapper.getContractId()).toMatch(contractIdRegex);
    });

    it("should initialize the Asset Controller contract with the proper parameters", async () => {
      await expect(
        assetController.initialize({
          ...adminTxInvocation(),
          admin: admin.getPublicKey(),
          wrapper: wrapper.getContractId(),
          asset: sac.sorobanTokenHandler.getContractId(),
          probation_period: 600n,
          quota_time_limit: 300n,
          inflow_limit: 100_0000000n,
          outflow_limit: 100_0000000n,
        })
      ).resolves.toBeDefined();

      const fetchedProbationPeriod = await assetController.getProbationPeriod();
      const fetchedQuotaTimeLimit = await assetController.getQuotaTimeLimit();
      const fetchedInflowLimit = await assetController.getInflowLimit();
      const fetchedOutflowLimit = await assetController.getOutflowLimit();
      const fetchedAdmin = await assetController.getAdmin();
      const fetchedAsset = await assetController.getAsset();

      expect(fetchedProbationPeriod.toString()).toBe(600n.toString());
      expect(fetchedQuotaTimeLimit.toString()).toBe(300n.toString());
      expect(fetchedInflowLimit.toString()).toBe(100_0000000n.toString());
      expect(fetchedOutflowLimit.toString()).toBe(100_0000000n.toString());
      expect(fetchedAdmin).toBe(admin.getPublicKey());
      expect(fetchedAsset).toBe(sac.sorobanTokenHandler.getContractId());
    });

    it("should initialize the Cassic Wrapper contract with the proper parameters", async () => {
      await expect(
        wrapper.initialize({
          ...adminTxInvocation(),
          admin: admin.getPublicKey(),
          asset_controller: assetController.getContractId(),
          asset: sac.sorobanTokenHandler.getContractId(),
        })
      ).resolves.toBeDefined();

      const fetchedAdmin = await wrapper.getAdmin();

      expect(fetchedAdmin).toBe(admin.getPublicKey());
    });
  });

  describe("Once the contracts are initialized", () => {
    let userA: DefaultAccountHandler;
    let userB: DefaultAccountHandler;
    describe("During the setup of two users", () => {
      beforeAll(async () => {
        userA = new DefaultAccountHandler({ networkConfig });
        userB = new DefaultAccountHandler({ networkConfig });

        await userA.initializeWithFriendbot();
        await userB.initializeWithFriendbot();
      });

      it("users >> should be able to create their trustlines", async () => {
        await expect(
          sac.classicHandler.addTrustline({
            to: userA.getPublicKey(),
            ...simpleTxInvocation(userA),
          })
        ).resolves.toBeDefined();
        await expect(
          sac.classicHandler.addTrustline({
            to: userB.getPublicKey(),
            ...simpleTxInvocation(userB),
          })
        ).resolves.toBeDefined();
      });

      it("admin >> should be able to mint units through the wrapper to the user and reflect the balance on classic", async () => {
        await expect(
          wrapper.mint({
            to: userA.getPublicKey(),
            amount: 1_000_0000000n,
            ...adminTxInvocation(),
          })
        ).resolves.toBeDefined();
        await expect(
          wrapper.mint({
            to: userB.getPublicKey(),
            amount: 1_000_0000000n,
            ...adminTxInvocation(),
          })
        ).resolves.toBeDefined();

        await expect(
          sac.classicHandler.balance(userA.getPublicKey())
        ).resolves.toBe(1_000);
        await expect(
          sac.classicHandler.balance(userB.getPublicKey())
        ).resolves.toBe(1_000);
      });
    });

    describe("After the users are ready", () => {
      it("User A & B >> should not be in probation before starting to interact with the asset directly", async () => {
        const userAprobation = await assetController.getAccountProbationPeriod(
          userA.getPublicKey()
        );
        const userBprobation = await assetController.getAccountProbationPeriod(
          userB.getPublicKey()
        );

        expect(userAprobation.toString()).toBe(600n.toString()); // full probation means they're not in probation
        expect(userBprobation.toString()).toBe(600n.toString()); // full probation means they're not in probation
      });

      it("User A >> should be able to send 100 units to user B through the Wrapper, initiating their probation", async () => {
        await expect(
          wrapper.transfer({
            from: userA.getPublicKey(),
            to: userB.getPublicKey(),
            amount: 100_0000000n,
            ...simpleTxInvocation(userA),
          })
        ).resolves.toBeDefined();

        const userAprobation = await assetController.getAccountProbationPeriod(
          userA.getPublicKey()
        );
        const userBprobation = await assetController.getAccountProbationPeriod(
          userB.getPublicKey()
        );
        const userAQuotas = await assetController.getQuotaReleaseTime(
          userA.getPublicKey()
        );
        const userBQuotas = await assetController.getQuotaReleaseTime(
          userB.getPublicKey()
        );

        expect(Number(userAprobation)).toBeGreaterThan(0);
        expect(Number(userBprobation)).toBeGreaterThan(0);
        expect(userAQuotas.inflow.length).toBe(0);
        expect(userAQuotas.outflow.length).toBe(1);
        expect(userBQuotas.inflow.length).toBe(1);
        expect(userBQuotas.outflow.length).toBe(0);
        await expect(
          sac.classicHandler.balance(userA.getPublicKey())
        ).resolves.toBe(900);
        await expect(
          sac.classicHandler.balance(userB.getPublicKey())
        ).resolves.toBe(1100);
      });

      it("User A >> should not be able to send another 100 units to user B through the Wrapper due to the allocated quota", async () => {
        await expect(
          wrapper.transfer({
            from: userA.getPublicKey(),
            to: userB.getPublicKey(),
            amount: 100_0000000n,
            ...simpleTxInvocation(userA),
          })
        ).rejects.toThrow();
      });

      it("Admin >> should be able to approve the users trustline and finish their probation", async () => {
        await expect(
          wrapper.setAuthorized({
            id: userA.getPublicKey(),
            authorize: true,
            ...adminTxInvocation(),
          })
        ).resolves.toBeDefined();

        await expect(
          wrapper.setAuthorized({
            id: userB.getPublicKey(),
            authorize: true,
            ...adminTxInvocation(),
          })
        ).resolves.toBeDefined();

        const userAprobation = await assetController.getAccountProbationPeriod(
          userA.getPublicKey()
        );
        const userBprobation = await assetController.getAccountProbationPeriod(
          userB.getPublicKey()
        );

        expect(userAprobation.toString()).toBe(600n.toString()); // full probation means they're not in probation
        expect(userBprobation.toString()).toBe(600n.toString()); // full probation means they're not in probation
      });

      it("Admin >> should be able to deauthorize the users trustline and reinstate their probation", async () => {
        await expect(
          wrapper.setAuthorized({
            id: userA.getPublicKey(),
            authorize: false,
            ...adminTxInvocation(),
          })
        ).resolves.toBeDefined();

        await expect(
          wrapper.setAuthorized({
            id: userB.getPublicKey(),
            authorize: false,
            ...adminTxInvocation(),
          })
        ).resolves.toBeDefined();

        const userAprobation = await assetController.getAccountProbationPeriod(
          userA.getPublicKey()
        );
        const userBprobation = await assetController.getAccountProbationPeriod(
          userB.getPublicKey()
        );

        expect(Number(userAprobation)).toBeGreaterThan(0);
        expect(Number(userAprobation)).toBeLessThan(600);
        expect(Number(userBprobation)).toBeGreaterThan(0);
        expect(Number(userBprobation)).toBeLessThan(600);
      });

      it("Admin >> should be able to send tokens and not enter probation", async () => {
        await expect(
          wrapper.transfer({
            to: userA.getPublicKey(),
            from: admin.getPublicKey(),
            amount: 50n,
            ...adminTxInvocation(),
          })
        ).resolves.toBeDefined();

        const adminprobation = await assetController.getAccountProbationPeriod(
          admin.getPublicKey()
        );

        expect(adminprobation.toString()).toBe(600n.toString()); // full probation means they're not in probation
      });
    });
  });
});
