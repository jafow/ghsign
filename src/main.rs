#[macro_use]
extern crate clap;
extern crate libghsign;

use clap::{Arg, App};
use libghsign::GhSign;

fn main() {

    // let matches = App::new("ghsign")
    //                     .version("0.1.0")
    //                     .about("signs wth gh public key")
    //                     .arg(Arg::with_name("sign")
    //                          .short("s")
    //                          .long("sign"))
    //                     .arg(Arg::with_name("verify")
    //                          .short("e")
    //                          .long("verify"))
    //                     .arg(Arg::with_name("username")
    //                          .short("u")
    //                          .long("user")
    //                          .takes_value(true)
    //                          .required(true))
    //                     .get_matches();
    let user = GhSign {
        username: String::from("jafow")
    };

    let data: String = String::from("hello world");
    let signer = user.sign();
}
