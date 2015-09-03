extern crate hyper;

use hyper::Server;
use hyper::client::request::Request as ClientRequest;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri;
use hyper::net::Fresh;
use std::io::Read;

fn pass(req: Request, res: Response<Fresh>) {
    let url = match req.uri {
        RequestUri::AbsoluteUri(url) => Some(url),
        _ => None,
    }.unwrap();

    let new_res = ClientRequest::new(req.method, url);
    let mut body = String::new();
    new_res.unwrap().start().unwrap().send().unwrap().read_to_string(&mut body);
    let real = body.replace("Hoe", "Joe");
    println!("{:?}", body);
    res.send(real.as_bytes());
}

pub fn run() {
    Server::http("127.0.0.1:3000").unwrap().handle(pass);
}

#[test]
fn it_works() {
}
