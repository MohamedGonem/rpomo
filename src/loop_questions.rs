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
