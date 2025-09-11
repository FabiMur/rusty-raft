mod grpc;
mod http;

pub use grpc::*;
pub use http::*;

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: u64,
    pub address: std::net::SocketAddr,
}