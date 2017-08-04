//#![deny(warnings)]
extern crate hyper;
extern crate futures;
extern crate pretty_env_logger;

use std::fs::{File};
use std::io::{Read};

use self::futures::future::FutureResult;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

const HTTP_ADDR: &'static str = "0.0.0.0:1980";
const HTML_DATA: &'static str = "html/index.html";

struct Webpage;

impl Service for Webpage {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;
    fn call(&self, _req: Request) -> Self::Future {
        let mut buf = String::new();
        let mut page =  File::open(HTML_DATA).unwrap();
        page.read_to_string(&mut buf).unwrap();

        futures::future::ok(
        Response::new()
                .with_header(ContentLength(buf.len() as u64))
//                .with_header(ContentType::plaintext())
                .with_body(buf)
        )
    }

}

pub fn serve() {
    pretty_env_logger::init().unwrap();
    let addr = HTTP_ADDR.parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Webpage)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}