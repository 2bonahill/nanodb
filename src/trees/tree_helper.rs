use serde::Deserialize;
use serde_json::Value;

use crate::{error::NanoDBError, trees::tree::PathStep};

fn _new_path_is_valid(
    data: &Value,
    path: &[PathStep],
    new_step: &PathStep,
) -> Result<bool, NanoDBError> {
    let mut current = data;
    for step in path {
        match step {
            PathStep::Key(k) => {
                if let Some(new_data) = current.get(k) {
                    current = new_data;
                } else {
                    return Err(NanoDBError::InvalidJSONPath);
                }
            }
            PathStep::Index(i) => {
                if let Some(new_data) = current.get(*i) {
                    current = new_data;
                } else {
                    return Err(NanoDBError::InvalidJSONPath);
                }
            }
        }
    }
    match new_step {
        PathStep::Key(k) => {
            if current.get(k).is_none() {
                return Err(NanoDBError::InvalidJSONPath);
            };
        }
        PathStep::Index(idx) => {
            if !current.is_array() {
                return Err(NanoDBError::NotAnArray("xx".to_owned())); // TODO: better error message
            }
            if current.get(idx).is_none() {
                return Err(NanoDBError::IndexOutOfBounds(*idx));
            }
        }
    }
    Ok(true)
}

fn _goto<T: for<'de> Deserialize<'de>>(data: Value, path: Vec<PathStep>) -> Result<T, NanoDBError> {
    // get the right json field according to the path steps in path
    let data = path.iter().fold(data, |acc, step| match step {
        PathStep::Key(k) => acc.get(k).unwrap().clone(),
        PathStep::Index(i) => acc.get(*i).unwrap().clone(),
    });
    serde_json::from_value(data).map_err(Into::into)
}
