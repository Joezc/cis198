extern crate hyper;
extern crate rustc_serialize;
extern crate bbs;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::TcpStream;

use hyper::Client;
use hyper::server::{Request, Response, Server};
use hyper::status::StatusCode;
use rustc_serialize::json;
use bbs::Message;
use bbs::{SERVER_ADDR, BOT_ADDR, HTML_DATA, HTML_HEADER, HTML_FOOTER};

// Returns val from Ok(val) or sets the response to return an InternalServerError.
macro_rules! try_or_server_err {
    ($expr:expr, $res:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            *($res).status_mut() = StatusCode::InternalServerError;
            return;
        }
    })
}

fn read_file(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("file not found");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("Wrong reading the file");
    buffer
}

fn write_file(file_name: &str, content: String) {
    let mut buffer : File;
    buffer = OpenOptions::new().write(true).append(true).create(true).open(HTML_DATA).unwrap();
    buffer.write_all(content.as_bytes()).expect("Wrong writing to the file");
}

fn req_handler(mut req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            // Read the files [HTML_HEADER, HTML_DATA, HTML_FOOTER] into buf.
            // If HTML_DATA doesn't exist, it should be skipped without failure.
            // Use `try_or_server_err!(expression, res)` instead of `try!(expression)` in
            // order to return an internal server error.
            let mut buf = String::new();
            // TODO
            buf = buf + &read_file(HTML_HEADER) + &read_file(HTML_DATA) + &read_file(HTML_FOOTER);

            // And return buf as the response.
            *res.status_mut() = StatusCode::Ok;
            res.send(&buf.as_bytes()).unwrap();
        },
        hyper::Post => {
            // Read the message out of the `req` into a buffer, handle it, and respond with Ok.
            // TODO
            let mut buf = String::new();
            req.read_to_string(&mut buf).unwrap();

            let decoded: Message = json::decode(&buf).unwrap();
            let mut ans = String::new();
            ans = ans + &decoded.user + ": " + &decoded.text + "\n";

            let content: &str = &decoded.text;
            let vec: Vec<&str> = content.split(" ").collect();
            if vec[0] == "choose" {
                let bot_client = Client::new();
                let bot_url = "http://".to_string() + &BOT_ADDR;
                let mut response = bot_client.post(&bot_url).body(content).send().unwrap();
            }

            write_file(HTML_DATA, ans + "<br>");
            *res.status_mut() = StatusCode::Ok;
            res.send(String::new().as_bytes()).unwrap();
        },
        _ => *res.status_mut() = StatusCode::ImATeapot,
    }
}

fn main() {
    println!("Listening on {}.", SERVER_ADDR);
    match Server::http(SERVER_ADDR) {
        Ok(server) => match server.handle(req_handler) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        },
        Err(e) => println!("{:?}", e),
    }
}

#[cfg(test)]
mod test {
    use super::{HTML_HEADER, HTML_DATA, BOT_ADDR};
    use hyper::Client;
    #[test]
    fn test_read_file() {
        let file_name = HTML_DATA;
        println!("{}", super::read_file(file_name));
    }

    #[test]
    fn test_write_file() {
        let file_name = HTML_DATA;
        super::write_file(file_name, "dsd".to_string());
    }

    #[test]
    fn test_send_to_bot() {
        let buf = "choose 123 21".to_string();
        let bot_client = Client::new();
        let bot_url = "http://".to_string() + &BOT_ADDR;
        match bot_client.post(&bot_url).body(&buf).send() {
            Ok(response) => {
                println!("{}", response.status);
            },
            Err(e) => println!("{:?}", e),
        };
    }
}