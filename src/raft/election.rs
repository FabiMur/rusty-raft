/// Implementation of the election related RPCs
impl RaftState {

    /// Starts an election by transitioning to candidate state and sending RequestVote RPCs
    pub fn start_election(&mut self, node_id: u64) {
        self.become_candidate();
        self.voted_for = node_id;
        let req = RequestVote {
            term: self.current_term,
            candidate_id: node_id,
            last_log_index: self.last_log_index(),
            last_log_term: self.log.last().map(|e| e.term).unwrap_or(0),
        };



        for peer in &self.peers {
            transport::grpc::send_request_vote(peer, req.clone());
        }

        info!("Node {} started election for term {}", node_id, self.current_term);
    }


    /// Handles a RequestVote RPC
    pub fn handle_request_vote(
        &mut self,
        term: u64,
        candidate_id: u64,
        candidate_last_log_index: u64,
        candidate_last_log_term: u64,
    ) -> bool {
        if term > self.current_term {
            self.current_term = term;
            self.voted_for = None;
        }
        let my_last_log_index = self.log.len().saturating_sub(1) as u64;
        let my_last_log_term = self.log.last().map(|e| e.term).unwrap_or(0);

        // Candidate's log is at least as up-to-date as receiver's log
        // Candidate has newer term in last log entry, or same term but newer or same index
        let log_up_to_date =
            candidate_last_log_term > my_last_log_term ||
            (candidate_last_log_term == my_last_log_term && candidate_last_log_index >= my_last_log_index);

        if self.voted_for.is_none() && log_up_to_date {
            self.voted_for = Some(candidate_id);
            info!("Node {} voted for candidate {} in term {}", self.id, candidate_id, term);
            return true;
        }

        info!("Node {} rejected vote for candidate {} in term {}", self.id, candidate_id, term);
        false
    }
    
}