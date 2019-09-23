use crate::{ErrorKind, JsonValue, MyResult};
use failure::Fail;
use serde_json;
use std::collections::{BTreeMap, HashMap};

pub fn sort_list(value: JsonValue, recursive: bool) -> MyResult<JsonValue> {
    let list: Vec<JsonValue> = value
        .as_array()
        .map(|x| x.to_owned())
        .ok_or(ErrorKind::ValueError)?;
    let mut new_list: Vec<JsonValue> = vec![];
    for json_value in list.into_iter() {
    	if recursive {
    	   new_list.push(sort_json(json_value)?)
    	} else {
    		new_list.push(json_value);
      }
    }
    serde_json::to_value(new_list).map_err(|err| err.context("Not able to Serialize").into())
}

pub fn sort_map(value: JsonValue, recursive: bool) -> MyResult<JsonValue> {
    let map = value
        .as_object()
        .map(|x| x.to_owned())
        .ok_or(ErrorKind::ValueError)?;
    let mut new_map = HashMap::new();
    for (key, value) in map.into_iter() {
    	if recursive {
        	new_map.insert(key, sort_json(value)?);
    	} else {
  		    new_map.insert(key, value);
      }
    }
    let btree_map: BTreeMap<_, _> = new_map.iter().collect();
    serde_json::to_value(btree_map).map_err(|err| err.context("Not able to Serialize").into())
}

pub fn sort_json_shallow(value: JsonValue) -> MyResult<JsonValue> {
    if value.is_object() {
        return sort_map(value, false);
    }
    if value.is_array() {
        return sort_list(value, false);
    }
    Ok(value)
}

pub fn sort_json(value: JsonValue) -> MyResult<JsonValue> {
    if value.is_object() {
        return sort_map(value, true);
    }
    if value.is_array() {
        return sort_list(value, true);
    }
    Ok(value)
}
