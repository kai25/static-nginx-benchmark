extern crate hyper;

use std::fs;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use std::sync::{Arc};


fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let file_data = Arc::new(fs::read("./static/med.txt").unwrap());

    let new_svc = move || {
        let file_data_cloned = file_data.clone();
        service_fn_ok( move |_req: Request<Body>| {
            Response::new(Body::from(file_data_cloned.to_vec()))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}