use regex::Regex;
use serde_json::{Number, Value};

pub fn decode(encoded_value: &str) -> Value {
    if encoded_value.starts_with('l') && encoded_value.ends_with('e') {
        let inner_value = &encoded_value[1..encoded_value.len() - 1];
        let mut decoded_list = Vec::new();
        let mut remaining = inner_value;

        while !remaining.is_empty() {
            let (decoded_element, new_remaining) = decode_inner(remaining);
            decoded_list.push(decoded_element);
            remaining = new_remaining;
        }
        Value::Array(decoded_list)
    } else {
        decode_inner(encoded_value).0
    }
}

fn decode_inner(encoded_value: &str) -> (Value, &str) {
    let regex_digit = Regex::new(r"^(\d+):(.+)$").unwrap();
    let regex_integer = Regex::new(r"^i(-?\d+)e").unwrap();
    let regex_list = Regex::new(r"^l(.+)e$").unwrap();

    match encoded_value {
        encoded if regex_digit.is_match(encoded) => {
            let length_end = encoded.find(':').unwrap();
            let length: usize = encoded[..length_end].parse().unwrap();
            let string = &encoded[length_end + 1..length_end + 1 + length];
            (
                Value::String(string.to_string()),
                &encoded[length_end + 1 + length..],
            )
        }
        encoded if regex_integer.is_match(encoded) => {
            let number_end = encoded.find('e').unwrap();
            let number = &encoded[1..number_end];
            let parsed_number = number.parse::<i64>().unwrap();
            (
                Value::Number(Number::from(parsed_number)),
                &encoded[number_end + 1..],
            )
        }
        encoded if regex_list.is_match(encoded) => {
            let captures = regex_list.captures(encoded).unwrap();
            let inner_value = &captures[1];
            let mut decoded_list = Vec::new();
            let mut remaining = inner_value;

            while !remaining.is_empty() {
                let (decoded_element, new_remaining) = decode_inner(remaining);
                decoded_list.push(decoded_element);
                remaining = new_remaining;
            }
            (
                Value::Array(decoded_list),
                &encoded[encoded.len() - remaining.len()..],
            )
        }
        _ => panic!("Unhandled encoded value: {}", encoded_value),
    }
}
