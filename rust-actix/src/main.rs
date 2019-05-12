use std::fs::File;
use std::io::BufReader;

use std::fs;
use bytes::Bytes;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};


fn main() -> std::io::Result<()> {
    // Create the TLS acceptor.
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/etc/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("/etc/cert.pem").unwrap();


    HttpServer::new(|| {
        App::new()
            .service(web::resource("/med.txt").to(|req: HttpRequest| {
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(Bytes::from(fs::read("/data/med.txt").unwrap()))
            }))
    })
    .workers(4)
    .disable_signals()
    .bind_ssl("0.0.0.0:443", builder)?
    .run()
}