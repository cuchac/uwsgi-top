use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReqInfo {
    #[serde(default)]
    pub request_start: i64
}

#[derive(Serialize, Deserialize)]
pub struct Core {
    pub id: i64,
    pub vars: Vec<String>,
    pub req_info: ReqInfo,
}

#[derive(Serialize, Deserialize)]
pub struct Worker {
    pub id: i64,
    pub status: String,
    pub cores: Vec<Core>
}

#[derive(Serialize, Deserialize)]
pub struct UwsgiStatus {
    pub version: String,
    pub workers: Vec<Worker>
}