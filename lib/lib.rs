extern crate reqwest;
extern crate curl;

use curl::easy::Easy;
use std::io::{stdout, Write};

pub struct GhSign {
    pub username: String
}

impl GhSign {

    fn fetch_key (self) {
        let base_url = "https://github.com/";
        let request_url = format!("{}{}.keys", base_url, self.username);

        let mut easy = Easy::new();
        easy.url(&request_url).unwrap();
        easy.write_function(|data| {
            Ok(stdout().write(data).unwrap())
        }).unwrap();
        easy.perform().unwrap();
    }

    pub fn sign (self) {
        let pk = self.fetch_key();
    }
}
