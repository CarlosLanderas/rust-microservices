pub mod ring;
pub mod ring_grpc;

use crate::ring::{HelloRequest, HelloReply};
use crate::ring_grpc::{Greeter, GreeterClient};
use grpc::{ClientConf, ClientStubExt, Error as GrpcError, RequestOptions, SingleResponse};
use std::net::SocketAddr;

pub struct Remote {
    client: GreeterClient
}

impl Remote {
    pub fn new(addr: SocketAddr) -> Result<Self, GrpcError> {
        let host = addr.ip().to_string();
        let port = addr.port();
        let conf = ClientConf::default();
        let client = GreeterClient::new_plain(&host, port, conf)?;
        Ok(Self { client })
    }

    pub fn send_hello_request<S : Into<String>>(&self, name: S) -> HelloReply {

        let message = HelloRequest {
            name : String::from(name.into()),
            ..Default::default()
        };

       let reply = self.client.say_hello(RequestOptions::new(), message)
            .wait()
            .map(|(_, value, _)| value).unwrap();

        reply
    }
}

impl Greeter for Remote {
    fn say_hello(&self, _o: RequestOptions, p: HelloRequest) -> SingleResponse<HelloReply> {
        println!("[Server] Received from client: {}", p.name);
        //self.client.say_hello(o, p)
        SingleResponse::completed(HelloReply {message : format!("Received! : {}", p.name), ..Default::default()})
    }
}