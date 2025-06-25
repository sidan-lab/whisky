#[cfg(test)]
mod tests {
    use serde_json::json;
    use whisky_common::data::*;

    #[test]
    fn test_constr() {
        let correct_constr = "{\"constructor\":10,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(
            constr(10, json!([byte_string("hello")])).to_string(),
            correct_constr
        );

        let new_box = Box::new(PlutusData::ByteString(ByteString::new("hello")));

        let constructor = Constr::new(10, new_box).to_json_string();
        assert_eq!(constructor, correct_constr);
    }

    #[test]
    fn test_constr0() {
        let correct_constr0 = "{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(constr0([byte_string("hello")]).to_string(), correct_constr0);
    }

    #[test]
    fn test_constr1() {
        let correct_constr1 = "{\"constructor\":1,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(constr1([byte_string("hello")]).to_string(), correct_constr1);
    }

    #[test]
    fn test_constr2() {
        let correct_constr2 = "{\"constructor\":2,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(constr2([byte_string("hello")]).to_string(), correct_constr2);
    }
}
