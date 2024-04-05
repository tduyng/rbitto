use serde_json;

// Example: "5:hello" -> "hello"
pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if encoded_value.chars().next().unwrap().is_ascii_digit() {
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];

        serde_json::Value::String(string.to_string())
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}
