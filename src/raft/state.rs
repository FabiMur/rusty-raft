
use crate::raft::*;

/// Represents the state of a Raft node
pub struct RaftState {
    /// Role of the node in the cluster
    pub role: Role,

    /// Current term of the node
    pub current_term: u64,

    /// Candidate the node voted for in the current term
    pub voted_for: Option<u64>,

    /// Log entries for the node
    pub log: Vec<LogEntry>,

    /// Index of the last applied log entry
    pub last_applied: u64,

    /// Index of the highest log entry known to be committed
    pub commit_index: u64,

    // --- Only used by the leader ---

    /// Index of the next log entry to send to each node
    pub next_index: Vec<u64>,

    /// Index of the highest log entry known to be replicated for each node
    pub match_index: Vec<u64>,
}

impl RaftState {
    /// Creates a new RaftState with default values
    pub fn new(num_nodes: usize) -> Self {
        RaftState {
            role: Role::Follower,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            last_applied: 0,
            commit_index: 0,
            next_index: vec![1; num_nodes], // Initialized to 1 (first log index)
            match_index: vec![0; num_nodes], // Initialized to 0
        }
    }

    /// Transitions the node to the follower state
    pub fn become_follower(&mut self, term: u64) {
        self.role = Role::Follower;
        self.current_term = term;
        self.voted_for = None;
    }

    /// Transitions the node to the candidate state
    pub fn become_candidate(&mut self) {
        self.role = Role::Candidate;
        self.current_term += 1;
        self.voted_for = None;
    }

    /// Transitions the node to the leader state
    pub fn become_leader(&mut self) {
        self.role = Role::Leader;
        // Initialize next_index and match_index for each follower
        let next_index = self.last_log_index() + 1;
        for i in 0..self.next_index.len() {
            self.next_index[i] = next_index;
            self.match_index[i] = 0;
        }
    }

    /// Returns the index of the last log entry
    pub fn last_log_index(&self) -> u64 {
        self.log.len() as u64
    }

    /// Returns the term of the last log entry
    pub fn last_log_term(&self) -> u64 {
        self.log.last().map(|e| e.term).unwrap_or(0)
    }

}