fn ask_question(text: &str) -> bool {
    std::process::Command::new("zenity")
        .args([
            "--question",
            "--title=Pomodoro",
            &format!("--text=Session {text}"),
        ])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn ask_continue(current_session: usize, total_sessions: usize) -> bool {
    ask_question(&format!(
        "{}/{} done!\nContinue to next session?",
        current_session, total_sessions
    ))
}
