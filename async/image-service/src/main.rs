use std::path::Path;
use std::io::{Error, ErrorKind};
use hyper::{Server, Request, Body, Response, Method, StatusCode};
use hyper::service::service_fn;
use futures::{Future, future, Stream};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::fs;
use tokio::fs::File;
use regex::Regex;
use lazy_static::lazy_static;
use hyper_staticfile::FileChunkStream;

const INDEX : &str = "<html><body>Images micro service</body></html>";
lazy_static! {
    static ref DOWNLOAD_FILE: Regex = Regex::new("^/download/(?P<filename>\\w{20})?$").unwrap();
}

fn main() {
    let files = Path::new("./files");

    if !Path::exists(files) {
        match fs::create_dir(files) {
            Err(e) => panic!("Could not create directory files"),
            Result=> ()
        };
    }


    let addr = ([127,0,0,1], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(move || {
        service_fn(move |req| microservice_handler(req, &files))
    });

    let server = server.map_err(drop);
    hyper::rt::run(server);
}

fn microservice_handler(req: Request<Body>, files: &Path) -> Box<Future<Item=Response<Body>, Error=Error> + Send>
{
    match(req.method(), req.uri().path().to_owned().as_ref()) {
        (&Method::GET, "/") => {
            Box::new(future::ok(Response::new(INDEX.into())))
        },
        (&Method::GET, path) if path.starts_with("/download") => {
            if let Some(cap) = DOWNLOAD_FILE.captures(path) {
                let filename = cap.name("filename").unwrap().as_str();
                let mut filepath = files.to_path_buf();
                filepath.push(filename);
                let open_file = File::open(filepath);
                let body = open_file.map(|file| {
                    let chunks = FileChunkStream::new(file);
                    Response::new(Body::wrap_stream(chunks))
                });

                Box::new(body)

            } else {
                response_with_code(StatusCode::NOT_FOUND)
            }
        },
        (&Method::POST, "/upload") => {
            let name: String = thread_rng().sample_iter(&Alphanumeric).take(20).collect();
            let mut filepath = files.to_path_buf();
            filepath.push(&name);

            let create_file = File::create(filepath);
            let write = create_file.and_then(|file| {
                req.into_body()
                    .map_err(other)
                    .fold(file, |file, chunk| {
                        tokio::io::write_all(file, chunk)
                            .map(|(file, _)| file)
                    })
            });

            let body = write.map(|_| {
                Response::new(name.into())
            });

            Box::new(body)
        }
        _ => {
            response_with_code(StatusCode::NOT_FOUND)
        }
    }
}

fn response_with_code(status_code: StatusCode) -> Box<Future<Item=Response<Body>, Error=Error> + Send> {
    let resp = Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap();

    Box::new(future::ok(resp))
}

fn other<E>(err: E) -> std::io::Error
    where
        E: Into<Box<std::error::Error + Send + Sync>>,
{
    std::io::Error::new(ErrorKind::Other, err)
}
