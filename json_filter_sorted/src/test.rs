use crate::{filter, sort};
use serde_json::Value as JsonValue;

#[test]
fn sort_test() {
    let j = json!(
        {"d": 1, "e": 2, "f": 3, "a": 5, "b": 4, "c": 5}
    );
    let k = sort::sort_json(j)
        .unwrap()
        .as_object()
        .map(|x| x.to_owned())
        .unwrap();
    let sorted_keys = vec!["a", "b", "c", "d", "e", "f"];
    let mut after_sort = vec![];
    for k1 in k.keys() {
        after_sort.push(k1);
    }
    assert_eq!(after_sort, sorted_keys)
}

#[test]
fn filter_test() {
    let j = filter::filter(
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
            JsonValue::Null => false,
            _ => true,
        },
    );

    assert_eq!(j, json!([1,3,{"a": [1,2,3,4]}]))
}

#[test]
fn filter_map_keys() {
    let j = json!(
        {"e": 2, "f": 3, "a": 5, "b": 4, "c": 5}
    );
    let k = filter::filter_map_with_keys(
        json!({"d": 1, "e": 2, "f": 3, "a": 5, "b": 4, "c": 5}),
        &|key| {
            if key.contains("d") {
                false
            } else {
                true
            }
        },
    );
    assert_eq!(j, k)
}
