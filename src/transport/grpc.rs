
pub fn send_append_entries(&self, peer: &Peer, req: &AppendEntriesRequest) -> Result<AppendEntriesResponse> {
    // Serializa y envía el mensaje
}

pub fn send_request_vote(&self, peer: &Peer, req: &RequestVoteRequest) -> Result<RequestVoteResponse> {
    // Serializa y envía el mensaje
}