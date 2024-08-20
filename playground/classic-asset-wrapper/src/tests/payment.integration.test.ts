import { EnforcedClassicWrapper } from "@src/contracts/enforced-wrapper";
import { Probation } from "@src/contracts/probation";
import {
  adminTxInvocation,
  getAdmin,
  simpleTxInvocation,
} from "@src/utils/admin";
import { loadWasmFile } from "@src/utils/wasm";
import { DefaultAccountHandler } from "stellar-plus/lib/stellar-plus/account";
import { SACHandler } from "stellar-plus/lib/stellar-plus/asset";
import { NetworkConfig, TestNet } from "stellar-plus/lib/stellar-plus/network";
import {
  wasmPath as probationWasmPath,
  spec as acSpec,
} from "@src/contracts/probation/constants";
import {
  spec as wrSpec,
  wasmPath as wrapperWasmPath,
} from "@src/contracts/enforced-wrapper/constants";
import {
  wasmPath as paymentWasmPath,
  spec as paymentSpec,
} from "@src/contracts/payment/constants.";
import { PaymentContract } from "@src/contracts/payment";
import {
  contractIdRegex,
  wasmHashRegex,
} from "stellar-plus/lib/stellar-plus/utils/regex";
import { StellarPlusError } from "stellar-plus/lib/stellar-plus/error";

describe("Cross-contract Payment using Enforced Wrapper testing", () => {
  let sac: SACHandler;
  let wrapper: EnforcedClassicWrapper;
  let probation: Probation;
  let admin: DefaultAccountHandler;
  let networkConfig: NetworkConfig;
  let payment: PaymentContract;

  beforeAll(async () => {
    const wrapperWasm = await loadWasmFile(wrapperWasmPath);
    const probationWasm = await loadWasmFile(probationWasmPath);
    const paymentWasm = await loadWasmFile(paymentWasmPath);

    networkConfig = TestNet();
    admin = await getAdmin();
    sac = new SACHandler({
      networkConfig,
      code: "FIFOPAY",
      issuerAccount: admin,
    });
    wrapper = new EnforcedClassicWrapper({
      networkConfig,
      contractParameters: {
        wasm: wrapperWasm,
        spec: wrSpec,
      },
      asset: sac,
    });
    probation = new Probation({
      networkConfig,
      contractParameters: {
        wasm: probationWasm,
        spec: acSpec,
      },
    });

    payment = new PaymentContract({
      networkConfig,
      contractParameters: {
        wasm: paymentWasm,
        spec: paymentSpec,
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
        probation.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(probation.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should upload the Classic Wrapper contract", async () => {
      await expect(
        wrapper.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(wrapper.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should upload the Payment contract", async () => {
      await expect(
        payment.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(payment.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should deploy a new instance of the Asset Controller contract", async () => {
      await expect(
        probation.deploy(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(probation.getContractId()).toMatch(contractIdRegex);
    });

    it("should deploy a new instance of the Classic Wrapper contract", async () => {
      await expect(wrapper.deploy(adminTxInvocation())).resolves.toBeDefined();

      expect(wrapper.getContractId()).toMatch(contractIdRegex);
    });

    it("should deploy a new instance of the Payment contract", async () => {
      await expect(payment.deploy(adminTxInvocation())).resolves.toBeDefined();

      expect(payment.getContractId()).toMatch(contractIdRegex);
    });

    it("should initialize the Asset Controller contract with the proper parameters", async () => {
      await expect(
        probation.initialize({
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

      const fetchedProbationPeriod = await probation.getProbationPeriod();
      const fetchedQuotaTimeLimit = await probation.getQuotaTimeLimit();
      const fetchedInflowLimit = await probation.getInflowLimit();
      const fetchedOutflowLimit = await probation.getOutflowLimit();
      const fetchedAdmin = await probation.getAdmin();
      const fetchedAsset = await probation.getAsset();

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
          asset_controller: probation.getContractId(),
          asset: sac.sorobanTokenHandler.getContractId(),
        })
      ).resolves.toBeDefined();

      const fetchedAdmin = await wrapper.getAdmin();

      expect(fetchedAdmin).toBe(admin.getPublicKey());
    });

    it("should initialize the Payment contract with the proper parameters", async () => {
      await expect(
        payment.initialize({
          ...adminTxInvocation(),
          asset: wrapper.getContractId(),
        })
      ).resolves.toBeDefined();
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
      it("User A  >> should transfer through Payment to user B", async () => {
        // payment
        //   .pay({
        //     from: userA.getPublicKey(),
        //     to: userB.getPublicKey(),
        //     amount: 100_0000000n,
        //     ...simpleTxInvocation(userA),
        //   })
        //   .catch((e) => {
        //     console.log("ERROR:", e);
        //     console.log("ERROR SP:", e as StellarPlusError);
        //     console.log("ERROR META:", (e as StellarPlusError).meta);
        //     console.log(
        //       "ERROR SIMU:",
        //       (e as StellarPlusError).meta?.sorobanSimulationData
        //     );
        //   });

        await expect(
          payment.pay({
            from: userA.getPublicKey(),
            to: userB.getPublicKey(),
            amount: 100_0000000n,
            ...simpleTxInvocation(userA),
          })
        ).resolves.toBeDefined();

        await expect(
          sac.classicHandler.balance(userA.getPublicKey())
        ).resolves.toBe(900);
        await expect(
          sac.classicHandler.balance(userB.getPublicKey())
        ).resolves.toBe(1100);
      });

      it("User B  >> should transfer through Payment to Payment itself", async () => {
        await expect(
          payment.pay({
            from: userB.getPublicKey(),
            to: payment.getContractId(),
            amount: 100_0000000n,
            ...simpleTxInvocation(userB),
          })
        ).resolves.toBeDefined();

        await expect(
          sac.classicHandler.balance(userB.getPublicKey())
        ).resolves.toBe(1000);

        const contractBalance = await sac.sorobanTokenHandler.balance({
          id: payment.getContractId(),
          ...adminTxInvocation(),
        });
        expect(contractBalance.toString()).toBe(100_0000000n.toString());

        console.log("Admin ID: ", admin.getPublicKey());
        console.log("User B ID: ", userB.getPublicKey());
        console.log("User A ID: ", userA.getPublicKey());
        console.log("Payment ID: ", payment.getContractId());
        console.log("Wrapper ID: ", wrapper.getContractId());
        console.log("Probation ID: ", probation.getContractId());
      });
    });
  });
});
