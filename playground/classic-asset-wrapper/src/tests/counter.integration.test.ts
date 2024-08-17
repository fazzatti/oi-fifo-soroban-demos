import { Counter } from "@src/contracts/counter";
import {
  wasmPath as counterWasmPath,
  spec as counterSpec,
} from "@src/contracts/counter/constants";
import { OptionalClassicWrapper } from "@src/contracts/optional-wrapper";
import {
  wasmPath as optionalWasmPath,
  spec as optionalSpec,
} from "@src/contracts/optional-wrapper/constants";
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
  contractIdRegex,
  wasmHashRegex,
} from "stellar-plus/lib/stellar-plus/utils/regex";

describe("End-to-end Optional Classic Wrapper Test with Counter", () => {
  let sac: SACHandler;
  let wrapper: OptionalClassicWrapper;
  let counter: Counter;
  let admin: DefaultAccountHandler;
  let networkConfig: NetworkConfig;

  beforeAll(async () => {
    networkConfig = TestNet();

    const wrapperWasm = await loadWasmFile(optionalWasmPath);
    const counterWasm = await loadWasmFile(counterWasmPath);

    admin = await getAdmin();
    sac = new SACHandler({
      networkConfig,
      code: "FIFO",
      issuerAccount: admin,
    });

    wrapper = new OptionalClassicWrapper({
      networkConfig,
      contractParameters: {
        wasm: wrapperWasm,
        spec: optionalSpec,
      },
    });

    counter = new Counter({
      networkConfig,
      contractParameters: {
        wasm: counterWasm,
        spec: counterSpec,
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
  });

  describe("During Setup", () => {
    it("should upload the Counter contract", async () => {
      await expect(
        counter.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(counter.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should upload the Classic Wrapper contract", async () => {
      await expect(
        wrapper.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(wrapper.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should deploy a new instance of the Asset Controller contract", async () => {
      await expect(counter.deploy(adminTxInvocation())).resolves.toBeDefined();

      expect(counter.getContractId()).toMatch(contractIdRegex);
    });

    it("should deploy a new instance of the Classic Wrapper contract", async () => {
      await expect(wrapper.deploy(adminTxInvocation())).resolves.toBeDefined();

      expect(wrapper.getContractId()).toMatch(contractIdRegex);
    });

    it("should initialize the Wrapper contract with the proper parameters", async () => {
      await expect(
        wrapper.initialize({
          ...adminTxInvocation(),
          admin: admin.getPublicKey(),
          asset: sac.sorobanTokenHandler.getContractId(),
          asset_controller: counter.getContractId(),
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

      it("admin >> should be able to mint units through the sac to the user and reflect the balance on classic", async () => {
        await expect(
          sac.sorobanTokenHandler.mint({
            to: userA.getPublicKey(),
            amount: 1_000_0000000n,
            ...adminTxInvocation(),
          })
        ).resolves.toBeDefined();
        await expect(
          sac.sorobanTokenHandler.mint({
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
      it("should be able to call the wrapper contract to perform transfers and track the odd and even transaction", async () => {
        await expect(
          wrapper.transfer({
            from: userA.getPublicKey(),
            to: userB.getPublicKey(),
            amount: 100n,
            ...simpleTxInvocation(userA),
          })
        ).resolves.toBeDefined();

        let evenCount = await counter.getEvenCount();
        expect(evenCount.toString()).toEqual("1");

        await expect(
          wrapper.transfer({
            from: userA.getPublicKey(),
            to: userB.getPublicKey(),
            amount: 2n,
            ...simpleTxInvocation(userA),
          })
        ).resolves.toBeDefined();

        evenCount = await counter.getEvenCount();
        expect(evenCount.toString()).toEqual("2");

        await expect(
          wrapper.transfer({
            from: userB.getPublicKey(),
            to: userA.getPublicKey(),
            amount: 80n,
            ...simpleTxInvocation(userB),
          })
        ).resolves.toBeDefined();

        evenCount = await counter.getEvenCount();
        expect(evenCount.toString()).toEqual("3");

        let oddCount = await counter.getOddCount();
        expect(oddCount.toString()).toEqual("0");

        await expect(
          wrapper.transfer({
            from: userB.getPublicKey(),
            to: userA.getPublicKey(),
            amount: 1n,
            ...simpleTxInvocation(userB),
          })
        ).resolves.toBeDefined();

        oddCount = await counter.getOddCount();
        expect(oddCount.toString()).toEqual("1");

        await expect(
          wrapper.transfer({
            from: userA.getPublicKey(),
            to: userB.getPublicKey(),
            amount: 10n,
            ...simpleTxInvocation(userA),
          })
        ).resolves.toBeDefined();

        oddCount = await counter.getOddCount();
        evenCount = await counter.getEvenCount();
        expect(oddCount.toString()).toEqual("1");
        expect(evenCount.toString()).toEqual("4");
      });
    });
  });
});
