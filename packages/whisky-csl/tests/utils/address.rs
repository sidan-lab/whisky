#[cfg(test)]
mod tests {
    use whisky_common::DeserializedAddress;
    use whisky_csl::{deserialize_address, serialize_address_obj};

    #[test]
    fn test_serialize_address_obj() {
        let base_addr = "addr_test1qpnjm2a4vt0efgkrvw50k96jyk3n8wejyh47qfemnda98886r5k6nf7zgqxm6uxr3f2j8823mh58yln5t65hlsn9kzdqvutf79";
        let expected_enterprize_addr =
            "addr_test1vpnjm2a4vt0efgkrvw50k96jyk3n8wejyh47qfemnda988qm8as9a";

        let mut deser_addr = deserialize_address(base_addr);
        deser_addr.stake_key_hash = "".to_string();
        deser_addr.stake_key_script_hash = "".to_string();
        let enterprize_addr = serialize_address_obj(deser_addr, 0).unwrap();
        assert_eq!(enterprize_addr, expected_enterprize_addr);
    }
}
