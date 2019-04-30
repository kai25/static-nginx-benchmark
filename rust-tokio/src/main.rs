extern crate hyper;
extern crate http;

use std::fs;
use std::sync::Arc;
use bytes::Bytes;
use std::collections::HashMap;

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use http::response::{Builder};


fn main() {
    let addr = ([0, 0, 0, 0], 80).into();

    let mut mapping = HashMap::new();

    mapping.insert(
        "/med.txt",
        Bytes::from(fs::read("/data/med.txt").unwrap()),
    );

    mapping.insert(
        "/med1.txt",
        Bytes::from(fs::read("/data/med1.txt").unwrap()),

    );

    mapping.insert(
        "/med2.txt",
        Bytes::from(fs::read("/data/med2.txt").unwrap()),
    );

    let mapping_rc = Arc::new(mapping);

    let server = Server::bind(&addr)
        .serve(move || {
            let mapping_rc_cloned = mapping_rc.clone();
            service_fn_ok(move |req: Request<Body>| {
                match mapping_rc_cloned.get(req.uri().path()) {
                    Some(bytes) => Response::new(Body::from(bytes.clone())),
                    None => Builder::new()
                        .status(404)
                        .body(Body::from("404 not found"))
                        .unwrap(),
                }
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
