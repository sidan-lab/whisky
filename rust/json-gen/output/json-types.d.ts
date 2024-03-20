export type AddressJSON = string;
export type AssetNameJSON = string;
export type AssetNamesJSON = string[];
export interface AssetsJSON {
  [k: string]: string;
}
export interface AuxiliaryDataJSON {
  metadata?: {
    [k: string]: string;
  } | null;
  native_scripts?: NativeScriptsJSON | null;
  plutus_scripts?: PlutusScriptsJSON | null;
  prefer_alonzo_format: boolean;
}
export type AuxiliaryDataHashJSON = string;
export interface AuxiliaryDataSetJSON {
  [k: string]: AuxiliaryDataJSON;
}
export type BigIntJSON = string;
export type BigNumJSON = string;
export interface BlockJSON {
  auxiliary_data_set: {
    [k: string]: AuxiliaryDataJSON;
  };
  header: HeaderJSON;
  invalid_transactions: number[];
  transaction_bodies: TransactionBodiesJSON;
  transaction_witness_sets: TransactionWitnessSetsJSON;
}
export type BlockHashJSON = string;
export interface BootstrapWitnessJSON {
  attributes: number[];
  chain_code: number[];
  signature: string;
  vkey: VkeyJSON;
}
export type BootstrapWitnessesJSON = BootstrapWitnessJSON[];
export type CertificateJSON = CertificateEnumJSON;
export type CertificateEnumJSON =
  | {
      StakeRegistrationJSON: StakeRegistration;
    }
  | {
      StakeDeregistrationJSON: StakeDeregistration;
    }
  | {
      StakeDelegationJSON: StakeDelegation;
    }
  | {
      PoolRegistrationJSON: PoolRegistration;
    }
  | {
      PoolRetirementJSON: PoolRetirement;
    }
  | {
      GenesisKeyDelegationJSON: GenesisKeyDelegation;
    }
  | {
      MoveInstantaneousRewardsCertJSON: MoveInstantaneousRewardsCert;
    };
export type CertificatesJSON = CertificateJSON[];
export type CostModelJSON = string[];
export interface CostmdlsJSON {
  [k: string]: CostModelJSON;
}
export type DNSRecordAorAAAAJSON = string;
export type DNSRecordSRVJSON = string;
export type DataHashJSON = string;
export type DataOptionJSON =
  | {
      DataHashJSON: string;
    }
  | {
      Data: string;
    };
export type Ed25519KeyHashJSON = string;
export type Ed25519KeyHashesJSON = string[];
export type Ed25519SignatureJSON = string;
export interface ExUnitPricesJSON {
  mem_price: UnitIntervalJSON;
  step_price: UnitIntervalJSON;
}
export interface ExUnitsJSON {
  mem: string;
  steps: string;
}
export interface GeneralTransactionMetadataJSON {
  [k: string]: string;
}
export type GenesisDelegateHashJSON = string;
export type GenesisHashJSON = string;
export type GenesisHashesJSON = string[];
export interface GenesisKeyDelegationJSON {
  genesis_delegate_hash: string;
  genesishash: string;
  vrf_keyhash: string;
}
export interface HeaderJSON {
  body_signature: string;
  header_body: HeaderBodyJSON;
}
export interface HeaderBodyJSON {
  block_body_hash: string;
  block_body_size: number;
  block_number: number;
  issuer_vkey: VkeyJSON;
  leader_cert: HeaderLeaderCertEnumJSON;
  operational_cert: OperationalCertJSON;
  prev_hash?: string | null;
  protocol_version: ProtocolVersionJSON;
  slot: string;
  vrf_vkey: string;
}
export type HeaderLeaderCertEnumJSON =
  | {
      NonceAndLeader: [VRFCertJSON, VRFCert];
    }
  | {
      VrfResult: VRFCertJSON;
    };
export type IntJSON = string;
export type Ipv4JSON = [number, number, number, number];
export type Ipv6JSON = [
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number
];
export type KESVKeyJSON = string;
export type LanguageJSON = LanguageKindJSON;
export type LanguageKindJSON = "PlutusV1" | "PlutusV2";
export type LanguagesJSON = LanguageJSON[];
export type MIREnumJSON =
  | {
      ToOtherPot: string;
    }
  | {
      ToStakeCredentials: {
        [k: string]: ProtocolParamUpdateJSON;
      };
    };
