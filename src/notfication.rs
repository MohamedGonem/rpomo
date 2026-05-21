pub struct Notification {
    summary: String,
    body: Option<String>,
    urgency: Option<String>,
    timeout: Option<u32>,
    icon: Option<String>,
}
