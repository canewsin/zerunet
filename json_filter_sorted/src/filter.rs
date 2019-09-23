use serde_json;
use serde_json::Value as JsonValue;

pub fn filter_list_for_map_keys<F>(data: Vec<JsonValue>, f: &F) -> Vec<JsonValue>
where
    F: Fn(&str) -> bool,
{
    let mut vector = Vec::new();
    for value in data.into_iter() {
        vector.push(filter_map_with_keys(value, f));
    }
    vector
}

fn filter_map_keys<F>(
    data: serde_json::Map<String, JsonValue>,
    f: &F,
) -> serde_json::Map<String, JsonValue>
where
    F: Fn(&str) -> bool,
{
    let mut map = serde_json::Map::new();
    for (key, value) in data.into_iter() {
        if f(&key) {
            map.insert(key, filter_map_with_keys(value, f));
        }
    }
    map
}

pub fn filter_map_with_keys<F>(data: JsonValue, f: &F) -> JsonValue
where
    F: Fn(&str) -> bool,
{
    match data {
        JsonValue::Null => JsonValue::Null,
        JsonValue::String(v) => JsonValue::String(v),
        JsonValue::Array(v) => JsonValue::Array(filter_list_for_map_keys(v, f)),
        JsonValue::Number(number) => JsonValue::Number(number),
        JsonValue::Object(map) => JsonValue::Object(filter_map_keys(map, f)),
        JsonValue::Bool(v) => JsonValue::Bool(v),
    }
}

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
