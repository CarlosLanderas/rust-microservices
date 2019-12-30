mod ring;
mod ring_grpc;
use crate::ring::HelloRequest;
use crate::ring_grpc::{Greeter, GreeterClient};
use grpc::{ClientConf, ClientStubExt, Error as GrpcError, RequestOptions};
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
        Ok(Self{client})
    }
}