use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct ReqInfo {
    #[serde(default)]
    pub request_start: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Core {
    pub id: i64,
    #[serde(deserialize_with = "parse_vars")]
    pub vars: HashMap<String, String>,
    pub req_info: ReqInfo,
}

fn parse_vars<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let vars: Result<Vec<String>, D::Error> = Deserialize::deserialize(deserializer);

    // do better hex decoding than this
    let mut map = HashMap::new();

    vars.unwrap().iter().for_each(|v| {
        let parts = v.split_once('=');

        if parts.is_some() {
            map.insert(parts.unwrap().0.to_string(), parts.unwrap().1.to_string());
        }
    });

    Ok(map)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Worker {
    pub id: i64,
    pub status: String,
    pub cores: Vec<Core>,
}

#[derive(Serialize, Deserialize)]
pub struct UwsgiStatus {
    pub version: String,
    pub workers: Vec<Worker>,
}
