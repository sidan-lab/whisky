use crate::csl;
use crate::model::{
    Asset, Datum, JsVecString, LanguageVersion, MeshTxBuilderBody, Output, ProvidedScriptSource,
    ValidityRange,
};

pub struct MeshTxParser {
    pub tx_hex: String,
    pub tx_fee_lovelace: u64,
    pub tx_body: MeshTxBuilderBody,
    pub csl_tx_body: csl::TransactionBody,
}

pub trait MeshTxParserTrait {
    fn new(s: &str) -> Self;
    // TODO: add testing method lists here
    fn get_tx_outs_cbor(&self) -> Vec<String>;
}

impl MeshTxParserTrait for MeshTxParser {
    // Constructor method
    fn new(s: &str) -> MeshTxParser {
        // TODO: Deserialized into the tx_body
        let mut tx_body = MeshTxBuilderBody {
            inputs: vec![],
            outputs: vec![],
            collaterals: vec![],
            required_signatures: JsVecString::new(),
            reference_inputs: vec![],
            mints: vec![],
            change_address: "".to_string(),
            change_datum: None,
            metadata: vec![],
            validity_range: ValidityRange {
                invalid_before: None,
                invalid_hereafter: None,
            },
            signing_key: JsVecString::new(),
        };
        let csl_tx = csl::Transaction::from_hex(s).expect("Invalid transaction");
        let csl_tx_body = csl_tx.body();
        for i in 0..csl_tx_body.outputs().len() {
            tx_body
                .outputs
                .push(csl_output_to_mesh_output(csl_tx_body.outputs().get(i)))
        }
        MeshTxParser {
            tx_hex: s.to_string(),
            tx_fee_lovelace: csl::utils::from_bignum(&csl_tx.body().fee()),
            tx_body,
            csl_tx_body,
        }
    }

    fn get_tx_outs_cbor(&self) -> Vec<String> {
        let tx_outs = self.csl_tx_body.outputs();
        let mut result = vec![];
        for i in 0..tx_outs.len() {
            let tx_out: csl::TransactionOutput = tx_outs.get(i);
            let tx_out_cbor = tx_out.to_hex();
            result.push(tx_out_cbor);
        }
        result
    }
}

fn csl_output_to_mesh_output(output: csl::TransactionOutput) -> Output {
    let mut value: Vec<Asset> = vec![];
    value.push(Asset {
        unit: "lovelace".to_string(),
        quantity: output.amount().coin().to_str(),
    });
    let multi_asset = output.amount().multiasset();

    match multi_asset {
        None => {}
        Some(multi_asset) => {
            for policy_id_index in 0..multi_asset.keys().len() {
                let policy_id = multi_asset.keys().get(policy_id_index);
                let assets = multi_asset.get(&policy_id).unwrap();
                for asset_index in 0..assets.keys().len() {
                    let asset_name = assets.keys().get(asset_index);
                    let asset_quantity = assets.get(&asset_name).unwrap();
                    let concated_name = policy_id.to_hex() + &asset_name.to_string();

                    value.push(Asset {
                        unit: concated_name,
                        quantity: asset_quantity.to_str(),
                    })
                }
            }
        }
    }

    // TODO: Handle datum hash case
    let datum: Option<Datum> = output.plutus_data().map(|csl_datum| Datum {
        type_: "Inline".to_string(),
        data: csl_datum
            .to_json(csl::plutus::PlutusDatumSchema::DetailedSchema)
            .unwrap(),
    });

    let reference_script: Option<ProvidedScriptSource> = match output.script_ref() {
        Some(csl_script_ref) => {
            let plutus_script = csl_script_ref.plutus_script().unwrap();
            let language_version = match plutus_script.language_version().kind() {
                csl::plutus::LanguageKind::PlutusV1 => LanguageVersion::V1,
                csl::plutus::LanguageKind::PlutusV2 => LanguageVersion::V2,
            };
            Some(ProvidedScriptSource {
                script_cbor: plutus_script.to_hex(),
                language_version,
            })
        }
        None => None,
    };
    Output {
        address: output.address().to_bech32(None).unwrap(),
        amount: value,
        datum,
        reference_script,
    }
}

