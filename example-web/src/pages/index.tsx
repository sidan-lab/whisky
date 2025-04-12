import { Inter } from "next/font/google";
import Head from "next/head";
import { CardanoWallet, MeshBadge, useWallet } from "@meshsdk/react";
import axios from "axios";
import {
  applyCborEncoding,
  byteString,
  conStr0,
  deserializeAddress,
  deserializeDatum,
  MaestroProvider,
  pubKeyHash,
  resolveScriptHash,
  stringToHex,
} from "@meshsdk/core";
import { HelloWorldSpendBlueprint } from "../components/blueprint";

const inter = Inter({ subsets: ["latin"] });

const provider = new MaestroProvider({
  network: "Preprod",
  apiKey: process.env.NEXT_PUBLIC_MAESTRO_API_KEY!,
});

const blueprint = new HelloWorldSpendBlueprint();

const helloWorldScriptCbor = blueprint.cbor;
const helloWorldScriptAddress = blueprint.address;

const alwaysSucceedScriptRawCompiledCode =
  "58340101002332259800a518a4d153300249011856616c696461746f722072657475726e65642066616c736500136564004ae715cd01";

const alwaysSucceedScriptCbor = applyCborEncoding(
  alwaysSucceedScriptRawCompiledCode
);
const alwaysSucceedPolicyId = resolveScriptHash(alwaysSucceedScriptCbor, "V3");

const whisky = axios.create({
  baseURL: "http://127.0.0.1:8080",
  headers: {
    "Content-Type": "application/json",
  },
});

export default function Home() {
  const { wallet } = useWallet();
  const sendLovelace = async () => {
    const inputs = await wallet.getUtxos();
    const address = await wallet.getChangeAddress();
    const response = await whisky.post("/send_lovelace", {
      recipientAddress:
        "addr_test1qqmrzjhtanauj20wg37uk58adyrqfm82a9qr52vdnv0e54r42v0mu8ngky0f5yxmh3wl3z0da2fryk59kavth0u8xhvsufgmc8",
      myAddress: address,
      inputs,
    });
    const txHex = response.data.txHex;
    const signedTx = await wallet.signTx(txHex);
    const txHash = await wallet.submitTx(signedTx);
    console.log("txHash", txHash);
  };

  const lockFund = async () => {
    const inputs = await wallet.getUtxos();
    const address = await wallet.getChangeAddress();
    const ownPubKey = deserializeAddress(address).pubKeyHash;
    const datum = conStr0([pubKeyHash(ownPubKey)]);

    const response = await whisky.post("/lock_fund", {
      scriptAddress: helloWorldScriptAddress,
      datum: JSON.stringify(datum),
      myAddress: address,
      inputs,
    });
    const txHex = response.data.txHex;
    const signedTx = await wallet.signTx(txHex);
    const txHash = await wallet.submitTx(signedTx);
    console.log("txHash", txHash);
  };

  const unlockFund = async () => {
    // "8fb75f27f60e8149a091c749f9712ad59c9d114c457aed1c1acc8d9225d5c662"
    const inputs = await wallet.getUtxos();
    const address = await wallet.getChangeAddress();
    const collateral = (await wallet.getCollateral())[0];

    const ownPubKey = deserializeAddress(address).pubKeyHash;
    const scriptInput = (
      await provider.fetchAddressUTxOs(helloWorldScriptAddress)
    ).find((input) => {
      if (input.output.plutusData) {
        const datum = blueprint.datum(
          deserializeDatum(input.output.plutusData)
        );
        if (datum && datum.fields && datum.fields.length > 0) {
          return datum.fields[0].bytes === ownPubKey;
        }
      }
      return false;
    });

    const response = await whisky.post("/unlock_fund", {
      scriptUtxo: scriptInput,
      redeemer: JSON.stringify(
        blueprint.redeemer(conStr0([byteString(stringToHex("Hello, World!"))]))
      ),
      script: {
        scriptCbor: helloWorldScriptCbor,
        languageVersion: "v3",
      },
      myAddress: address,
      inputs,
      collateral,
    });
    const txHex = response.data.txHex;
    const signedTx = await wallet.signTx(txHex);
    console.log("signedTx", signedTx);

    // const txHash = await wallet.submitTx(signedTx);
    // console.log("txHash", txHash);
  };
  const mintTokens = async () => {
    const inputs = await wallet.getUtxos();
    const address = await wallet.getChangeAddress();
    const collateral = (await wallet.getCollateral())[0];

    const response = await whisky.post("/mint_tokens", {
      toMintAsset: { unit: alwaysSucceedPolicyId, quantity: "1" },
      redeemer: JSON.stringify(byteString("")),
      script: { scriptCbor: alwaysSucceedScriptCbor, languageVersion: "v3" },
      myAddress: address,
      inputs,
      collateral,
    });
    const txHex = response.data.txHex;
    const signedTx = await wallet.signTx(txHex);
    const txHash = await wallet.submitTx(signedTx);
    console.log("txHash", txHash);
  };

  return (
    <div className="bg-gray-900 w-full text-white text-center">
      <Head>
        <title>Mesh App on Cardano</title>
        <meta name="description" content="A Cardano dApp powered my Mesh" />
      </Head>
      <main
        className={`flex min-h-screen flex-col items-center justify-center p-24 ${inter.className} `}>
        <h1 className="text-6xl font-thin mb-20">
          <a href="https://meshjs.dev/" className="text-sky-600">
            Mesh
          </a>{" "}
          Next.js
        </h1>
        <h1 className="text-4xl font-thin mb-20">
          <a href="https://meshjs.dev/" className="text-orange-400">
            Whisky Example
          </a>{" "}
        </h1>

        <div className="mb-20">
          <CardanoWallet />
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 content-center justify-around ">
          <div className="bg-gray-800 rounded-xl border border-white transition max-w-96 p-5 m-5">
            <h2 className="text-2xl font-bold mb-4">Send Lovelace</h2>
            <button
              className="border-white py-2 px-4 border rounded-xl"
              onClick={() => sendLovelace()}>
              Build Tx
            </button>
          </div>
          <div className="bg-gray-800 rounded-xl border border-white transition max-w-96 p-5 m-5">
            <h2 className="text-2xl font-bold mb-4">Lock Fund</h2>
            <button
              className="border-white py-2 px-4 border rounded-xl"
              onClick={() => lockFund()}>
              Build Tx
            </button>
          </div>
          <div className="bg-gray-800 rounded-xl border border-white transition max-w-96 p-5 m-5">
            <h2 className="text-2xl font-bold mb-4">Unlock Fund</h2>
            <button
              className="border-white py-2 px-4 border rounded-xl"
              onClick={() => unlockFund()}>
              Build Tx
            </button>
          </div>
          <div className="bg-gray-800 rounded-xl border border-white transition max-w-96 p-5 m-5">
            <h2 className="text-2xl font-bold mb-4">Mint Assets</h2>
            <button
              className="border-white py-2 px-4 border rounded-xl"
              onClick={() => mintTokens()}>
              Build Tx
            </button>
          </div>
        </div>
      </main>
      <footer className="p-8 border-t border-gray-300 flex justify-center">
        <MeshBadge isDark={true} />
      </footer>
    </div>
  );
}
