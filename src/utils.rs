use regex::Regex;
use serde_json::{Number, Value};

pub fn decode(encoded_value: &str) -> Value {
    let regex_digit = Regex::new(r"^(\d+):(.+)$").unwrap();
    let regex_integer = Regex::new(r"^i(-?\d+)e$").unwrap();

    match encoded_value {
        encoded if regex_digit.is_match(encoded) => {
            let captures = regex_digit.captures(encoded).unwrap();
            let length: usize = captures[1].parse().unwrap();
            let string = &captures[2][..length];
            Value::String(string.to_string())
        }

        encoded if regex_integer.is_match(encoded) => {
            let captures = regex_integer.captures(encoded).unwrap();
            let number = captures[1].parse::<i64>().unwrap();

            Value::Number(Number::from(number))
        }
        _ => panic!("Unhandled encoded value: {}", encoded_value),
    }
}
