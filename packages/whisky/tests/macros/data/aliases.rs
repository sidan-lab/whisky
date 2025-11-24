use whisky::data::aliases::OutputReference;
use whisky::data::{output_reference, PlutusDataJson};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_reference() {
        let correct_output_reference =
            "{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"},{\"int\":12}]}";
        assert_eq!(
            output_reference("hello", 12).to_string(),
            correct_output_reference
        );
        assert_eq!(
            OutputReference::from("hello", 12).to_json_string(),
            correct_output_reference
        );
    }
}
