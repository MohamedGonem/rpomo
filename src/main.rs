mod loop_questions;
mod notfication;
mod questions;

fn main() {
    loop {
        let session = questions::ask();
        let mut current_session: usize = 0;
        for session_time in session.sessions_time.iter() {
            notfication::Notification::new("Pomodoro Started")
                .body(&format!("Focus for {session_time} minutes"))
                .urgency("normal")
                .icon("clock")
                .send();

            std::thread::sleep(std::time::Duration::from_secs(
                session.session_length as u64 * 60,
            ));
            current_session += 1;
            if !loop_questions::ask_continue(current_session, session.sessions_count) {
                break;
            }
        }
        notfication::Notification::new("Pomodoro Done!")
            .body(&format!("Take a {} minute break", session.break_length))
            .urgency("critical")
            .timeout(0)
            .icon("dialog-information")
            .send();

        std::thread::sleep(std::time::Duration::from_secs(
            session.break_length as u64 * 60,
        ));

        if !loop_questions::ask_new() {
            break;
        }
    }

    notfication::Notification::new("Pomodoro")
        .body("All done! Great work today")
        .send();
}
