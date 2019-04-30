extern crate hyper;
extern crate http;

use std::fs;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use http::response::{Builder};

fn main() {
    let addr = ([0, 0, 0, 0], 80).into();

    let server = Server::bind(&addr)
        .serve(|| {
            service_fn_ok(|req: Request<Body>| {
                match req.uri().path() {
                    "/med.txt" => Response::new(Body::from(fs::read("/data/med.txt").unwrap())),
                    _ => Builder::new()
                        .status(404)
                        .body(Body::from("404 not found"))
                        .unwrap(),
                }
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
