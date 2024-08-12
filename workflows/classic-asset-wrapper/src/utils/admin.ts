import {
  AccountHandler,
  DefaultAccountHandler,
} from "stellar-plus/lib/stellar-plus/account";
import { TestNet } from "stellar-plus/lib/stellar-plus/network";
import { TransactionInvocation } from "stellar-plus/lib/stellar-plus/types";

const networkConfig = TestNet();

let adminInstance: DefaultAccountHandler | null = null;
let isAdminInitialized = false;
let adminInitializationPromise: Promise<void> | null = null;

export const simpleTxInvocation = (
  account: AccountHandler
): TransactionInvocation => {
  return {
    header: {
      source: account.getPublicKey(),
      fee: "100000000", // 10 XLM
      timeout: 45,
    },
    signers: [account],
  } as TransactionInvocation;
};

export const adminTxInvocation = () => {
  return simpleTxInvocation(getAdminSync());
};

export const readTxInvocation = (): TransactionInvocation => {
  return {
    ...simpleTxInvocation(getAdminSync()),
    signers: [],
  } as TransactionInvocation;
};

export const getAdmin = async (): Promise<DefaultAccountHandler> => {
  if (isAdminInitialized) {
    return adminInstance!;
  }

  if (!adminInitializationPromise) {
    adminInstance = new DefaultAccountHandler({ networkConfig });
    adminInitializationPromise = adminInstance
      .initializeWithFriendbot()
      .then(() => {
        isAdminInitialized = true;
      })
      .catch((error) => {
        adminInstance = null;
        adminInitializationPromise = null;
        throw error;
      });
  }

  await adminInitializationPromise;
  return adminInstance!;
};

export const getAdminSync = (): DefaultAccountHandler => {
  if (!isAdminInitialized) {
    throw new Error("Admin is not initialized yet. Call getAdmin() first.");
  }
  return adminInstance!;
};

export default getAdmin().then((admin) => {
  return admin;
});
