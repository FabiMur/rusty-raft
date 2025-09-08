
#[derive(Eq, PartialEq, Debug)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

// Supported commands in the log entries 
// Get and MGet are not supported as they do not modify state so they don't need to be replicated
pub enum Command {
    // SET key value
    Set   { key: String, value: Vec<u8> },

    // MSET key value [key value ...]
    MSet  { pairs: Vec<(String, Vec<u8>)> },

    /// DEL key
    Del   { key: String },

    // Increment a value in a key
    IncrBy { key: String, delta: i64 },

    // Insert a value at the beginning of a list
    LPush { key: String, value: Vec<u8> },

    // Insert a value at the end of a list
    RPush { key: String, value: Vec<u8> },

    // HSET key field value
    HSet  { key: String, field: String, value: Vec<u8> },

    // HDel key field
    HDel  { key: String, field: String },

    // RENAME key new_key
    Rename { key: String, new_key: String },
}

/// A log entry in the Raft log
pub struct LogEntry {
    /// Term in which the entry was received
    pub term: u64,
    /// Data contained in the entry
    pub command: Command,
}

enum RpcMessage{
    RequestVote(RequestVote),
    RequestVoteResponse(RequestVoteResponse),
    AppendEntries(AppendEntries),
    AppendEntriesResponse(AppendEntriesResponse),
}

// RequestVote RPC for leader election, only called by candidates
struct RequestVote {
    term: u64,
    candidate_id: String,
    last_log_index: u64,
    last_log_term: u64,
}

// Response to RequestVote RPC
struct RequestVoteResponse {
    term: u64,
    vote_granted: bool,
}

// AppendEntries RPC for log replication and heartbeats, only called by leaders
struct AppendEntries {
    term: u64,
    leader_id: String,
    prev_log_index: u64,
    prev_log_term: u64,
    entries: Vec<LogEntry>,
    leader_commit: u64,
}

// Response to AppendEntries RPC
struct AppendEntriesResponse {
    term: u64,
    success: bool,
    match_index: u64,
}