#[macro_use]
extern crate serde_json;

use serde_json::Value as JsonValue;

pub fn filter_map<F>(
    data: serde_json::Map<String, JsonValue>,
    f: &F,
) -> serde_json::Map<String, JsonValue>
where
    F: Fn(&JsonValue) -> bool,
{
    let mut map = serde_json::Map::new();
    for (key, value) in data.into_iter() {
        match value {
            JsonValue::Null => map.insert(key, JsonValue::Null),
            JsonValue::Bool(flag) => map.insert(key, JsonValue::Bool(flag)),
            JsonValue::Number(number) => map.insert(key, JsonValue::Number(number)),
            JsonValue::String(string) => map.insert(key, JsonValue::String(string)),
            JsonValue::Array(arr) => map.insert(key, filter(JsonValue::Array(arr), f)),
            JsonValue::Object(map_value) => {
                map.insert(key, filter(JsonValue::Object(map_value), f))
            }
        };
    }
    map
}

pub fn filter_list<F>(data: Vec<JsonValue>, f: &F) -> Vec<JsonValue>
where
    F: Fn(&JsonValue) -> bool,
{
    let mut array = vec![];
    for value in data.into_iter() {
        match value.clone() {
            JsonValue::Null => {
                if f(&value) {
                    array.push(JsonValue::Null)
                }
            }
            JsonValue::Bool(flag) => {
                if f(&value) {
                    array.push(JsonValue::Bool(flag))
                }
            }
            JsonValue::Number(number) => {
                if f(&value) {
                    array.push(JsonValue::Number(number))
                }
            }
            JsonValue::String(string) => {
                if f(&value) {
                    array.push(JsonValue::String(string))
                }
            }
            JsonValue::Array(arr) => array.push(filter(JsonValue::Array(arr), f)),
            JsonValue::Object(map) => array.push(filter(JsonValue::Object(map), f)),
        };
    }
    array
}

pub fn filter<F>(data: JsonValue, f: &F) -> JsonValue
where
    F: Fn(&JsonValue) -> bool,
{
    match data {
        JsonValue::Null => JsonValue::Null,
        JsonValue::String(string) => JsonValue::String(string),
        JsonValue::Array(array) => JsonValue::Array(filter_list(array, f)),
        JsonValue::Number(number) => JsonValue::Number(number),
        JsonValue::Object(map) => JsonValue::Object(filter_map(map, f)),
        JsonValue::Bool(flag) => JsonValue::Bool(flag),
    }
}

fn main() {
    let j = filter(
        json!([1,null,3,null,{"a": [1,2,3,4, "Abrar" ]}]),
        &|value| match value {
            JsonValue::Number(number) => {
                if number.is_i64() && number.as_i64().unwrap() < 5 {
                    true
                } else {
                    false
                }
            }

            JsonValue::String(string) => {
                if string == "Abrar" {
                    false
                } else {
                    true
                }
            }

            _ => true,
        },
    );
    println!("{:?}", j);
    ()
}
