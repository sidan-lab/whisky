# Staking & Governance

This chapter covers stake operations and Conway-era governance actions.

## Stake Registration

Register a stake key on-chain:

```rust,ignore
use whisky::*;

let mut tx_builder = TxBuilder::new_core();
tx_builder
    .register_stake_certificate(stake_key_hash)
    .change_address(my_address)
    .select_utxos_from(inputs, 5000000)
    .complete_sync(None)?;
```

## Stake Delegation

Delegate to a stake pool (can be combined with registration):

```rust,ignore
tx_builder
    .register_stake_certificate(stake_key_hash)
    .delegate_stake_certificate(stake_key_hash, pool_id)
    .change_address(my_address)
    .select_utxos_from(inputs, 5000000)
    .complete_sync(None)?;
```

## Stake Deregistration

Deregister a stake key to reclaim the deposit:

```rust,ignore
tx_builder
    .deregister_stake_certificate(stake_key_hash)
    .change_address(my_address)
    .select_utxos_from(inputs, 5000000)
    .complete_sync(None)?;
```

## Withdrawals

Withdraw staking rewards:

```rust,ignore
tx_builder
    .tx_in(tx_hash, tx_index, amount, address)
    .change_address(my_address)
    .withdrawal(stake_address, 0)  // stake_address e.g., "stake_test1ur..."
    .required_signer_hash(pub_key_hash)
    .signing_key(signing_key_hex)
    .complete_sync(None)?
    .complete_signing()?;
```

## Governance: DRep Registration

Register as a Delegated Representative (Conway era):

```rust,ignore
tx_builder
    .drep_registration(drep_id, deposit_amount)
    .change_address(my_address)
    .select_utxos_from(inputs, 5000000)
    .complete_sync(None)?;
```

## Governance: Vote Delegation

Delegate your voting power to a DRep:

```rust,ignore
tx_builder
    .vote_delegation(stake_key_hash, drep)
    .change_address(my_address)
    .select_utxos_from(inputs, 5000000)
    .complete_sync(None)?;
```

## Governance: Voting

Cast a governance vote:

```rust,ignore
tx_builder
    .vote(voter, ref_tx_hash, ref_tx_index, vote_kind)
    .change_address(my_address)
    .select_utxos_from(inputs, 5000000)
    .complete_sync(None)?;
```

For script-based voting, use the Plutus vote pattern:

```rust,ignore
tx_builder
    .vote_plutus_script_v3()
    .vote(voter, ref_tx_hash, ref_tx_index, vote_kind)
    .vote_script(&script_cbor)
    .vote_redeemer_value(&redeemer)
    .tx_in_collateral(/* ... */)
    .complete(None)
    .await?;
```
