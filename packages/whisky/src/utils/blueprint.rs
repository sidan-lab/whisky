use crate::*;
use serde::Serialize;
use serde_json::json;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct MintingBlueprint<T>
where
    T: Serialize + 'static,
{
    pub version: LanguageVersion,
    pub cbor: String,
    pub hash: String,
    _phantom: PhantomData<T>,
}

impl<T> MintingBlueprint<T>
where
    T: Serialize + 'static,
{
    pub fn new(version: LanguageVersion) -> Self {
        Self {
            version,
            cbor: "".to_string(),
            hash: "".to_string(),
            _phantom: PhantomData,
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

    pub fn redeemer(redeemer: T) -> String {
        json!(redeemer).to_string()
    }
}

#[derive(Debug, Clone)]
pub struct WithdrawalBlueprint<T>
where
    T: Serialize + 'static,
{
    pub version: LanguageVersion,
    pub network_id: u8,
    pub cbor: String,
    pub hash: String,
    pub address: String,
    _phantom: PhantomData<T>,
}

impl<T> WithdrawalBlueprint<T>
where
    T: Serialize + 'static,
{
    pub fn new(version: LanguageVersion, network_id: u8) -> Self {
        Self {
            version,
            network_id,
            cbor: "".to_string(),
            hash: "".to_string(),
            address: "".to_string(),
            _phantom: PhantomData,
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

    pub fn redeemer(redeemer: T) -> String {
        json!(redeemer).to_string()
    }
}

#[derive(Debug, Clone)]
pub struct SpendingBlueprint<P, R, D>
where
    P: Serialize + 'static,
    R: Serialize + 'static,
    D: Serialize + 'static,
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
    P: Serialize + 'static,
    R: Serialize + 'static,
    D: Serialize + 'static,
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

    pub fn params(params: &[P]) -> String {
        json!(params).to_string()
    }

    pub fn redeemer(redeemer: R) -> String {
        json!(redeemer).to_string()
    }

    pub fn datum(datum: D) -> String {
        json!(datum).to_string()
    }
}
