use core::str;
use std::os::unix::net::SocketAddr;
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