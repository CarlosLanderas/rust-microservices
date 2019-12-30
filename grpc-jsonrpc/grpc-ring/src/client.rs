use failure::Error;
use std::net::SocketAddr;
use grpc::{ClientConf, ClientStubExt, RequestOptions};
use grpc_ring::ring_grpc::{GreeterClient, Greeter};
use grpc_ring::ring::{HelloReply, HelloRequest};
use grpc::Error as GrpcError;
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

fn main() -> Result<(), Error> {
    let client = Remote::new("127.0.0.1:8000".parse()?)?;

    for n in 1..10 {
        let reply = client.send_hello_request(format!("Carlos Landeras {}", n));
        println!("[Client] Reply from server: {}", reply.message);
    }

    Ok(())
}