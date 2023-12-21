use std::fs::File;
use std::io::{Read, Seek};
use crate::uwsgi_struct::UwsgiStatus;

pub struct StatsReader {
    file: File
}

impl StatsReader {
    pub fn new(path: &str) -> StatsReader {
        let file = File::open(path).expect("path should be valid json file");

        StatsReader {
            file
        }
    }

    pub fn read(&mut self) -> UwsgiStatus {
        let mut content = String::new();
        self.file.rewind().expect("file should be able to rewind");
        self.file.read_to_string(&mut content).expect("file should contains");

        let json: UwsgiStatus = serde_json::from_str(content.as_str()).expect("file content should contain valid json");

        json
    }
}