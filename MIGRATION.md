# Migration from whisky-csl to whisky-pallas

Currently, whisky uses `cardano-serialization-lib` (CSL) to handle all serialization to transaction cbor. However, since the `pallas` crate can also do the same types of serialization, it can be used for identical purposes.

In general both serialization libraries have quite similar functionality, but `pallas` tends to be better maintained to keep up with changes during hard forks, so a decision was made to migrate everything to use `pallas` for underlying serialization purposes.

## Transaction building

In order to migrate to using the `WhiskyPallas` serializer, you can import it like so, and use it within the transaction builder class.

```rust
use whisky::*;
use whisky_pallas::WhiskyPallas;

// Using WhiskyPallas serializer
let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyPallas::new(None)),
    evaluator: None,
    fetcher: None,
    submitter: None,
    params: None,
});
```

Then the transaction builder can be used as normal

```rust
fn build_simple_spend() -> String {
    let mut tx_builder = TxBuilder::new(TxBuilderParam {
        serializer: Box::new(WhiskyPallas::new(None)),
        evaluator: None,
        fetcher: None,
        submitter: None,
        params: None,
    });

    let signed_tx = tx_builder
        .tx_in(
            "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
            3,
            &[Asset::new_from_str("lovelace", "9891607895")],
            "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
        )
        .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
        .signing_key("your_signing_key_hex")
        .complete_sync(None)
        .unwrap()
        .complete_signing()
        .unwrap();

    signed_tx
}
```

## Transaction parsing

```rust
use whisky_pallas::{
    tx_parser::{check_tx_required_signers, parse},
    wrapper::transaction_body::{DRep, RewardAccount},
};

let tx_hex = "84a600d901028182582066e9f787106bf68431827fc3cde3db92705e9ca984d404516a2c8014b30c8142000181825839005867c3b8e27840f556ac268b781578b14c5661fc63ee720dbeab663f9d4dcd7e454d2434164f4efb8edeb358d86a1dad9ec6224cfcbce3e61a05e9c38b021a000c1d7505a1581df033d5840ab19fcfcff60c2ff509d5371124ee1c2670abd96db9e79064000b582075e3ddd00fd933d11169fbfea99e3c57c362d35a3298ba7de73891ea5048d8ae0dd901028182582066e9f787106bf68431827fc3cde3db92705e9ca984d404516a2c8014b30c814200a207d9010281583658340101002332259800a518a4d153300249011856616c696461746f722072657475726e65642066616c736500136564004ae715cd0105a18203008240821a006acfc01ab2d05e00f5f6";
let result = parse(tx_hex, &utxos).unwrap();
```

## Transaction evaluation

```rust
use uplc::tx::script_context::SlotConfig;
use whisky_common::{Asset, EvalResult, Network, UTxO, UtxoInput, UtxoOutput};
use whisky_pallas::utils::evaluate_tx_scripts;

let result = evaluate_tx_scripts(
        "84a80082825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad9800825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98010d81825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad9801128182582004b9070a30bd63abaaf59a3c48a1575c4127bb0edb00ecd5141fd18a85c721aa000181a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b00000002529898c810a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b0000000252882db4111a000412f1021a0002b74b0b5820775d0cf3c95993f6210e4410e92f72ebc3942ce9c1433694749aa239e5d13387a200818258201557f444f3ae6e61dfed593ae15ec8dbd57b8138972bf16fde5b4c559f41549b5840729f1f14ef05b7cf9b0d7583e6777674f80ae64a35bbd6820cc3c82ddf0412ca1d751b7d886eece3c6e219e1c5cc9ef3d387a8d2078f47125d54b474fbdfbd0105818400000182190b111a000b5e35f5f6",
        &vec![UTxO {
            input: UtxoInput {
                tx_hash: "604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98".to_string(),
                output_index: 0
            },
            output: UtxoOutput {
                address: "addr_test1wzlwsgq97vchypqzk8u8lz30w932tvx7akcj7csm02scl7qlghd97".to_string(),
                amount: vec![Asset::new_from_str("lovelace", "986990")],
                data_hash: None,
                plutus_data: Some("d87980".to_string()),
                script_hash: None,
                script_ref: None,
            }
    },
                UTxO {
                    input: UtxoInput {
                        tx_hash: "604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98".to_string(),
                        output_index: 1
                    },
                    output: UtxoOutput {
                        address: "addr_test1vq0atw43vuecjuwe9dxc7z7l2lvgnyp7d6f5ul4r3376mug8v67h5".to_string(),
                        amount: vec![Asset::new_from_str("lovelace", "9974857893")],
                        data_hash: None,
                        plutus_data: None,
                        script_hash: None,
                        script_ref: None,
                    }
                },
                UTxO {
                    input: UtxoInput {
                        tx_hash: "04b9070a30bd63abaaf59a3c48a1575c4127bb0edb00ecd5141fd18a85c721aa".to_string(),
                        output_index: 0
                    },
                    output: UtxoOutput {
                        address: "addr_test1wzlwsgq97vchypqzk8u8lz30w932tvx7akcj7csm02scl7qlghd97".to_string(),
                        amount: vec![Asset::new_from_str("lovelace", "986990")],
                        data_hash: None,
                        plutus_data: None,
                        script_hash: None,
                        script_ref: Some("82025655010000322223253330054a229309b2b1bad0025735".to_string())
                    }
                }],
        &[],
        &Network::Mainnet,
        &SlotConfig::default()
    );
```
