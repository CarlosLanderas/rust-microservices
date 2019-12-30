use std::thread;
use grpc_ring::Remote;
use grpc_ring::ring_grpc::{GreeterServer};
use grpc::Error;


fn main()  -> Result<(), Error> {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(8000);

    let remote = Remote::new("127.0.0.1:8000".parse().unwrap())?;
    server.add_service(GreeterServer::new_service_def(remote));
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}