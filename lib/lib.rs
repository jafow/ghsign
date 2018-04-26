extern crate request;

use std::collections::HashMap;

pub struct GhSign {
    pub username: String
}

impl GhSign {

    fn fetch_key () {
        let username = matches.value_of("username").unwrap();
        let base_url = "https://github.com/";
        let request_url = format!("{}{}.keys", base_url, username);

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Connection".to_string(), "close".to_string());

        let res = match request::get(&request_url, &mut headers) {
            Ok(res) => res,
            Err(err) => { println!("Error no keys: {}", err); return; }
        };
    }
}

