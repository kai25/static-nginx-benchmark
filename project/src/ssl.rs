use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslAcceptorBuilder};

pub fn create_ssl_builder(
    key_path: &str, cert_path: &str,
) -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(key_path, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(cert_path).unwrap();
    builder
}