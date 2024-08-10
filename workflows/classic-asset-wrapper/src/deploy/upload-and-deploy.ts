import { loadWasmFile } from "@src/utils/wasm";
import { DefaultAccountHandler } from "stellar-plus/lib/stellar-plus/account";
import { ContractEngine } from "stellar-plus/lib/stellar-plus/core/contract-engine";
import { TestNet } from "stellar-plus/lib/stellar-plus/network";
async function uploadAndDeploy(filePath: string) {
  console.log("Starting 'Upload and Deploy'...");

  const wasmBuffer = await loadWasmFile(filePath);

  const networkConfig = TestNet();
  const contractEngine = new ContractEngine({
    networkConfig,
    contractParameters: {
      wasm: wasmBuffer,
    },
  });

  const admin = new DefaultAccountHandler({ networkConfig });
  console.log("Initializing Admin...");
  await admin.initializeWithFriendbot();

  const txInvocation = {
    header: {
      source: admin.getPublicKey(),
      fee: "10000000",
      timeout: 45,
    },
    signers: [admin],
  };

  console.log("Uploading contract wasm...");
  await contractEngine.uploadWasm(txInvocation);

  console.log("\nContract Wasm Hash: ", contractEngine.getWasmHash());

  console.log("\nDeploying contract...");
  await contractEngine.deploy(txInvocation);
  console.log("\nContract Id: ", contractEngine.getContractId());
}

const wasmFilePath = process.argv[2];
uploadAndDeploy(wasmFilePath)
  .then((result) => {
    console.log("\nDone");
  })
  .catch((error) => {
    console.error("\nDeployment failed:\n", error);
  });
