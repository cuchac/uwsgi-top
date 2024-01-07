use crate::uwsgi_struct::{Core, UwsgiStatus, Worker};
use crate::Settings;
use std::fs::File;
use std::io::{Read, Seek};
use std::net::TcpStream;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StatsReader {
    settings: Settings,
}

impl StatsReader {
    pub fn new(settings: &Settings) -> StatsReader {
        return StatsReader {
            settings: settings.clone(),
        };
    }

    pub fn read(&self) -> UwsgiStatus {
        if self.settings.file.is_some() {
            return StatsReader::read_from_file(self.settings.file.as_ref().unwrap());
        }

        if self.settings.socket.is_some() {
            return StatsReader::read_from_socket(self.settings.socket.as_ref().unwrap());
        }

        if self.settings.network.is_some() {
            return StatsReader::read_from_network(self.settings.network.as_ref().unwrap());
        }

        panic!("No input method selected!")
    }

    fn read_from_file(file: &PathBuf) -> UwsgiStatus {
        let mut file = File::open(file).expect("path should be valid json file");
        let mut content = String::new();
        file.rewind().expect("file should be able to rewind");
        file.read_to_string(&mut content)
            .expect("file should contains");

        let json: UwsgiStatus =
            serde_json::from_str(content.as_str()).expect("file content should contain valid json");

        json
    }

    fn read_from_socket(path: &PathBuf) -> UwsgiStatus {
        let mut socket = UnixStream::connect(path).expect("socket should be able to connect");
        let mut content = String::new();
        socket
            .read_to_string(&mut content)
            .expect("file should contains");

        let json: UwsgiStatus =
            serde_json::from_str(content.as_str()).expect("file content should contain valid json");

        json
    }

    fn read_from_network(address: &std::net::SocketAddr) -> UwsgiStatus {
        let mut socket = TcpStream::connect(address).expect("address should be able to connect");
        let mut content = String::new();
        socket
            .read_to_string(&mut content)
            .expect("address should return data");

        let json: UwsgiStatus =
            serde_json::from_str(content.as_str()).expect("returned data should contain valid json");

        json
    }
}

impl Worker {
    pub fn get_uri(&self) -> String {
        self.cores[0].get_uri()
    }

    pub fn get_core(&self) -> &Core {
        &self.cores[0]
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

    pub fn get_uri(&self) -> String {
        let mut parts = vec![
            "https://".to_string(),
            self.get_var("HTTP_HOST"),
            self.get_var("REQUEST_URI"),
        ];

        let qs = self.get_var("QUERY_STRING");
        if !qs.is_empty() {
            parts.push(format!("?{qs}"))
        }

        parts.join("")
    }
}
