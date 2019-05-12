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

    let file_bytes = Bytes::from(fs::read("/data/med.txt").unwrap());

    HttpServer::new(move || {
        let _bytes = file_bytes.clone();
        App::new()
            .service(web::resource("/med.txt").to(move |_req: HttpRequest| {
                HttpResponse::Ok()
                    .body(_bytes.clone())
            }))
    })
    .workers(4)
    .bind_ssl("0.0.0.0:443", builder)?
    .run()
}