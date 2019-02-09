extern crate hyper;

use std::fs;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn main() {
    let addr = ([0, 0, 0, 0], 80).into();

    let med_file_bytes: Vec<u8> = fs::read("/data/med.txt").unwrap();
    let small_file_bytes: Vec<u8> = fs::read("/data/big.txt").unwrap();
    let big_file_bytes: Vec<u8> = fs::read("/data/small.txt").unwrap();

    let s_med_file:&'static[u8] = unsafe {std::mem::transmute(&med_file_bytes[..])};
    let s_big_file_bytes:&'static[u8] = unsafe {std::mem::transmute(&big_file_bytes[..])};
    let s_small_file_bytes:&'static[u8] = unsafe {std::mem::transmute(&small_file_bytes[..])};

    let new_svc = move || {
        service_fn_ok(move |req: Request<Body>| {
            if req.uri().path() == "/med.txt" {
                return Response::new(Body::from(s_med_file));
            } else if req.uri().path() == "/big.txt" {
                return Response::new(Body::from(s_big_file_bytes));
            } else if req.uri().path() == "/small.txt" {
                return Response::new(Body::from(s_small_file_bytes));
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
