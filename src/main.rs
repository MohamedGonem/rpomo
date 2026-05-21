mod loop_questions;
mod notfication;
mod questions;

fn main() {
    loop {
        let session = questions::ask();
        let mut current_session: usize = 0;
    }

    notfication::Notification::new("Pomodoro")
        .body("All done! Great work today")
        .send();
}
