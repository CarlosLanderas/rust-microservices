use std::thread;
use grpc_ring::ring_grpc::{GreeterServer, Greeter};
use grpc::{Error, RequestOptions, SingleResponse};
use grpc_ring::ring::{HelloRequest, HelloReply};

struct GreeterSvc;

impl Greeter for GreeterSvc {
    fn say_hello(&self, _o: RequestOptions, p: HelloRequest) -> SingleResponse<HelloReply> {
        println!("[Server] Received from client: {}", p.name);
        //self.client.say_hello(o, p)
        SingleResponse::completed(HelloReply {message : format!("Received! : {}", p.name), ..Default::default()})
    }
}

fn main()  -> Result<(), Error> {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(8000);

    server.add_service(GreeterServer::new_service_def(GreeterSvc));
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}