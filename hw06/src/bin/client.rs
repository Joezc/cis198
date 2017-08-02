extern crate hyper;
extern crate rustc_serialize;
extern crate bbs;

use std::env::args;

use bbs::{UserClient, HTML_ADDR};
use hyper::Client;

fn main() {
    // TODO (see README)
    // example: get
    // example: post (username)xx (content)xx
    let args: Vec<String> = args().collect();
    match args[1].as_ref() {
        "get" => {
            let client = UserClient::new(String::new(), HTML_ADDR.to_string());
            println!("{}", client.get_content().unwrap().1);
        }
        "post" => {
            let client = UserClient::new(args[2].clone(), HTML_ADDR.to_string());
            println!("{}", client.send_msg(args[3].clone()).unwrap().1);
        }
        _ => {
            println!("Please input get or post");
        }
    }
}
