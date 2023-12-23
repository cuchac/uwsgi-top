use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ReqInfo {
    #[serde(default)]
    pub request_start: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Core {
    pub id: i64,
    pub vars: Vec<String>,
    pub req_info: ReqInfo,
    pub parsed_vars: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
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
