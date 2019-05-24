#[macro_use]
extern crate log;
extern crate env_logger;
extern crate futures;

use std::{ fs, process, env };

use bytes::Bytes;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Error, Responder};
use env_logger::Env;

use futures::future::{Future, result};

mod config;
mod ssl;
mod file_manager;

use file_manager::FileManager;
use config::{Config};
use ssl::create_ssl_builder;
use std::fs::File;
use std::sync::Arc;


fn init_logger() {
    env_logger::from_env(
        Env::default().default_filter_or("info")
    ).init();
}

fn get_config_from_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => Ok("config.yml".to_owned()),
        1 => Ok(args[0].clone()),
        _ => Err("Unexpected amount of arguments".to_owned())
    }
}


fn main() -> std::io::Result<()> {
    init_logger();

    let config_path = get_config_from_args();
    if let Err(msg) = config_path {
        error!("{}", msg);
        process::exit(1);
    }

    let config = match Config::from_file("config.yml") {
        Ok(config) => config,
        Err(msg) => {
            error!("{}", msg);
            process::exit(2)
        },
    };

    info!("Running with config {:?}", config);

    let builder = create_ssl_builder(
        &config.ssl.key_path, &config.ssl.cert_path
    );

    let file_bytes = Bytes::from(fs::read("ssl/cert.pem").unwrap());

    let file_manager = Arc::new(
        FileManager::from_config(&config.content));

    fn handler(req: &HttpRequest, file_manager: &FileManager)
        -> Box<Future<Item=HttpResponse, Error=Error>> {
        Box::from(result(match file_manager.get(&req.path()) {
            Some(bytes) => {
//                info!("Got request {}, 200", req.path());
                Ok(HttpResponse::Ok().body(bytes))
            },
            None => {
//                info!("Got request {}, 404", req.path());
                Ok(HttpResponse::NotFound().finish())
            }
        }))
    }

    HttpServer::new(move || {
        let file_manager = file_manager.clone();
        App::new()
            .default_service(
                web::route().to(
                    move |req: HttpRequest| handler(&req, &file_manager)))
    })
    .workers(config.runtime.workers)
    .bind(config.get_http_address())?
    .bind_ssl(config.get_ssl_address(), builder)?
    .run()
}
