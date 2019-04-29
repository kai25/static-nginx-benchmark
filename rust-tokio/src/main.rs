extern crate hyper;
extern crate http;

use std::fs;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use bytes::Bytes;
use http::response::{Builder};

fn main() {
    let addr = ([0, 0, 0, 0], 80).into();

    let med_file_bytes = Bytes::from(fs::read("/data/med.txt").unwrap());
    let small_file_bytes = Bytes::from(fs::read("/data/small.txt").unwrap());
    let big_file_bytes = Bytes::from(fs::read("/data/big.txt").unwrap());

    let server = Server::bind(&addr)
        .serve(move || {
            let med_file_bytes = med_file_bytes.clone();
            let small_file_bytes = small_file_bytes.clone();
            let big_file_bytes = big_file_bytes.clone();


            service_fn_ok(move |req: Request<Body>| {
                match req.uri().path() {
                    "/med.txt" => Response::new(Body::from(med_file_bytes.clone())),
                    "/big.txt" => Response::new(Body::from(big_file_bytes.clone())),
                    "/small.txt" => Response::new(Body::from(small_file_bytes.clone())),
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
