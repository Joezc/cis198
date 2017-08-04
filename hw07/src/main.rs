extern crate hyper;
extern crate rustc_serialize;
extern crate websocket;

//mod webpage;
mod chatserver;
mod newwebpage;

fn main() {
    chatserver::start();
//    webpage::serve();
    newwebpage::serve();
}
