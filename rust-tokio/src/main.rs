extern crate hyper;

use std::fs;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use std::sync::{Arc};

fn main() {
    let addr = ([0, 0, 0, 0], 80).into();

    let med_file_bytes_pointer = Arc::new(fs::read("./static/med.txt").unwrap());
    let big_file_bytes_pointer = Arc::new(fs::read("./static/big.txt").unwrap());
    let small_file_bytes_pointer = Arc::new(fs::read("./static/small.txt").unwrap());

    let new_svc = move || {
        let med_file_bytes = Arc::clone(&med_file_bytes_pointer);
        let small_file_bytes = Arc::clone(&small_file_bytes_pointer);
        let big_file_bytes = Arc::clone(&big_file_bytes_pointer);
        service_fn_ok(move |req: Request<Body>| {
            if req.uri().path() == "/med.txt" {
                return Response::new(Body::from(med_file_bytes.clone().to_vec()));
            } else if req.uri().path() == "/big.txt" {
                return Response::new(Body::from(big_file_bytes.clone().to_vec()));
            } else if req.uri().path() == "/small.txt" {
                return Response::new(Body::from(small_file_bytes.clone().to_vec()));
            } else {
                return Response::new(Body::from("404")); 
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}