#[test]
fn test_getting_output_cbor() {
    let tx_hex = "84a800848258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a0882582040e1afc8b735a9daf665926554b0e11902e3ed7e4a31a23b917483d4de42c05e04825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6402825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c64030182a3005839104477981671d60af19c524824cacc0a9822ba2a7f32586e57c18156215ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0016e360a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a0243d580028201d81843d87980a300583910634a34d9c1ec5dd0cae61e4c86a4e85214bafdc80c57214fc80745b55ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0075b8d4a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a1298be00028201d81858b1d8799fd8799fd87a9f581c57f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b3ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd8799fd87a9f581c4477981671d60af19c524824cacc0a9822ba2a7f32586e57c1815621ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd87a801a000985801a1dcd6500ff021a0004592c09a00b5820a68f004e69dfc4ed4ff789ceb9be63e9f2412e8d3d7fa0b0cb19e509c927a03c0d818258203fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814070e82581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa1286825820ac7744adce4f25027f1ca009f5cab1d0858753e62c6081a3a3676cfd5333bb03008258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a0882582040e1afc8b735a9daf665926554b0e11902e3ed7e4a31a23b917483d4de42c05e04825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6402825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6403825820efe6fbbdd6b993d96883b96c572bfcaa0a4a138c83bd948dec1751d1bfda09b300a30082825820aa8ce9e908f525c3b700a65669430ec68ca19615e7309e25bb6fa883964cfa9f5840a023ea4e2a266fca669cfdffe3718718c2b2c6e3fbc90da58785079583d94be98f20d2b87327edb940984a739c1fdb25e20e6b04374db299b4de66369208de038258207f4747ca0c20a1e5c28716c4a10fffbcbe8fe6253cb427ae2f0e24d231a9808458402aa02a8a0f2129d727e44cd21f4699b1b1deb43c974ebc6f484b3809e0b5a417e864c43c9be5327fba31fa8146c744c487b00748cb63daf3dc60114850321d0d03800584840000d87980821a000382f61a04d45a03840001d87980821a000382f61a04d45a03840002d87980821a000382f61a04d45a03840003d87980821a000382f61a04d45a03f5f6";
    // let tx_hex = "84a800848258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a098258205150964d0bc32df047f1eb99c35f14e351f21b1303795ffe2b58ebf7de58f67b0082582085aa98980be06b0f5d926bee007301ba7a96d448dfa9dced091fb73b0bcd07bb03825820879f68fef00fa676abcfba0396916299eddbf29e1103442aee031b383ee0f3ad060182a3005839104477981671d60af19c524824cacc0a9822ba2a7f32586e57c18156215ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0016e360a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a0243d580028201d81843d87980a300583910634a34d9c1ec5dd0cae61e4c86a4e85214bafdc80c57214fc80745b55ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a00756f63a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a1298be00028201d81858b1d8799fd8799fd87a9f581c57f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b3ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd8799fd87a9f581c4477981671d60af19c524824cacc0a9822ba2a7f32586e57c1815621ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd87a801a000985801a1dcd6500ff021a0004a29d09a00b58205eb15f7d48931475604b5491a294f5d914ecf03c41a520d80087e2938910d9e70d818258203fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814070e82581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa128682582085aa98980be06b0f5d926bee007301ba7a96d448dfa9dced091fb73b0bcd07bb038258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a09825820efe6fbbdd6b993d96883b96c572bfcaa0a4a138c83bd948dec1751d1bfda09b3008258205150964d0bc32df047f1eb99c35f14e351f21b1303795ffe2b58ebf7de58f67b00825820879f68fef00fa676abcfba0396916299eddbf29e1103442aee031b383ee0f3ad06825820ac7744adce4f25027f1ca009f5cab1d0858753e62c6081a3a3676cfd5333bb0300a30082825820aa8ce9e908f525c3b700a65669430ec68ca19615e7309e25bb6fa883964cfa9f5840904f798c3cbda08e499945f9e00e6b1a968166de063ad3ecb16139a0c5dc10541cc7a33304c60ed7fb350938d2b11fcacb56baf84330473b8544b669640229028258207f4747ca0c20a1e5c28716c4a10fffbcbe8fe6253cb427ae2f0e24d231a98084584016b15d782922177e29e1eae8f7f173db80508692292b6ff3e63c7d33ed1cc231bac0acbb963503e75b96b7c541189508e050fb64034ea4d47a13115f7483ce0d03800584840000d87980821a00045e1e1a0609fd16840001d87980821a00045e1e1a0609fd16840002d87980821a00045e1e1a0609fd16840003d87980821a00045e1e1a0609fd16f5f6";
    let tx_parser = MeshTxParser::new(tx_hex);
    let tx_outs_cbor = tx_parser.get_tx_outs_cbor();
    println!("{:?}", tx_outs_cbor);
}