export type MIRPotJSON = "Reserves" | "Treasury";
export interface MIRToStakeCredentialsJSON {
  [k: string]: ProtocolParamUpdateJSON;
}
export type MintJSON = [string, MintAssetsJSON][];
export interface MintAssetsJSON {
  [k: string]: string;
}
export interface MoveInstantaneousRewardJSON {
  pot: MIRPotJSON;
  variant: MIREnumJSON;
}
export interface MoveInstantaneousRewardsCertJSON {
  move_instantaneous_reward: MoveInstantaneousRewardJSON;
}
export interface MultiAssetJSON {
  [k: string]: AssetsJSON;
}
export interface MultiHostNameJSON {
  dns_name: DNSRecordSRVJSON;
}
export type NativeScriptJSON = NativeScript1JSON;
export type NativeScript1JSON =
  | {
      ScriptPubkeyJSON: ScriptPubkey;
    }
  | {
      ScriptAllJSON: ScriptAll;
    }
  | {
      ScriptAnyJSON: ScriptAny;
    }
  | {
      ScriptNOfKJSON: ScriptNOfK;
    }
  | {
      TimelockStartJSON: TimelockStart;
    }
  | {
      TimelockExpiryJSON: TimelockExpiry;
    };
export type NativeScriptsJSON = NativeScriptJSON[];
export type NetworkIdJSON = NetworkIdKindJSON;
export type NetworkIdKindJSON = "Testnet" | "Mainnet";
export interface NonceJSON {
  hash?:
    | [
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number,
        number
      ]
    | null;
}
export interface OperationalCertJSON {
  hot_vkey: string;
  kes_period: number;
  sequence_number: number;
  sigma: string;
}
export type PlutusScriptJSON = string;
export type PlutusScriptsJSON = string[];
export interface PoolMetadataJSON {
  pool_metadata_hash: string;
  url: URLJSON;
}
export type PoolMetadataHashJSON = string;
export interface PoolParamsJSON {
  cost: string;
  margin: UnitIntervalJSON;
  operator: string;
  pledge: string;
  pool_metadata?: PoolMetadataJSON | null;
  pool_owners: Ed25519KeyHashesJSON;
  relays: RelaysJSON;
  reward_account: string;
  vrf_keyhash: string;
}
export interface PoolRegistrationJSON {
  pool_params: PoolParamsJSON;
}
export interface PoolRetirementJSON {
  epoch: number;
  pool_keyhash: string;
}
export interface ProposedProtocolParameterUpdatesJSON {
  [k: string]: ProtocolParamUpdateJSON;
}
export interface ProtocolParamUpdateJSON {
  ada_per_utxo_byte?: string | null;
  collateral_percentage?: number | null;
  cost_models?: CostmdlsJSON | null;
  d?: UnitIntervalJSON | null;
  execution_costs?: ExUnitPricesJSON | null;
  expansion_rate?: UnitIntervalJSON | null;
  extra_entropy?: NonceJSON | null;
  key_deposit?: string | null;
  max_block_body_size?: number | null;
  max_block_ex_units?: ExUnitsJSON | null;
  max_block_header_size?: number | null;
  max_collateral_inputs?: number | null;
  max_epoch?: number | null;
  max_tx_ex_units?: ExUnitsJSON | null;
  max_tx_size?: number | null;
  max_value_size?: number | null;
  min_pool_cost?: string | null;
  minfee_a?: string | null;
  minfee_b?: string | null;
  n_opt?: number | null;
  pool_deposit?: string | null;
  pool_pledge_influence?: UnitIntervalJSON | null;
  protocol_version?: ProtocolVersionJSON | null;
  treasury_growth_rate?: UnitIntervalJSON | null;
}
export interface ProtocolVersionJSON {
  major: number;
  minor: number;
}
export type PublicKeyJSON = string;
export interface RedeemerJSON {
  data: string;
  ex_units: ExUnitsJSON;
  index: string;
  tag: RedeemerTagJSON;
}
export type RedeemerTagJSON = RedeemerTagKindJSON;
export type RedeemerTagKindJSON = "Spend" | "MintJSON" | "Cert" | "Reward";
export type RedeemersJSON = RedeemerJSON[];
export type RelayJSON = RelayEnumJSON;
export type RelayEnumJSON =
  | {
      SingleHostAddrJSON: SingleHostAddr;
    }
  | {
      SingleHostNameJSON: SingleHostName;
    }
  | {
      MultiHostNameJSON: MultiHostName;
    };
