use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek};
use crate::uwsgi_struct::{Core, UwsgiStatus, Worker};

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

impl Worker {
    pub fn get_uri(&mut self) -> String {
        self.cores[0].get_var("REQUEST_URI")
    }
}

impl Core {
    pub fn get_var(&mut self, name: &str) -> String {
        if self.parsed_vars.is_none() {
            let mut map = HashMap::new();

            self.vars.iter().for_each(|v| {
                let parts = v.split_once('=');

                if parts.is_some() {
                    map.insert(parts.unwrap().0.to_string(), parts.unwrap().1.to_string());
                }
            });

            self.parsed_vars = Some(map);
        }

        match self.parsed_vars.as_mut().unwrap().get(name) {
            Some(v) => v.clone(),
            None => String::from(""),
        }
    }
}