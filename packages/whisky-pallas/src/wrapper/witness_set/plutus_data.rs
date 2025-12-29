use pallas::{
    interop::utxorpc::spec::cardano::plutus_data,
    ledger::primitives::{conway::PlutusData as PallasPlutusData, Fragment},
};

#[derive(Debug, Clone)]
pub struct PlutusData {
    pub inner: PallasPlutusData,
}

impl PlutusData {
    pub fn new(plutus_data_hex: String) -> Result<Self, String> {
        let bytes = hex::decode(plutus_data_hex).map_err(|e| format!("Hex decode error: {}", e))?;
        let inner = PallasPlutusData::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;
        Ok(Self { inner })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at PlutusData"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasPlutusData::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
