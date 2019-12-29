use hyper::{Server, Response, Body};
use hyper::service::service_fn_ok;
use log::{debug, info, trace};
use hyper::rt::Future;
use std::env;
use dotenv::dotenv;
use clap::{App, crate_name, crate_authors, crate_description, Arg};
use std::net::SocketAddr;
use std::fs::File;
use std::io::{Read, ErrorKind};
use serde_derive::Deserialize;
use log::{warn};

#[derive(Deserialize)]
struct Config {
    address: SocketAddr,
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("ADDRESS")
            .help("Set an address for the server")
            .takes_value(true))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();


    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");

    let config = File::open("microservice.toml")
        .and_then(|mut file| {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        }).and_then(|buffer| {
            toml::from_str::<Config>(&buffer)
                .map_err(|err| std::io::Error::new(ErrorKind::Other, err))
        }).map_err(|err| {
            warn!("Can't read config file: {}", err);
        })
        .ok();

    let addr = matches.value_of("address")
        .map(|s| s.to_owned())
        .or(env::var("ADDRESS").ok())
        .and_then(|addr| addr.parse().ok())
        .or(config.map(|config| config.address))
        .or_else(|| Some(([127,0,0,1], 8080).into()))
        .unwrap();

    debug!("Trying to bind server to address: {}", addr);

    let builder = Server::bind(&addr);
    trace!("Creating service handler...");
    let server = builder.serve(|| {
        service_fn_ok(|req| {
            trace!("Incoming request is: {:?}", req);
            let random_byte = rand::random::<u8>();
            debug!("Generated value is: {}", random_byte);
            Response::new(Body::from(random_byte.to_string()))
        })
    });

    info!("Used address: {}", server.local_addr());
    let server = server.map_err(drop);
    debug!("Server is starting...");
    hyper::rt::run(server);
}
