// A tiny async TLS echo server with Tokio
extern crate native_tls;
extern crate tokio;
extern crate tokio_tls;

use std::fs;
use bytes::Bytes;
use native_tls::Identity;
use tokio::io;
use tokio::net::{TcpListener};
use tokio::prelude::*;

fn main() -> Result<(), Box<std::error::Error>> {
    // Bind the server's socket
    let addr = "0.0.0.0:4433".parse()?;
    let tcp = TcpListener::bind(&addr)?;

    // Create the TLS acceptor.
    let der = include_bytes!("user1-cert.p12");
    let cert = Identity::from_pkcs12(der, "hello")?;
    let tls_acceptor =
        tokio_tls::TlsAcceptor::from(native_tls::TlsAcceptor::builder(cert).build()?);

    let body = fs::read("static/med.txt").unwrap();
    let headers = format!("HTTP/1.1 200 OK\n\rContent-Length: {}\n\r\n\r", body.len());
    let mut data_vec = vec![];
    data_vec.extend_from_slice(headers.as_bytes());
    data_vec.extend_from_slice(&body[..]);
    let data = Bytes::from(data_vec);

    // Iterate incoming connections
    let server = tcp
        .incoming()
        .for_each(move |tcp| {
            let _data = data.clone();
            // Accept the TLS connection.
            let tls_accept = tls_acceptor
                .accept(tcp)
                .and_then(move |tls| {
                    // Split up the read and write halves
                    let (_, writer) = tls.split();

                    // Copy the data back to the client
                    let conn = io::write_all(writer, _data)
                        .map(|(_, n)| println!("wrote {} bytes", n.len()))
                        // Handle any errors
                        .map_err(|err| println!("IO error {:?}", err));

                    // Spawn the future as a concurrent task
                    tokio::spawn(conn);

                    Ok(())
                })
                .map_err(|err| {
                    println!("TLS accept error: {:?}", err);
                });
            tokio::spawn(tls_accept);

            Ok(())
        })
        .map_err(|err| {
            println!("server error {:?}", err);
        });

    // Start the runtime and spin up the server
    tokio::run(server);
    Ok(())
}
