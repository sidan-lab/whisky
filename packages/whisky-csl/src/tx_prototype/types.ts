export type AddressJSON = string;
export type URLJSON = string;

export interface AnchorJSON {
  anchor_data_hash: string;
  anchor_url: URLJSON;
}
export type AnchorDataHashJSON = string;
export type AssetNameJSON = string;
export type AssetNamesJSON = string[];
export interface AssetsJSON {
  [k: string]: string;
}
export type NativeScriptJSON =
  | { type: "SCRIPT_PUBKEY"; value: ScriptPubkeyJSON }
  | { type: "SCRIPT_ALL"; value: ScriptAllJSON }
  | { type: "SCRIPT_ANY"; value: ScriptAnyJSON }
  | { type: "SCRIPT_N_OF_K"; value: ScriptNOfKJSON }
  | { type: "TIMELOCK_START"; value: TimelockStartJSON }
  | { type: "TIMELOCK_EXPIRY"; value: TimelockExpiryJSON };

export interface AuxiliaryDataJSON {
  metadata?: TxMetadata | null;
  native_scripts?: NativeScriptJSON[] | null;
  plutus_scripts?: string[] | null;
  prefer_alonzo_format: boolean;
}
export interface ScriptPubkeyJSON {
  addr_keyhash: string;
}
export interface ScriptAllJSON {
  native_scripts: NativeScriptJSON[];
}
export interface ScriptAnyJSON {
  native_scripts: NativeScriptJSON[];
}
export interface ScriptNOfKJSON {
  n: number;
  native_scripts: NativeScriptJSON[];
}
export interface TimelockStartJSON {
  slot: string;
}
export interface TimelockExpiryJSON {
  slot: string;
}
export type AuxiliaryDataHashJSON = string;
export interface AuxiliaryDataSetJSON {
  [k: string]: AuxiliaryDataJSON;
}
export type BigIntJSON = string;
export type BigNumJSON = string;
export type VkeyJSON = string;
export type CertificateJSON =
  | { type: "STAKE_REGISTRATION"; value: StakeRegistrationJSON }
  | { type: "STAKE_DEREGISTRATION"; value: StakeDeregistrationJSON }
  | { type: "STAKE_DELEGATION"; value: StakeDelegationJSON }
  | { type: "POOL_REGISTRATION"; value: PoolRegistrationJSON }
  | { type: "POOL_RETIREMENT"; value: PoolRetirementJSON }
  | { type: "GENESIS_KEY_DELEGATION"; value: GenesisKeyDelegationJSON }
  | {
      type: "MOVE_INSTANTANEOUS_REWARDS_CERT";
      value: MoveInstantaneousRewardsCertJSON;
    }
  | { type: "COMMITTEE_HOT_AUTH"; value: CommitteeHotAuthJSON }
  | { type: "COMMITTEE_COLD_RESIGN"; value: CommitteeColdResignJSON }
  | { type: "DREP_DEREGISTRATION"; value: DRepDeregistrationJSON }
  | { type: "DREP_REGISTRATION"; value: DRepRegistrationJSON }
  | { type: "DREP_UPDATE"; value: DRepUpdateJSON }
  | { type: "STAKE_AND_VOTE_DELEGATION"; value: StakeAndVoteDelegationJSON }
  | {
      type: "STAKE_REGISTRATION_AND_DELEGATION";
      value: StakeRegistrationAndDelegationJSON;
    }
  | {
      type: "STAKE_VOTE_REGISTRATION_AND_DELEGATION";
      value: StakeVoteRegistrationAndDelegationJSON;
    }
  | { type: "VOTE_DELEGATION"; value: VoteDelegationJSON }
  | {
      type: "VOTE_REGISTRATION_AND_DELEGATION";
      value: VoteRegistrationAndDelegationJSON;
    };
export type CredTypeJSON =
  | { type: "SCRIPT"; value: string }
  | { type: "KEY"; value: string };
export type RelayJSON =
  | { type: "SINGLE_HOST_ADDR"; value: SingleHostAddrJSON }
  | { type: "SINGLE_HOST_NAME"; value: SingleHostNameJSON }
  | { type: "MULTI_HOST_NAME"; value: MultiHostNameJSON };
/**
 * @minItems 4
 * @maxItems 4
 */
