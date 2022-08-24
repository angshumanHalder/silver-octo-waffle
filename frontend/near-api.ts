import { connect, ConnectConfig, Contract, keyStores, Near, WalletConnection } from "near-api-js";
import { Gas, NEAR } from "near-units";
import { getConfig } from "./near-config";

const nearConfig = getConfig(process.env.NODE_ENV || "development");
const config = Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() }, headers: {} }, nearConfig);
const near = new Near(config);

export async function initContract() {
  window.walletConnection = new WalletConnection(near, "silver-octo-waffle");
  window.accountId = window.walletConnection.getAccountId();

  // window.contract = new Contract(window.walletConnection.account(), nearConfig.contractName!, {
  //   viewMethods: ["get_greeting"],
  //   changeMethods: ["set_greeting"],
  // });
}

export function signOutNearWallet() {
  window.walletConnection.signOut();
  window.location.replace(window.location.origin + window.location.pathname);
}

export function signInWithNearWallet() {
  window.walletConnection.requestSignIn(nearConfig.contractName);
}

export async function view(
  method: string,
  args: Record<string, any> = {},
  options: {
    parse?: (response: Uint8Array) => any;
    stringify?: (input: any) => Buffer;
  } = {}
): Promise<any> {
  const account = await near.account(nearConfig.contractName!);
  return account.viewFunction(nearConfig.contractName!, method, args, options);
}

export async function call(
  method: string,
  args: Record<string, any> = {},
  options?: {
    gas?: Gas;
    attachedDeposit?: NEAR;
    walletMeta?: string;
    walletCallbackUrl?: string;
    stringify?: (input: any) => Buffer;
  }
): Promise<any> {
  const currentUser = window.walletConnection.account();
  if (!currentUser) {
    throw new Error("Must sign in before calling a change method");
  }
  return await currentUser.functionCall({
    contractId: nearConfig.contractName!,
    methodName: method,
    args,
    ...(options ?? {}),
  });
}
