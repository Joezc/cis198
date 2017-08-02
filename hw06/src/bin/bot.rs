extern crate hyper;
extern crate rustc_serialize;
extern crate bbs;

use std::io::Read;
use std::net::TcpListener;

use hyper::Client;
use hyper::header::UserAgent;
use hyper::server::{Server, Request, Response};

use bbs::{BOT_ADDR, HTML_ADDR};
use bbs::UserClient;
use bbs::Message;

use rustc_serialize::json;


/// get post from server
fn req_handler(mut req: Request, mut res: Response) {
    if req.method != hyper::Post {
        return
    }
    let mut buf = String::new();
    req.read_to_string(&mut buf).unwrap();

    let vec: Vec<&str> = buf.split(" ").collect();
    if vec[0] != "choose" {
        return
    }
    let mut numbers: Vec<i32> = Vec::new();
    for i in &vec {
        if i.to_string() != "choose" {
            numbers.push(i.parse::<i32>().unwrap());
        }
    }
    let choosed_number = choose(numbers);
    send_to_server(choosed_number);
}

fn choose(numbers: Vec<i32>) -> String {
    let rand_client: Client = hyper::Client::new();
    let url = "https://www.random.org/integers/?num=1&min=1&max=".to_string() + &numbers.len().to_string()
        + "&col=1&base=10&format=plain&rnd=new";
    let mut response = rand_client.get(&url).send().unwrap();
    let mut buf = String::new();
    response.read_to_string(&mut buf).unwrap();
//    println!("string={}", buf);
    let new_line = buf.len() - 1;
    buf.truncate(new_line);
    let idx = buf.parse::<i32>().unwrap() - 1;
    return numbers[idx as usize].to_string();
}

fn send_to_server(number: String) {
    println!("send number {} to server", number);
    let client = UserClient::new("bot".to_string(), HTML_ADDR.to_string());
    client.send_msg(number);
}

fn main() {
    // Create a bot user.
    // TODO

    // Start TcpListener.
    // TODO
    println!("Listening on {}.", BOT_ADDR);
    match Server::http(BOT_ADDR) {
        Ok(server) => match server.handle(req_handler) {
            Ok(_) => (),
            Err(e) => println!("server.hanle error: {:?}", e),
        },
        Err(e) => println!("Server::http error: {:?}", e),
    }

    // Listen for incoming TCP connections.
    // For each connection, accept it and read all data from the stream.
    // If it's a 'choose' message, post to the BBS with a response (via the above bot user).
    // TODO
}

#[cfg(test)]
mod test {

    #[test]
    fn test_choose() {
        let a = vec![123, 21, 3133, 23, 32, 42123];
        println!("{}", super::choose(a));
    }

    #[test]
    fn test_convert() {
        let mut my_string = "2\n".to_string();
        let int_vec = String::new();
//        for i in &my_string {
//            if i >= "0" && i <= "9" {
//                int_vec = int_vec + &i;
//            }
//        }
        let new_len = my_string.len()-1;
        my_string.truncate(new_len);
        let my_int = my_string.parse::<i32>().unwrap();
        println!("{}", my_int);
    }

    #[test]
    fn test_send_to_server() {
        super::send_to_server("10".to_string());
    }
}