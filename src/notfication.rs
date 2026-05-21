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

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn urgency(mut self, urgency: impl Into<String>) -> Self {
        self.urgency = Some(urgency.into());
        self
    }

    pub fn timeout(mut self, ms: u32) -> Self {
        self.timeout = Some(ms);
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn send(self) {
        let mut cmd = std::process::Command::new("notify-send");
        if let Some(u) = self.urgency {
            cmd.args(["--urgency", &u]);
        }

        if let Some(ms) = self.timeout {
            cmd.args(["--expire-time", &ms.to_string()]);
        }

        if let Some(icon) = self.icon {
            cmd.args(["--icon", &icon]);
        }

        cmd.arg(&self.summary);

        if let Some(body) = self.body {
            cmd.arg(&body);
        }

        cmd.spawn().ok();
    }
}
