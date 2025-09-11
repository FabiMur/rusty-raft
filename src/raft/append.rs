impl RaftState {
    pub fn append_entries(&mut self, entries: Vec<LogEntry>) {
        self.log.extend(entries);
    }

    pub fn handle_append_entries(&self, req: &AppendEntriesRequest) -> AppendEntriesResponse {

    }


}