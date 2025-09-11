use crate::raft::*;
use crate::transport::*;

use anyhow::Result;
use tracing::{info, warn};
use rand::Rng;
use tokio::time::{sleep, Duration};

/// Represents a Raft node in the cluster
pub struct Node {
    
    /// Unique identifier for the node
    pub id: u64,

    /// Raft state for the node
    pub raft_state: RaftState,

    /// List of peers in the cluster
    pub peers: Vec<Peer>,
    /// gRPC port for the node
    pub grpc_port: u16,
    /// HTTP port for the node
    pub http_port: u16,
}

impl Node {
    /// Creates a new Node with the given id, ports, and peers
    pub fn new(id: u64, grpc_port: u16, http_port: u16, peers: Vec<Peer>) -> Self {
        let num_nodes = peers.len() + 1; // Including self
        Node {
            id,
            raft_state: RaftState::new(num_nodes),
            peers,
            grpc_port,
            http_port,
        }
    }

    /// Starts the node's main event loop (stub)
    pub async fn start(&mut self) -> Result<()> {
        info!("Node {} starting on gRPC port {} and HTTP port {}", self.id, self.grpc_port, self.http_port);
        // Placeholder for starting gRPC server and HTTP server
        Ok(())
    }

    /// Returns a random election timeout duration
    fn random_election_timeout() -> Duration {
        let ms = rand::thread_rng().gen_range(150..=300);
        Duration::from_millis(ms)
    }

    /// Starts a new election for the node (stub)
    async fn start_election(&mut self) -> Result<()> {
        self.raft_state.become_candidate();
        self.raft_state.voted_for = Some(self.id);
        info!("Node {} started election for term {}", self.id, self.raft_state.current_term);

        // TODO: send RequestVote to peers
        // self.transport.broadcast_request_vote(...).await?;

        Ok(())
    }

    /// Runs the election timer for the node
    async fn run_election_timer(&mut self) {
        loop {
            let t = Self::random_election_timeout();
            sleep(t).await;
            if self.raft_state.role != Role::Leader {
                warn!("Node {} election timeout, starting election", self.id);
                self.start_election().await;
            }
        }
    }
}