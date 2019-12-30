use std::path::Path;
use std::fs;
use hyper::Server;
use hyper::service::service_fn;
use futures::Future;

fn main() {
    let files = Path::new("./files");
    fs::create_dir(files).ok();
    let addr = ([127,0,0,1], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(move || {
        service_fn(move |req| microservice_handler(req, &files))
    });

    let server = server.map_err(drop);
    hyper::rt::run(server);
}
