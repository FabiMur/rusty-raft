
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


pub struct LogEntry {
    /// Term in which the entry was received
    pub term: u64,
    /// Data contained in the entry
    pub command: Command,
}
