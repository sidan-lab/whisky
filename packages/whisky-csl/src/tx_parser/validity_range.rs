use whisky_common::{ValidityRange, WError};

use super::CSLParser;

impl CSLParser {
    pub fn get_validity_range(&self) -> &ValidityRange {
        &self.tx_body.validity_range
    }

    pub(super) fn extract_validity_range(&mut self) -> Result<(), WError> {
        let validity_interval_start_bignum = self.csl_tx_body.validity_start_interval_bignum();
        let validity_interval_end_bignum = self.csl_tx_body.ttl_bignum();
        let validity_interval_start = validity_interval_start_bignum
            .map(|b| b.to_str().parse::<u64>().ok())
            .flatten();
        let validity_interval_end = validity_interval_end_bignum
            .map(|b| b.to_str().parse::<u64>().ok())
            .flatten();
        self.tx_body.validity_range.invalid_before = validity_interval_start;
        self.tx_body.validity_range.invalid_hereafter = validity_interval_end;
        Ok(())
    }
}
