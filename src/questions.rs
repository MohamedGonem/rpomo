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

fn ask_question(title: &str, text: &str) -> Option<usize> {
    let output = std::process::Command::new("zenity")
        .args([
            "--entry",
            &format!("--title={}", title),
            &format!("--text={}", text),
            "--entry-text=5",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let input = String::from_utf8(output.stdout).ok()?;
    input.trim().parse::<usize>().ok()
}

fn ask_n_sessions() -> Option<usize> {
    ask_question("Pomodoro", "How many sessions?")
}

fn ask_session_length() -> Option<usize> {
    ask_question("Pomodoro", "How long is one session?")
}

fn ask_break_length() -> Option<usize> {
    ask_question("Pomodoro", "How long is break time?")
}

fn calculate_sessions_length(sessions_count: usize, session_length: usize) -> Vec<usize> {
    (0..sessions_count)
        .map(|i| session_length * (sessions_count - i))
        .collect()
}
