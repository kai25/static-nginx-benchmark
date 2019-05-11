extern crate actix_web;

use bytes::Bytes;
use actix_web::{server, App, HttpRequest, Responder, HttpResponse};
use actix_web::http::{Method, ContentEncoding};
use std::fs;

fn index(req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .content_encoding(ContentEncoding::Identity)
        .body(Bytes::from(fs::read("/data/med.txt").unwrap()))
}

fn main() {
    server::new(|| App::new()
        .resource("/med.txt",
                  |r| r.method(Method::GET).f(index)))
        .bind("0.0.0.0:80")
        .unwrap()
        .run();
}
