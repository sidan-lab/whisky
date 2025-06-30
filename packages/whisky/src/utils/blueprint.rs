use crate::data::*;
use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct MintingBlueprint<P = (), R = ByteString>
where
    P: PlutusDataJson,
    R: PlutusDataJson,
{
    pub version: LanguageVersion,
    pub cbor: String,
    pub hash: String,
    _phantom_param: PhantomData<P>,
    _phantom_redeemer: PhantomData<R>,
}

impl<P, R> MintingBlueprint<P, R>
where
    P: PlutusDataJson,
    R: PlutusDataJson,
{
    pub fn new(version: LanguageVersion) -> Self {
        Self {
            version,
            cbor: "".to_string(),
            hash: "".to_string(),
            _phantom_param: PhantomData,
            _phantom_redeemer: PhantomData,
        }
    }

    pub fn param_script(
        &mut self,
        compiled_code: &str,
        params: &[&str],
        params_type: BuilderDataType,
    ) -> Result<&mut Self, WError> {
        let cbor = apply_params_to_script(compiled_code, params, params_type)?;
        let hash = get_script_hash(&cbor, self.version.clone())?;
        self.hash = hash;
        self.cbor = cbor;
        Ok(self)
    }

    pub fn no_param_script(&mut self, compiled_code: &str) -> Result<&mut Self, WError> {
        let cbor = apply_params_to_script(compiled_code, &[], BuilderDataType::CBOR)?;
        let hash = get_script_hash(&cbor, self.version.clone())?;
        self.hash = hash;
        self.cbor = cbor;
        Ok(self)
    }

    pub fn params(&self, params: &[P]) -> Vec<String> {
        params
            .iter()
            .map(|p| p.to_json_string())
            .collect::<Vec<String>>()
    }

    pub fn redeemer(&self, redeemer: R) -> WData {
        WData::JSON(redeemer.to_json_string())
    }
}

#[derive(Debug, Clone)]
pub struct WithdrawalBlueprint<P = ByteString, R = ByteString>
where
    P: PlutusDataJson,
    R: PlutusDataJson,
{
    pub version: LanguageVersion,
    pub network_id: u8,
    pub cbor: String,
    pub hash: String,
    pub address: String,
    _phantom_param: PhantomData<P>,
    _phantom_redeemer: PhantomData<R>,
}

impl<P, R> WithdrawalBlueprint<P, R>
where
    P: PlutusDataJson,
    R: PlutusDataJson,
{
    pub fn new(version: LanguageVersion, network_id: u8) -> Self {
        Self {
            version,
            network_id,
            cbor: "".to_string(),
            hash: "".to_string(),
            address: "".to_string(),
            _phantom_param: PhantomData,
            _phantom_redeemer: PhantomData,
        }
    }

    pub fn param_script(
        &mut self,
        compiled_code: &str,
        params: &[&str],
        params_type: BuilderDataType,
    ) -> Result<&mut Self, WError> {
        let cbor = apply_params_to_script(compiled_code, params, params_type).unwrap();
        let hash = get_script_hash(&cbor, self.version.clone()).unwrap();
        self.address = script_hash_to_stake_address(&hash, self.network_id)?;
        self.hash = hash;
        self.cbor = cbor;
        Ok(self)
    }

    pub fn no_param_script(&mut self, compiled_code: &str) -> Result<&mut Self, WError> {
        let cbor = apply_params_to_script(compiled_code, &[], BuilderDataType::CBOR)?;
        let hash = get_script_hash(&cbor, self.version.clone())?;
        self.address = script_hash_to_stake_address(&hash, self.network_id)?;
        self.hash = hash;
        self.cbor = cbor;
        Ok(self)
    }

    pub fn params(&self, params: &[P]) -> Vec<String> {
        params
            .iter()
            .map(|p| p.to_json_string())
            .collect::<Vec<String>>()
    }

    pub fn redeemer(&self, redeemer: R) -> WData {
        WData::JSON(redeemer.to_json_string())
    }
}

#[derive(Debug, Clone)]
pub struct SpendingBlueprint<P = ByteString, R = ByteString, D = ByteString>
where
    P: PlutusDataJson,
    R: PlutusDataJson,
    D: PlutusDataJson,
{
    pub version: LanguageVersion,
    pub network_id: u8,
    pub stake_hash: Option<(String, bool)>,
    pub cbor: String,
    pub hash: String,
    pub address: String,
    _phantom_param: PhantomData<[P]>,
    _phantom_redeemer: PhantomData<R>,
    _phantom_datum: PhantomData<D>,
}

impl<P, R, D> SpendingBlueprint<P, R, D>
where
    P: PlutusDataJson,
    R: PlutusDataJson,
    D: PlutusDataJson,
{
    pub fn new(
        version: LanguageVersion,
        network_id: u8,
        stake_hash: Option<(String, bool)>,
    ) -> Self {
        Self {
            version,
            network_id,
            stake_hash,
            cbor: "".to_string(),
            hash: "".to_string(),
            address: "".to_string(),
            _phantom_param: PhantomData,
            _phantom_redeemer: PhantomData,
            _phantom_datum: PhantomData,
        }
    }

    pub fn param_script(
        &mut self,
        compiled_code: &str,
        params: &[&str],
        params_type: BuilderDataType,
    ) -> Result<&mut Self, WError> {
        let cbor = apply_params_to_script(compiled_code, params, params_type)?;
        let hash = get_script_hash(&cbor, self.version.clone())?;
        let stake_hash: Option<(&str, bool)> = self
            .stake_hash
            .as_ref()
            .map(|(hash, is_script)| (hash.as_str(), *is_script));

        let address = script_to_address(self.network_id, &hash, stake_hash);
        self.hash = hash;
        self.cbor = cbor;
        self.address = address;
        Ok(self)
    }

    pub fn no_param_script(&mut self, compiled_code: &str) -> Result<&mut Self, WError> {
        let cbor = apply_params_to_script(compiled_code, &[], BuilderDataType::CBOR)?;
        let hash = get_script_hash(&cbor, self.version.clone())?;
        let stake_hash: Option<(&str, bool)> = self
            .stake_hash
            .as_ref()
            .map(|(hash, is_script)| (hash.as_str(), *is_script));
        let address = script_to_address(self.network_id, &hash, stake_hash);
        self.hash = hash;
        self.cbor = cbor;
        self.address = address;
        Ok(self)
    }

    pub fn params(&self, params: &[P]) -> Vec<String> {
        params
            .iter()
            .map(|p| p.to_json_string())
            .collect::<Vec<String>>()
    }

    pub fn redeemer(&self, redeemer: R) -> WData {
        WData::JSON(redeemer.to_json_string())
    }

    pub fn datum(&self, datum: D) -> WData {
        WData::JSON(datum.to_json_string())
    }
}
