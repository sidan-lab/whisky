/**
 * The hello world contract taken from MeshJS contract -
 * https://github.com/MeshJS/mesh/blob/main/packages/mesh-contract/src/hello-world/aiken-workspace-v2/plutus.json
 */
import blueprint from "../../plutus.json";

import {
  ConStr0,
  ByteString,
  PubKeyHash,
  SpendingBlueprint,
} from "@meshsdk/core";

const version = "V3";
const networkId = 0; // 0 for testnet; 1 for mainnet
// Every spending validator would compile into an address with an staking key hash
// Recommend replace with your own stake key / script hash
const stakeKeyHash = "";
const isStakeScriptCredential = false;

export class HelloWorldSpendBlueprint extends SpendingBlueprint {
  compiledCode: string;

  constructor() {
    const compiledCode = blueprint.validators[0]!.compiledCode;
    super(version, networkId, stakeKeyHash, isStakeScriptCredential);
    this.compiledCode = compiledCode;
    this.noParamScript(compiledCode);
  }

  datum = (data: Datum): Datum => data;
  redeemer = (data: Redeemer): Redeemer => data;
}

export type Redeemer = ConStr0<[ByteString]>;

export type Datum = ConStr0<[PubKeyHash]>;
