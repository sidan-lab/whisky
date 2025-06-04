use cardano_serialization_lib as csl;
use whisky_common::{Metadata, WError};

use super::CSLParser;

impl CSLParser {
    pub fn get_metadata(&self) -> &Vec<Metadata> {
        &self.tx_body.metadata
    }

    pub(super) fn extract_metadata(&mut self) -> Result<(), WError> {
        if let Some(aux_data) = &self.csl_aux_data {
            self.tx_body.metadata = csl_aux_data_to_metadata(&aux_data)?;
        }
        Ok(())
    }
}

fn csl_aux_data_to_metadata(aux_data: &csl::AuxiliaryData) -> Result<Vec<Metadata>, WError> {
    let metadata = aux_data.metadata();
    let csl_metadata_list = metadata.unwrap_or(csl::GeneralTransactionMetadata::new());
    let keys = csl_metadata_list.keys();
    let metadata_list_len = keys.len();
    let mut metadata_list = Vec::new();
    for i in 0..metadata_list_len {
        let key = keys.get(i);
        let metadata = csl_metadata_list
            .get(&key)
            .ok_or_else(|| WError::new("csl_aux_data_to_metadata", "Failed to get metadata"))?;
        let metadata = Metadata {
            tag: key.to_string(),
            metadata: csl::decode_metadatum_to_json_str(
                &metadata,
                csl::MetadataJsonSchema::NoConversions,
            )
            .map_err(|e| {
                WError::new(
                    "csl_aux_data_to_metadata",
                    &format!("Failed to decode metadata: {}", e),
                )
            })?,
        };
        metadata_list.push(metadata);
    }
    Ok(metadata_list)
}
