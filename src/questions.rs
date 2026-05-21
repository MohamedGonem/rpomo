pub struct Session {
    pub sessions_count: usize,
    pub session_length: usize,
    pub sessions_time: Vec<usize>,
    pub break_length: usize,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            sessions_count: 5,
            session_length: 5,
            sessions_time: vec![25, 20, 15, 10, 5],
            break_length: 5,
        }
    }
}