export type RelaysJSON = RelayJSON[];
export type RewardAddressJSON = string;
export type RewardAddressesJSON = string[];
export interface ScriptAllJSON {
  native_scripts: NativeScriptsJSON;
}
export interface ScriptAnyJSON {
  native_scripts: NativeScriptsJSON;
}
export type ScriptDataHashJSON = string;
export type ScriptHashJSON = string;
export type ScriptHashesJSON = string[];
export interface ScriptNOfKJSON {
  n: number;
  native_scripts: NativeScriptsJSON;
}
export interface ScriptPubkeyJSON {
  addr_keyhash: string;
}
export type ScriptRefJSON = ScriptRefEnumJSON;
export type ScriptRefEnumJSON =
  | {
      NativeScriptJSON: NativeScript;
    }
  | {
      PlutusScriptJSON: string;
    };
export interface SingleHostAddrJSON {
  ipv4?: Ipv4JSON | null;
  ipv6?: Ipv6JSON | null;
  port?: number | null;
}
export interface SingleHostNameJSON {
  dns_name: DNSRecordAorAAAAJSON;
  port?: number | null;
}
export type StakeCredTypeJSON =
  | {
      Key: string;
    }
  | {
      Script: string;
    };
export type StakeCredentialJSON = StakeCredTypeJSON;
export type StakeCredentialsJSON = StakeCredTypeJSON[];
export interface StakeDelegationJSON {
  pool_keyhash: string;
  stake_credential: StakeCredTypeJSON;
}
export interface StakeDeregistrationJSON {
  stake_credential: StakeCredTypeJSON;
}
export interface StakeRegistrationJSON {
  stake_credential: StakeCredTypeJSON;
}
export interface TimelockExpiryJSON {
  slot: string;
}
export interface TimelockStartJSON {
  slot: string;
}
export interface TransactionJSON {
  auxiliary_data?: AuxiliaryDataJSON | null;
  body: TransactionBodyJSON;
  is_valid: boolean;
  witness_set: TransactionWitnessSetJSON;
}
export type TransactionBodiesJSON = TransactionBodyJSON[];
export interface TransactionBodyJSON {
  auxiliary_data_hash?: string | null;
  certs?: CertificatesJSON | null;
  collateral?: TransactionInputsJSON | null;
  collateral_return?: TransactionOutputJSON | null;
  fee: string;
  inputs: TransactionInputsJSON;
  mint?: MintJSON | null;
  network_id?: NetworkIdJSON | null;
  outputs: TransactionOutputsJSON;
  reference_inputs?: TransactionInputsJSON | null;
  required_signers?: Ed25519KeyHashesJSON | null;
  script_data_hash?: string | null;
  total_collateral?: string | null;
  ttl?: string | null;
  update?: UpdateJSON | null;
  validity_start_interval?: string | null;
  withdrawals?: {
    [k: string]: ProtocolParamUpdateJSON;
  } | null;
}
export type TransactionHashJSON = string;
export interface TransactionInputJSON {
  index: number;
  transaction_id: string;
}
export type TransactionInputsJSON = TransactionInputJSON[];
export type TransactionMetadatumJSON = string;
export interface TransactionOutputJSON {
  address: string;
  amount: ValueJSON;
  plutus_data?: DataOptionJSON | null;
  script_ref?: ScriptRefJSON | null;
}
export type TransactionOutputsJSON = TransactionOutputJSON[];
export interface TransactionUnspentOutputJSON {
  input: TransactionInputJSON;
  output: TransactionOutputJSON;
}
export type TransactionUnspentOutputsJSON = TransactionUnspentOutputJSON[];
export interface TransactionWitnessSetJSON {
  bootstraps?: BootstrapWitnessesJSON | null;
  native_scripts?: NativeScriptsJSON | null;
  plutus_data?: PlutusList | null;
  plutus_scripts?: PlutusScriptsJSON | null;
  redeemers?: RedeemersJSON | null;
  vkeys?: VkeywitnessesJSON | null;
}
export type TransactionWitnessSetsJSON = TransactionWitnessSetJSON[];
export type URLJSON = string;
export interface UnitIntervalJSON {
  denominator: string;
  numerator: string;
}
export interface UpdateJSON {
  epoch: number;
  proposed_protocol_parameter_updates: {
    [k: string]: ProtocolParamUpdateJSON;
  };
}
export interface VRFCertJSON {
  output: number[];
  proof: number[];
}
export type VRFKeyHashJSON = string;
export type VRFVKeyJSON = string;
export interface ValueJSON {
  coin: string;
  multiasset?: MultiAssetJSON | null;
}
export type VkeyJSON = string;
export interface VkeywitnessJSON {
  signature: string;
  vkey: VkeyJSON;
}
export type VkeywitnessesJSON = VkeywitnessJSON[];
export interface WithdrawalsJSON {
  [k: string]: ProtocolParamUpdateJSON;
}
