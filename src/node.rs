use std::net::SocketAddr;
use crate::raft::*;

use anyhow::Result;
use tracing::{info, warn};




pub struct Node {
    
    /// Unique identifier for the node
    pub id: u64,

    /// Raft state for the node
    pub raft_state: RaftState,

    /// List of peers in the cluster
    pub peers: Vec<(SocketAddr, u64)>,
    /// gRPC port for the node
    pub grpc_port: u16,
    /// HTTP port for the node
    pub http_port: u16,
}

impl Node {
    /// Creates a new Node with the given id, ports, and peers
    pub fn new(id: u64, grpc_port: u16, http_port: u16, peers: Vec<(SocketAddr, u64)>) -> Self {
        let num_nodes = peers.len() + 1; // Including self
        Node {
            id,
            raft_state: RaftState::new(num_nodes),
            peers,
            grpc_port,
            http_port,
        }
    }

    /// Starts the node's main event loop (placeholder)
    pub async fn start(&mut self) -> Result<()> {
        info!("Node {} starting on gRPC port {} and HTTP port {}", self.id, self.grpc_port, self.http_port);
        // Placeholder for starting gRPC server and HTTP server
        Ok(())
    }
}