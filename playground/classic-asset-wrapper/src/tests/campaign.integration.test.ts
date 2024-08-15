import { Campaign } from "@src/contracts/campaign";
import {
  wasmPath as campaignWasmPath,
  spec as campaignSpec,
} from "@src/contracts/campaign/constants";
import { UserData } from "@src/contracts/campaign/types";
import { OptionalClassicWrapper } from "@src/contracts/optional-wrapper";
import {
  wasmPath as optionalWasmPath,
  spec as optionalSpec,
} from "@src/contracts/optional-wrapper/constants";
import {
  adminTxInvocation,
  getAdmin,
  readTxInvocation,
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

describe("End-to-end Optiopnal Classic Wrapper Test with Campaign", () => {
  let sac: SACHandler;
  let wrapper: OptionalClassicWrapper;
  let campaign: Campaign;
  let admin: DefaultAccountHandler;
  let networkConfig: NetworkConfig;

  beforeAll(async () => {
    const wrapperWasm = await loadWasmFile(optionalWasmPath);
    const campaignWasm = await loadWasmFile(campaignWasmPath);

    networkConfig = TestNet();
    admin = await getAdmin();
    sac = new SACHandler({ networkConfig, code: "FIFO", issuerAccount: admin });
    wrapper = new OptionalClassicWrapper({
      networkConfig,
      contractParameters: {
        wasm: wrapperWasm,
        spec: optionalSpec,
      },
    });
    campaign = new Campaign({
      networkConfig,
      contractParameters: {
        wasm: campaignWasm,
        spec: campaignSpec,
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
    it("should upload the Campaign contract", async () => {
      await expect(
        campaign.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(campaign.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should upload the Classic Wrapper contract", async () => {
      await expect(
        wrapper.uploadWasm(adminTxInvocation())
      ).resolves.toBeDefined();

      expect(wrapper.getWasmHash()).toMatch(wasmHashRegex);
    });

    it("should deploy a new instance of the Asset Controller contract", async () => {
      await expect(campaign.deploy(adminTxInvocation())).resolves.toBeDefined();

      expect(campaign.getContractId()).toMatch(contractIdRegex);
    });

    it("should deploy a new instance of the Classic Wrapper contract", async () => {
      await expect(wrapper.deploy(adminTxInvocation())).resolves.toBeDefined();

      expect(wrapper.getContractId()).toMatch(contractIdRegex);
    });

    it("should initialize the Campaign contract with the proper parameters", async () => {
      await expect(
        campaign.initialize({
          ...adminTxInvocation(),
          admin: admin.getPublicKey(),
          wrapper: wrapper.getContractId(),
          asset: sac.sorobanTokenHandler.getContractId(),
          prize_amount: BigInt(10 * 10 ** 7),
          inflow_points: BigInt(100),
          outflow_points: BigInt(150),
          target_points: BigInt(1000 * 10 ** 7),
          wait_interval: BigInt(10),
          end_date: BigInt(Date.now() + 3600 * 1000),
        })
      ).resolves.toBeDefined();
    });

    it("should initialize the Wrapper contract with the proper parameters", async () => {
      await expect(
        wrapper.initialize({
          ...adminTxInvocation(),
          admin: admin.getPublicKey(),
          asset: sac.sorobanTokenHandler.getContractId(),
          asset_controller: campaign.getContractId(),
        })
      ).resolves.toBeDefined();
    });
    it("admin should fund the campaign", async () => {
      await expect(
        campaign.addFunds({
          ...adminTxInvocation(),
          amount: BigInt(10000 * 10 ** 7),
        })
      ).resolves.toBeDefined();

      const balance = await sac.sorobanTokenHandler.balance({
        id: campaign.getContractId(),
        ...readTxInvocation(),
      });

      expect(balance.toString()).toBe(BigInt(10000 * 10 ** 7).toString());
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
      it("User A & B >> should not have points before transacting", async () => {
        const userAData = await campaign.getUser({
          user: userA.getPublicKey(),
        });
        const userBData = await campaign.getUser({
          user: userB.getPublicKey(),
        });

        expect(userAData).toStrictEqual({
          points: 0n,
          wait_until: 0n,
        } as UserData);
        expect(userBData).toStrictEqual({
          points: 0n,
          wait_until: 0n,
        } as UserData);
      });

      it("User A >> should transfer to B and both accumlate points", async () => {
        await expect(
          wrapper.transfer({
            from: userA.getPublicKey(),
            to: userB.getPublicKey(),
            amount: 100_0000000n,
            ...simpleTxInvocation(userA),
          })
        ).resolves.toBeDefined();

        await expect(
          campaign.getUser({ user: userA.getPublicKey() })
        ).resolves.toStrictEqual({
          points: 150_0000000n,
          wait_until: 0n,
        } as UserData);
        await expect(
          campaign.getUser({ user: userB.getPublicKey() })
        ).resolves.toStrictEqual({
          points: 100_0000000n,
          wait_until: 0n,
        } as UserData);
      });

      it("User A >> should transfer to B to complete points and get prize", async () => {
        await expect(
          wrapper.transfer({
            from: userA.getPublicKey(),
            to: userB.getPublicKey(),
            amount: 600_0000000n,
            ...simpleTxInvocation(userA),
          })
        ).resolves.toBeDefined();

        const userAData = await campaign.getUser({
          user: userA.getPublicKey(),
        });

        expect(userAData.points).toBe(0n);
        expect(userAData.wait_until).toBeGreaterThan(0n);

        await expect(
          sac.classicHandler.balance(userA.getPublicKey())
        ).resolves.toBe(310);

        await expect(
          campaign.getUser({ user: userB.getPublicKey() })
        ).resolves.toStrictEqual({
          points: 700_0000000n,
          wait_until: 0n,
        } as UserData);
      });
    });
  });
});