export type Ipv4JSON = [number, number, number, number];
/**
 * @minItems 16
 * @maxItems 16
 */
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
export type DNSRecordAorAAAAJSON = string;
export type DNSRecordSRVJSON = string;
export type RelaysJSON = RelayJSON[];
export type MIRPotJSON = { type: "RESERVES" } | { type: "TREASURY" };
export type MIREnumJSON =
  | { type: "TO_OTHER_POT"; value: string }
  | { type: "TO_STAKE_CREDENTIALS"; value: StakeToCoinJSON[] };
export type DRepJSON =
  | { type: "ALWAYS_ABSTAIN" }
  | { type: "ALWAYS_NO_CONFIDENCE" }
  | { type: "KEY_HASH"; value: string }
  | { type: "SCRIPT_HASH"; value: string };
export type DataOptionJSON =
  | { type: "DATA_HASH"; value: string }
  | { type: "DATA"; value: PlutusDataVariant };
/** ScriptRef is stored as a CBOR hex string */
export type ScriptRefJSON = string;
/** Mint uses the same structure as MultiAsset */
export type MintJSON = MultiAssetJSON;
export type NetworkIdJSON = { type: "TESTNET" } | { type: "MAINNET" };
export type TransactionOutputsJSON = TransactionOutputJSON[];
export type CostModelJSON = string[];
export type VoterJSON =
  | { type: "CONSTITUTIONAL_COMMITTEE_HOT_CRED"; value: CredTypeJSON }
  | { type: "DREP"; value: CredTypeJSON }
  | { type: "STAKING_POOL"; value: string };
export type VoteKindJSON =
  | { type: "NO" }
  | { type: "YES" }
  | { type: "ABSTAIN" };
export type GovernanceActionJSON =
  | { type: "PARAMETER_CHANGE_ACTION"; value: ParameterChangeActionJSON }
  | { type: "HARD_FORK_INITIATION_ACTION"; value: HardForkInitiationActionJSON }
  | {
      type: "TREASURY_WITHDRAWALS_ACTION";
      value: TreasuryWithdrawalsActionJSON;
    }
  | { type: "NO_CONFIDENCE_ACTION"; value: NoConfidenceActionJSON }
  | { type: "UPDATE_COMMITTEE_ACTION"; value: UpdateCommitteeActionJSON }
  | { type: "NEW_CONSTITUTION_ACTION"; value: NewConstitutionActionJSON }
  | { type: "INFO_ACTION" };
/**
 * @minItems 0
 * @maxItems 0
 */
export type InfoActionJSON = [];
export type TransactionBodiesJSON = TransactionBodyJSON[];
export type RedeemerTagJSON =
  | { type: "SPEND" }
  | { type: "MINT" }
  | { type: "CERT" }
  | { type: "REWARD" }
  | { type: "VOTE" }
  | { type: "VOTING_PROPOSAL" };

