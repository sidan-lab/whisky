import { Inter } from "next/font/google";
import Head from "next/head";
import { CardanoWallet, MeshBadge, useWallet } from "@meshsdk/react";
import axios from "axios";
import { applyCborEncoding } from "@meshsdk/core-csl";
import {
  ByteString,
  byteString,
  ConStr0,
  conStr0,
  deserializeAddress,
  MaestroProvider,
  PubKeyHash,
  pubKeyHash,
  resolveScriptHash,
  serializePlutusScript,
  stringToHex,
  UTxO,
} from "@meshsdk/core";
import { parseDatumCbor } from "@meshsdk/core-csl";

const inter = Inter({ subsets: ["latin"] });

const provider = new MaestroProvider({
  network: "Preprod",
  apiKey: process.env.NEXT_PUBLIC_MAESTRO_API_KEY!,
});

const helloWorldScriptRawCompiledCode =
  "59012f010000323232323232322323223232253330083232533300a3371e6eb8c008c030dd5002a4410d48656c6c6f2c20576f726c642100100114a06644646600200200644a66602000229404c94ccc038cdc79bae301200200414a226600600600260240026eb0c034c038c038c038c038c038c038c038c038c02cdd5180098059baa002375c600260166ea801c8c0340045261365653330063370e900018039baa001132533300a00116132533300b300d002149858c94cccccc038004585858584dd7000980580098041baa00116533333300b00110011616161653330033370e900018021baa0011325333007001161325333008300a002149858c94cccccc02c004585858584dd7000980400098029baa0011653333330080011001161616165734aae7555cf2ab9f5742ae895d201";

const helloWorldScriptCbor = applyCborEncoding(helloWorldScriptRawCompiledCode);
const helloWorldScriptAddress = serializePlutusScript({
  code: helloWorldScriptCbor,
  version: "V2",
}).address;

const alwaysSucceedScriptRawCompiledCode =
  "5834010000323232323222533300353330033370e900018021baa3006300730053754002294458526136565734aae7555cf2ba157441";

const alwaysSucceedScriptCbor = applyCborEncoding(
  alwaysSucceedScriptRawCompiledCode
);
const alwaysSucceedPolicyId = resolveScriptHash(alwaysSucceedScriptCbor, "V2");

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
        const datum: ConStr0<[PubKeyHash]> = parseDatumCbor(
          input.output.plutusData
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
        conStr0([byteString(stringToHex("Hello, World!"))])
      ),
      script: {
        scriptCbor: helloWorldScriptCbor,
        languageVersion: "v2",
      },
      myAddress: address,
      inputs,
      collateral,
    });
    const txHex = response.data.txHex;
    const signedTx = await wallet.signTx(txHex);
    const txHash = await wallet.submitTx(signedTx);
    console.log("txHash", txHash);
  };
  const mintTokens = async () => {
    const inputs = await wallet.getUtxos();
    const address = await wallet.getChangeAddress();
    const collateral = (await wallet.getCollateral())[0];

    const response = await whisky.post("/mint_tokens", {
      toMintAsset: { unit: alwaysSucceedPolicyId, quantity: "1" },
      redeemer: JSON.stringify(byteString("")),
      script: { scriptCbor: alwaysSucceedScriptCbor, languageVersion: "v2" },
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
