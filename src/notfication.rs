pub struct Notification {
    summary: String,
    body: Option<String>,
    urgency: Option<String>,
    timeout: Option<u32>,
    icon: Option<String>,
}

impl Notification {
    pub fn new(summary: impl Into<String>) -> Self {
        Self {
            summary: summary.into(),
            body: None,
            urgency: None,
            timeout: None,
            icon: None,
        }
    }
}