export interface ProtocolVersionJSON {
  major: number;
  minor: number;
}
export interface TransactionBodyJSON {
  auxiliary_data_hash?: string | null;
  certs?: CertificateJSON[] | null;
  collateral?: TransactionInputJSON[] | null;
  collateral_return?: TransactionOutputJSON | null;
  current_treasury_value?: string | null;
  donation?: string | null;
  fee: string;
  inputs: TransactionInputJSON[];
  mint?: MintJSON | null;
  network_id?: NetworkIdJSON | null;
  outputs: TransactionOutputsJSON;
  reference_inputs?: TransactionInputJSON[] | null;
  required_signers?: string[] | null;
  script_data_hash?: string | null;
  total_collateral?: string | null;
  ttl?: string | null;
  update?: UpdateJSON | null;
  validity_start_interval?: string | null;
  voting_procedures?: VoterVotesJSON[] | null;
  voting_proposals?: VotingProposalJSON[] | null;
  withdrawals?: {
    [k: string]: string;
  } | null;
}
export interface StakeRegistrationJSON {
  coin?: string | null;
  stake_credential: CredTypeJSON;
}
export interface StakeDeregistrationJSON {
  coin?: string | null;
  stake_credential: CredTypeJSON;
}
export interface StakeDelegationJSON {
  pool_keyhash: string;
  stake_credential: CredTypeJSON;
}
export interface PoolRegistrationJSON {
  pool_params: PoolParamsJSON;
}
export interface PoolParamsJSON {
  cost: string;
  margin: UnitIntervalJSON;
  operator: string;
  pledge: string;
  pool_metadata?: PoolMetadataJSON | null;
  pool_owners: string[];
  relays: RelaysJSON;
  reward_account: string;
  vrf_keyhash: string;
}
export interface UnitIntervalJSON {
  denominator: string;
  numerator: string;
}
export interface PoolMetadataJSON {
  pool_metadata_hash: string;
  url: URLJSON;
}
export interface SingleHostAddrJSON {
  ipv4?: Ipv4JSON | null;
  ipv6?: Ipv6JSON | null;
  port?: number | null;
}
export interface SingleHostNameJSON {
  dns_name: DNSRecordAorAAAAJSON;
  port?: number | null;
}
export interface MultiHostNameJSON {
  dns_name: DNSRecordSRVJSON;
}
export interface PoolRetirementJSON {
  epoch: number;
  pool_keyhash: string;
}
export interface GenesisKeyDelegationJSON {
  genesis_delegate_hash: string;
  genesishash: string;
  vrf_keyhash: string;
}
export interface MoveInstantaneousRewardsCertJSON {
  move_instantaneous_reward: MoveInstantaneousRewardJSON;
}
export interface MoveInstantaneousRewardJSON {
  pot: MIRPotJSON;
  variant: MIREnumJSON;
}
export interface StakeToCoinJSON {
  amount: string;
  stake_cred: CredTypeJSON;
}
export interface CommitteeHotAuthJSON {
  committee_cold_credential: CredTypeJSON;
  committee_hot_credential: CredTypeJSON;
}
export interface CommitteeColdResignJSON {
  anchor?: AnchorJSON | null;
  committee_cold_credential: CredTypeJSON;
}
export interface DRepDeregistrationJSON {
  coin: string;
  voting_credential: CredTypeJSON;
}
export interface DRepRegistrationJSON {
  anchor?: AnchorJSON | null;
  coin: string;
  voting_credential: CredTypeJSON;
}
export interface DRepUpdateJSON {
  anchor?: AnchorJSON | null;
  voting_credential: CredTypeJSON;
}
export interface StakeAndVoteDelegationJSON {
  drep: DRepJSON;
  pool_keyhash: string;
  stake_credential: CredTypeJSON;
}
export interface StakeRegistrationAndDelegationJSON {
  coin: string;
  pool_keyhash: string;
  stake_credential: CredTypeJSON;
}
export interface StakeVoteRegistrationAndDelegationJSON {
  coin: string;
  drep: DRepJSON;
  pool_keyhash: string;
  stake_credential: CredTypeJSON;
}
export interface VoteDelegationJSON {
  drep: DRepJSON;
  stake_credential: CredTypeJSON;
}
export interface VoteRegistrationAndDelegationJSON {
  coin: string;
  drep: DRepJSON;
  stake_credential: CredTypeJSON;
}
export interface TransactionInputJSON {
  index: number;
  transaction_id: string;
}
export interface TransactionOutputJSON {
  address: string;
  amount: ValueJSON;
  plutus_data?: DataOptionJSON | null;
  script_ref?: ScriptRefJSON | null;
}
export interface ValueJSON {
  coin: string;
  multiasset?: MultiAssetJSON | null;
}
export interface MultiAssetJSON {
  [k: string]: AssetsJSON;
}
export interface UpdateJSON {
  epoch: number;
  proposed_protocol_parameter_updates: {
    [k: string]: ProtocolParamUpdateJSON;
  };
}
export interface ProtocolParamUpdateJSON {
  ada_per_utxo_byte?: string | null;
  collateral_percentage?: number | null;
  committee_term_limit?: number | null;
  cost_models?: CostmdlsJSON | null;
  d?: UnitIntervalJSON | null;
  drep_deposit?: string | null;
  drep_inactivity_period?: number | null;
  drep_voting_thresholds?: DRepVotingThresholdsJSON | null;
  execution_costs?: ExUnitPricesJSON | null;
  expansion_rate?: UnitIntervalJSON | null;
  extra_entropy?: NonceJSON | null;
  governance_action_deposit?: string | null;
  governance_action_validity_period?: number | null;
  key_deposit?: string | null;
  max_block_body_size?: number | null;
  max_block_ex_units?: ExUnitsJSON | null;
  max_block_header_size?: number | null;
  max_collateral_inputs?: number | null;
  max_epoch?: number | null;
  max_tx_ex_units?: ExUnitsJSON | null;
  max_tx_size?: number | null;
  max_value_size?: number | null;
  min_committee_size?: number | null;
  min_pool_cost?: string | null;
  minfee_a?: string | null;
  minfee_b?: string | null;
  n_opt?: number | null;
  pool_deposit?: string | null;
  pool_pledge_influence?: UnitIntervalJSON | null;
  pool_voting_thresholds?: PoolVotingThresholdsJSON | null;
  protocol_version?: ProtocolVersionJSON | null;
  ref_script_coins_per_byte?: UnitIntervalJSON | null;
  treasury_growth_rate?: UnitIntervalJSON | null;
}
export interface CostmdlsJSON {
  [k: string]: CostModelJSON;
}
export interface DRepVotingThresholdsJSON {
  committee_no_confidence: UnitIntervalJSON;
  committee_normal: UnitIntervalJSON;
  hard_fork_initiation: UnitIntervalJSON;
  motion_no_confidence: UnitIntervalJSON;
  pp_economic_group: UnitIntervalJSON;
  pp_governance_group: UnitIntervalJSON;
  pp_network_group: UnitIntervalJSON;
  pp_technical_group: UnitIntervalJSON;
  treasury_withdrawal: UnitIntervalJSON;
  update_constitution: UnitIntervalJSON;
}
export interface ExUnitPricesJSON {
  mem_price: UnitIntervalJSON;
  step_price: UnitIntervalJSON;
}
export interface NonceJSON {
  /**
   * @minItems 32
   * @maxItems 32
   */
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
export interface ExUnitsJSON {
  mem: string;
  steps: string;
}
export interface PoolVotingThresholdsJSON {
  committee_no_confidence: UnitIntervalJSON;
  committee_normal: UnitIntervalJSON;
  hard_fork_initiation: UnitIntervalJSON;
  motion_no_confidence: UnitIntervalJSON;
  security_relevant_threshold: UnitIntervalJSON;
}
export interface VoterVotesJSON {
  voter: VoterJSON;
  votes: VoteJSON[];
}
export interface VoteJSON {
  action_id: GovernanceActionIdJSON;
  voting_procedure: VotingProcedureJSON;
}
export interface GovernanceActionIdJSON {
  index: number;
  transaction_id: string;
}
export interface VotingProcedureJSON {
  anchor?: AnchorJSON | null;
  vote: VoteKindJSON;
}
export interface VotingProposalJSON {
  anchor: AnchorJSON;
  deposit: string;
  governance_action: GovernanceActionJSON;
  reward_account: string;
}
export interface ParameterChangeActionJSON {
  gov_action_id?: GovernanceActionIdJSON | null;
  policy_hash?: string | null;
  protocol_param_updates: ProtocolParamUpdateJSON;
}
export interface HardForkInitiationActionJSON {
  gov_action_id?: GovernanceActionIdJSON | null;
  protocol_version: ProtocolVersionJSON;
}
export interface TreasuryWithdrawalsActionJSON {
  policy_hash?: string | null;
  withdrawals: TreasuryWithdrawalsJSON;
}
export interface TreasuryWithdrawalsJSON {
  [k: string]: string;
}
export interface NoConfidenceActionJSON {
  gov_action_id?: GovernanceActionIdJSON | null;
}
export interface UpdateCommitteeActionJSON {
  committee: CommitteeJSON;
  gov_action_id?: GovernanceActionIdJSON | null;
  members_to_remove: CredTypeJSON[];
}
export interface CommitteeJSON {
  members: CommitteeMemberJSON[];
  quorum_threshold: UnitIntervalJSON;
}
export interface CommitteeMemberJSON {
  stake_credential: CredTypeJSON;
  term_limit: number;
}
export interface NewConstitutionActionJSON {
  constitution: ConstitutionJSON;
  gov_action_id?: GovernanceActionIdJSON | null;
}
export interface ConstitutionJSON {
  anchor: AnchorJSON;
  script_hash?: string | null;
}
export interface TransactionWitnessSetJSON {
  bootstraps?: BootstrapWitnessJSON[] | null;
  native_scripts?: NativeScriptJSON[] | null;
  plutus_data?: PlutusListJSON | null;
  plutus_scripts?: string[] | null;
  redeemers?: RedeemerJSON[] | null;
  vkeys?: VkeywitnessJSON[] | null;
}
export interface BootstrapWitnessJSON {
  attributes: number[];
  chain_code: number[];
  signature: string;
  vkey: VkeyJSON;
}
export interface PlutusListJSON {
  definite_encoding?: boolean | null;
  elems: string[];
}
export interface RedeemerJSON {
  data: PlutusDataVariant;
  ex_units: ExUnitsJSON;
  index: string;
  tag: RedeemerTagJSON;
}
export interface VkeywitnessJSON {
  signature: string;
  vkey: VkeyJSON;
}
export type BlockHashJSON = string;
export type BootstrapWitnessesJSON = BootstrapWitnessJSON[];

export type CertificateEnumJSON = CertificateJSON;
export type CertificatesJSON = CertificateJSON[];

export type CredentialJSON = CredTypeJSON;
export type CredentialsJSON = CredTypeJSON[];
export type DRepEnumJSON =
  | { type: "ALWAYS_ABSTAIN" }
  | { type: "ALWAYS_NO_CONFIDENCE" }
  | { type: "KEY_HASH"; value: string }
  | { type: "SCRIPT_HASH"; value: string };
export type DataHashJSON = string;
export type Ed25519KeyHashJSON = string;
export type Ed25519KeyHashesJSON = string[];
export type Ed25519SignatureJSON = string;
export interface GeneralTransactionMetadataJSON {
  [k: string]: string;
}
export type GenesisDelegateHashJSON = string;
export type GenesisHashJSON = string;
export type GenesisHashesJSON = string[];
export type GovernanceActionEnumJSON = GovernanceActionJSON;
export type GovernanceActionIdsJSON = GovernanceActionIdJSON[];

export type IntJSON = string;
/**
 * @minItems 4
 * @maxItems 4
 */
export type KESVKeyJSON = string;
export type LanguageJSON = LanguageKindJSON;
export type LanguageKindJSON =
  | { type: "PLUTUS_V1" }
  | { type: "PLUTUS_V2" }
  | { type: "PLUTUS_V3" };
export type LanguagesJSON = LanguageJSON[];
export type MIRToStakeCredentialsJSON = StakeToCoinJSON[];

export type NativeScriptsJSON = NativeScriptJSON[];

export type NetworkIdKindJSON = NetworkIdJSON;
export type PlutusScriptJSON = string;
export type PlutusScriptsJSON = string[];
export type PoolMetadataHashJSON = string;
export interface ProposedProtocolParameterUpdatesJSON {
  [k: string]: ProtocolParamUpdateJSON;
}
export type PublicKeyJSON = string;
export type RedeemerTagKindJSON = RedeemerTagJSON;
export type RedeemersJSON = RedeemerJSON[];

export type RelayEnumJSON = RelayJSON;
/**
 * @minItems 4
 * @maxItems 4
 */
export type RewardAddressJSON = string;
export type RewardAddressesJSON = string[];
export type ScriptDataHashJSON = string;
export type ScriptHashJSON = string;
export type ScriptHashesJSON = string[];
/** ScriptRef is stored as a CBOR hex string */
export type ScriptRefEnumJSON = string;
export interface TransactionJSON {
  auxiliary_data?: AuxiliaryDataJSON | null;
  body: TransactionBodyJSON;
  is_valid: boolean;
  witness_set: TransactionWitnessSetJSON;
}
export type TransactionHashJSON = string;
export type TransactionInputsJSON = TransactionInputJSON[];

export interface TransactionUnspentOutputJSON {
  input: TransactionInputJSON;
  output: TransactionOutputJSON;
}
export type TransactionUnspentOutputsJSON = TransactionUnspentOutputJSON[];

export type VkeywitnessesJSON = VkeywitnessJSON[];

export type VoterEnumJSON = VoterJSON;
export type VotersJSON = VoterJSON[];
export type VotingProceduresJSON = VoterVotesJSON[];

export type VotingProposalsJSON = VotingProposalJSON[];

export interface WithdrawalsJSON {
  [k: string]: string;
}

/**
 * Metadatum (tagged enum with "type" discriminator)
 */
export type Metadatum =
  | { type: "INT"; value: number | bigint }
  | { type: "BYTES"; value: number[] } // raw bytes as array
  | { type: "STRING"; value: string }
  | { type: "LIST"; value: Metadatum[] }
  | { type: "MAP"; value: [Metadatum, Metadatum][] };

/** TxMetadata is a map from label (string) to Metadatum */
export type TxMetadata = { [label: string]: Metadatum };

/**
 * PlutusData (tagged enum with "type" discriminator)
 */
export type PlutusData =
  | { type: "INTEGER"; value: number | bigint }
  | { type: "BYTES"; value: string } // hex string
  | { type: "LIST"; value: PlutusData[] }
  | { type: "MAP"; value: [PlutusData, PlutusData][] }
  | { type: "CONSTR"; alternative: number; fields: PlutusData[] };

export type PlutusDataVariant =
  | {
      type: "CBOR";
      hex: string;
    }
  | {
      type: "MANUAL";
      data: PlutusData;
    };
