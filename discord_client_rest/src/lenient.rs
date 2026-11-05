use log::warn;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn parse_lenient_vec<T: DeserializeOwned>(value: Value) -> Vec<T> {
    let Some(array) = value.as_array() else {
        return Vec::new();
    };

    array
        .iter()
        .filter_map(|item| match serde_json::from_value::<T>(item.clone()) {
            Ok(parsed) => Some(parsed),
            Err(e) => {
                warn!("Skipping element that failed to deserialize: {}", e);
                None
            }
        })
        .collect()
}
