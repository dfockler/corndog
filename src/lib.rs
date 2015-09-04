extern crate hyper;

use hyper::Server;
use hyper::Url;
use hyper::client::request::Request as ClientRequest;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri;
use hyper::net::Fresh;
use std::io::Read;

fn pass(req: Request, res: Response<Fresh>) {
    // Setup a new Request based on the original
    let url = get_url(req.uri).unwrap();
    let new_req = ClientRequest::new(req.method, url);

    // Get the response from the new Request
    let mut new_res = new_req.unwrap().start().unwrap().send().unwrap();

    // Parse out the body and filter it
    let body = get_body(&mut new_res);
    let real = filter_body(&body);

    // Send the filtered response back to the client
    res.send(real.as_bytes());
}

fn filter_body(body: &String) -> String {
    body.replace("Hoe", "Joe")
}

fn get_url(uri: RequestUri) -> Option<Url> {
    match uri {
        RequestUri::AbsoluteUri(url) => Some(url),
        _ => None,
    }
}

fn get_body(res: &mut Read) -> String {
    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();
    return buffer;
}

pub fn run() {
    Server::http("127.0.0.1:3000").unwrap().handle(pass);
}

#[test]
fn it_works() {
}
