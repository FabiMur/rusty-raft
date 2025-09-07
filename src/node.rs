use core::str;
use std::os::unix::net::SocketAddr;
use crate::raft::types::LogEntry;

use anyhow::Result;
use tracing::{info, warn};

pub enum Role {
    Follower,
    Candidate,
    Leader,
}



pub struct Node {
    
    /// Unique identifier for the node
    pub id: u64,
    /// Role of the node in the cluster
    pub role: Role,
    /// Current term of the node
    pub current_term: u64,
    /// Candidate the node voted for in the current term
    pub voted_for: Option<u64>,


    // --- Log and State Machine Management ---

    /// Log entries for the node
    pub log: Vec<LogEntry>,
    /// Index of the last applied log entry
    pub last_applied: u64,
    /// Index of the highest log entry known to be committed
    pub commit_index: u64,

    // --- Communications ---

    /// List of peers in the cluster
    pub peers: Vec<(SocketAddr, u64)>,
    /// gRPC port for the node
    pub grpc_port: u16,
    /// HTTP port for the node
    pub http_port: u16,

    // --- Only used by the leader ---
    /// Index of the next log entry to send to each node
    pub next_index: Vec<u64>,
    /// Index of the highest log entry known to be replicated for each node
    pub match_index: Vec<u64>,
}