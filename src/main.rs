#[macro_use]
extern crate clap;
extern crate request;

use std::collections::HashMap;
use clap::{Arg, App};

fn main() {

    let matches = App::new("ghsign")
                        .version("0.1.0")
                        .about("signs wth gh public key")
                        .arg(Arg::with_name("sign")
                             .short("s")
                             .long("sign"))
                        .arg(Arg::with_name("verify")
                             .short("e")
                             .long("verify"))
                        .arg(Arg::with_name("username")
                             .short("u")
                             .long("user")
                             .takes_value(true)
                             .required(true))
                        .get_matches();

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
