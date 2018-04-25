extern crate request;
extern crate clap;

use std::env;
use std::collections::HashMap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("ghsign")
                    .version("0.1.0")
                    .arg(Arg::with_name("sign")
                         .short("s")
                         .long("sign")
                         .value_name("sign"))
                    .arg(Arg::with_name("verify")
                         .short("r")
                         .long("verify")
                         .value_name("verify"))
                    .get_matches();



    if let Some(k) = matches.value_of("sign") {
        println!("signer: {:?}", k);
    };

    println!("Hello, world!");
}
