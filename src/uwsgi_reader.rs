use crate::uwsgi_struct::{Core, UwsgiStatus, Worker};
use std::fs::File;
use std::io::{Read, Seek};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StatsReader {
    file: File,
}

impl StatsReader {
    pub fn new(path: &str) -> StatsReader {
        let file = File::open(path).expect("path should be valid json file");

        StatsReader { file }
    }

    pub fn read(&mut self) -> UwsgiStatus {
        let mut content = String::new();
        self.file.rewind().expect("file should be able to rewind");
        self.file
            .read_to_string(&mut content)
            .expect("file should contains");

        let json: UwsgiStatus =
            serde_json::from_str(content.as_str()).expect("file content should contain valid json");

        json
    }
}

impl Worker {
    pub fn get_uri(&self) -> String {
        let mut parts = vec!["https://".to_string(), self.cores[0].get_var("HTTP_HOST"), self.cores[0].get_var("REQUEST_URI")];

        let qs = self.cores[0].get_var("QUERY_STRING");
        if !qs.is_empty() {
            parts.push(format!("?{qs}"))
        }

        parts.join("")
    }

    pub fn get_duration(&self) -> i64 {
        let rs = self.cores[0].req_info.request_start;

        if rs == 0 {
            return 0;
        }

        let start = SystemTime::now();
        (start
            .duration_since(UNIX_EPOCH)
            .expect("Invalid time")
            .as_secs()
            - rs) as i64
    }

    pub fn has_request(&self) -> bool {
        !self.cores[0].vars.is_empty()
    }
}

impl Core {
    pub fn get_var(&self, name: &str) -> String {
        match self.vars.get(name) {
            Some(v) => v.clone(),
            None => String::from(""),
        }
    }
}
