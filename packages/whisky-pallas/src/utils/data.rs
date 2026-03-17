use pallas_primitives::{BigInt, BoundedBytes, MaybeIndefArray, PlutusData};

use serde_json::Value;
use uplc::{Constr, KeyValuePairs};
use whisky_common::WError;

pub fn encode_json_str_to_plutus_datum(json: &str) -> Result<PlutusData, WError> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(WError::from_err(
        "encode_json_str_to_plutus_datum - from_str",
    ))?;

    encode_json_value_to_plutus_datum(value)
}

pub fn encode_json_value_to_plutus_datum(value: Value) -> Result<PlutusData, WError> {
    fn encode_number(x: serde_json::Number) -> Result<PlutusData, WError> {
        if let Some(x) = x.as_u64() {
            Ok(PlutusData::BigInt(BigInt::Int((x as i64).into())))
        } else if let Some(x) = x.as_i64() {
            Ok(PlutusData::BigInt(BigInt::Int(x.into())))
        } else {
            Err(WError::new(
                "encode_number - ",
                "floats not allowed in plutus datums",
            ))
        }
    }

    fn encode_string(s: &str, is_key: bool) -> Result<PlutusData, WError> {
        if s.starts_with("0x") {
            hex::decode(&s[2..])
                .map(|bytes| PlutusData::BoundedBytes(BoundedBytes::from(bytes)))
                .map_err(WError::from_err("encode_string - hex decode"))
        } else if is_key {
            // try first as integer
            if let Ok(x) = s.parse::<u64>() {
                Ok(PlutusData::BigInt(BigInt::Int((x as i64).into())))
            } else if let Ok(x) = s.parse::<i64>() {
                Ok(PlutusData::BigInt(BigInt::Int(x.into())))
            } else {
                // if not integer, encode as bytes
                Ok(PlutusData::BoundedBytes(BoundedBytes::from(
                    s.as_bytes().to_vec(),
                )))
            }
        } else {
            Ok(PlutusData::BoundedBytes(BoundedBytes::from(
                s.as_bytes().to_vec(),
            )))
        }
    }

    fn encode_array(json_arr: Vec<Value>) -> Result<PlutusData, WError> {
        let mut arr: Vec<PlutusData> = Vec::new();
        for item in json_arr {
            arr.push(encode_json_value_to_plutus_datum(item)?);
        }
        Ok(PlutusData::Array(MaybeIndefArray::Def(arr)))
    }

    match value {
        Value::Object(obj) => {
            if obj.len() == 1 {
                let (k, v) = obj.into_iter().next().unwrap();
                match k.as_str() {
                    "int" => match v {
                        Value::Number(x) => encode_number(x),
                        _ => Err(WError::new(
                            "encode_json_value_to_plutus_datum - int",
                            "expected number for int type",
                        )),
                    },
                    "bytes" => match v {
                        Value::String(s) => encode_string(&s, false),
                        _ => Err(WError::new(
                            "encode_json_value_to_plutus_datum - bytes",
                            "expected string for bytes type",
                        )),
                    },
                    "list" => match v {
                        Value::Array(arr) => encode_array(arr),
                        _ => Err(WError::new(
                            "encode_json_value_to_plutus_datum - list",
                            "expected array for list type",
                        )),
                    },
                    "map" => match v {
                        Value::Array(map_vec) => {
                            let mut map: Vec<(PlutusData, PlutusData)> = vec![];
                            for entry in map_vec {
                                match entry {
                                    Value::Object(entry_obj) => {
                                        let raw_key = entry_obj.get("k").ok_or_else(|| {
                                            WError::new(
                                                "encode_json_value_to_plutus_datum - map entry",
                                                "missing key in map entry",
                                            )
                                        })?;
                                        let raw_value = entry_obj.get("v").ok_or_else(|| {
                                            WError::new(
                                                "encode_json_value_to_plutus_datum - map entry",
                                                "missing value in map entry",
                                            )
                                        })?;
                                        let encoded_key =
                                            encode_json_value_to_plutus_datum(raw_key.clone())?;
                                        let encoded_value =
                                            encode_json_value_to_plutus_datum(raw_value.clone())?;
                                        map.push((encoded_key, encoded_value));
                                    }
                                    _ => {
                                        return Err(WError::new(
                                            "encode_json_value_to_plutus_datum - map entry",
                                            "expected object for map entry",
                                        ))
                                    }
                                }
                            }
                            Ok(PlutusData::Map(KeyValuePairs::from(map)))
                        }
                        _ => Err(WError::new(
                            "encode_json_value_to_plutus_datum - map",
                            "expected array for map type",
                        )),
                    },
                    _ => Err(WError::new(
                        "encode_json_value_to_plutus_datum",
                        "unknown type key",
                    )),
                }
            } else {
                if obj.len() != 2 {
                    return Err(WError::new(
                        "encode_json_value_to_plutus_datum - constr",
                        "expected object with single key for constr type",
                    ));
                }
                let variant: u64 = obj
                    .get("constructor")
                    .ok_or_else(|| {
                        WError::new(
                            "encode_json_value_to_plutus_datum - constr",
                            "missing constr key for constr type",
                        )
                    })?
                    .as_u64()
                    .ok_or_else(|| {
                        WError::new(
                            "encode_json_value_to_plutus_datum - constr",
                            "expected unsigned integer for constr variant",
                        )
                    })?;
                let fields_json = obj
                    .get("fields")
                    .ok_or_else(|| {
                        WError::new(
                            "encode_json_value_to_plutus_datum - constr",
                            "missing fields key for constr type",
                        )
                    })?
                    .as_array()
                    .ok_or_else(|| {
                        WError::new(
                            "encode_json_value_to_plutus_datum - constr",
                            "expected array for constr fields",
                        )
                    })?;
                let mut fields: Vec<PlutusData> = Vec::new();
                for field_json in fields_json {
                    fields.push(encode_json_value_to_plutus_datum(field_json.clone())?);
                }
                return Ok(PlutusData::Constr(Constr {
                    tag: variant + 121,
                    any_constructor: None,
                    fields: MaybeIndefArray::Def(fields),
                }));
            }
        }
        _ => {
            return Err(WError::new(
                "encode_json_value_to_plutus_datum",
                "expected object with single key for typed value",
            ))
        }
    }
}
