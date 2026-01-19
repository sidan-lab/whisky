use cardano_serialization_lib as csl;
use whisky_common::data::PlutusDataJson;
use whisky_common::WError;

/// Extension trait that adds CBOR serialization/deserialization capabilities
/// to any type implementing `PlutusDataJson`.
///
/// This trait is implemented automatically for all types that implement `PlutusDataJson`,
/// allowing direct conversion to/from CBOR hex strings.
///
/// # Example
///
/// ```rust,ignore
/// use whisky_csl::PlutusDataCbor;
/// use whisky::data::PlutusDataJson;
///
/// #[derive(Debug, Clone, ConstrEnum)]
/// pub enum HydraOrderBookIntent {
///     PlaceOrderIntent(Box<(Order, MValue)>),
///     ModifyOrderIntent(Box<(Order, MValue)>),
/// }
///
/// // Deserialize from CBOR
/// let cbor_hex = "d87a9f...";
/// let intent = HydraOrderBookIntent::from_cbor(cbor_hex)?;
///
/// // Serialize back to CBOR
/// let cbor_out = intent.to_cbor()?;
/// ```
pub trait PlutusDataCbor: PlutusDataJson {
    /// Parse from CBOR hex string.
    ///
    /// This method converts the CBOR hex to JSON using CSL's DetailedSchema,
    /// then parses it using the type's `from_json` implementation.
    fn from_cbor(cbor_hex: &str) -> Result<Self, WError>;

    /// Serialize to CBOR hex string.
    ///
    /// This method converts the type to JSON using `to_json`,
    /// then serializes it to CBOR hex using CSL's DetailedSchema.
    fn to_cbor(&self) -> Result<String, WError>;
}

impl<T: PlutusDataJson> PlutusDataCbor for T {
    fn from_cbor(cbor_hex: &str) -> Result<Self, WError> {
        let csl_data = csl::PlutusData::from_hex(cbor_hex)
            .map_err(WError::from_err("PlutusDataCbor::from_cbor - invalid CBOR hex"))?;

        let json_str = csl_data
            .to_json(csl::PlutusDatumSchema::DetailedSchema)
            .map_err(WError::from_err(
                "PlutusDataCbor::from_cbor - failed to convert to JSON",
            ))?;

        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(WError::from_err("PlutusDataCbor::from_cbor - invalid JSON"))?;

        Self::from_json(&json_value)
    }

    fn to_cbor(&self) -> Result<String, WError> {
        let json_str = self.to_json_string();

        let csl_data = csl::PlutusData::from_json(&json_str, csl::PlutusDatumSchema::DetailedSchema)
            .map_err(WError::from_err("PlutusDataCbor::to_cbor - failed to parse JSON"))?;

        Ok(csl_data.to_hex())
    }
}
