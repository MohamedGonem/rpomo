mod loop_questions;
mod notfication;
mod questions;

fn main() {
    loop {
        let session = questions::ask();
        let mut current_session: usize = 0;
        for session_time in session.sessions_time.iter() {
            current_session += 1;
            let notfication_body: &str = if current_session == session.sessions_count {
                &format!("Last {} minutes. You can do it!", session.session_length)
            } else {
                &format!(
                    "{} minutes left!\n{} minutes till break. Focus!",
                    session_time, session.session_length
                )
            };

            notfication::Notification::new("Pomodoro Started")
                .body(notfication_body)
                .urgency("normal")
                .icon("clock")
                .send();

            std::thread::sleep(std::time::Duration::from_secs(
                session.session_length as u64 * 60,
            ));

            notfication::Notification::new("Session Done!")
                .body(&format!("Take a {} minute break", session.break_length))
                .urgency("critical")
                .timeout(0)
                .icon("dialog-information")
                .send();

            std::thread::sleep(std::time::Duration::from_secs(
                session.break_length as u64 * 60,
            ));
            if !loop_questions::ask_continue(current_session, session.sessions_count) {
                break;
            }
        }
        notfication::Notification::new("Pomodoro Done!")
            .body("Take a break")
            .urgency("critical")
            .timeout(0)
            .icon("dialog-information")
            .send();

        std::thread::sleep(std::time::Duration::from_secs(60));

        if !loop_questions::ask_new() {
            break;
        }
    }

    notfication::Notification::new("Pomodoro")
        .body("All done! Great work today")
        .send();
}
