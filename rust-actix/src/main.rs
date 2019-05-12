use std::fs::File;
use std::io::BufReader;

use bytes::Bytes;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs;


fn main() -> std::io::Result<()> {
    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(&include_bytes!("cert.pem")[..]);
    let key_file = &mut BufReader::new(&include_bytes!("key.pem")[..]);
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/med.txt").to(|req: HttpRequest| {
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(Bytes::from(fs::read("/data/med.txt").unwrap()))
            }))
    })
    .workers(2)
    .bind_rustls("0.0.0.0:443", config)?
    .run()
}