use anyhow::Result;
use serde_json::{Number, Value};

pub fn decode(encoded_value: &str) -> Result<Value> {
    let value: serde_bencode::value::Value = serde_bencode::from_str(encoded_value)?;
    decode_inner(value)
}

fn decode_inner(encoded_value: serde_bencode::value::Value) -> Result<Value> {
    match encoded_value {
        serde_bencode::value::Value::Bytes(b) => Ok(Value::String(String::from_utf8(b)?)),
        serde_bencode::value::Value::Int(i) => Ok(Value::Number(Number::from(i))),
        serde_bencode::value::Value::List(l) => {
            let array = l
                .into_iter()
                .map(decode_inner)
                .collect::<anyhow::Result<Vec<serde_json::Value>>>()?;
            Ok(Value::Array(array))
        }
        serde_bencode::value::Value::Dict(d) => {
            let object = d
                .into_iter()
                .map(|(k, v)| {
                    let key = String::from_utf8(k)?;
                    let value = decode_inner(v)?;
                    Ok((key, value))
                })
                .collect::<Result<serde_json::Map<String, serde_json::Value>>>()?;

            Ok(Value::Object(object))
        }
    }
}
