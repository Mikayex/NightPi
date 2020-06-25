use crate::system;
use askama::Template;
use log::warn;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

impl IndexTemplate {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Template)]
#[template(path = "settings.html")]
pub struct SettingsTemplate {
    ssh: bool,
}

impl SettingsTemplate {
    pub fn new() -> Self {
        Self {
            ssh: system::ssh_enabled().unwrap_or_else(|error| {
                warn!("Unable to get status for ssh service {}", error);
                false
            }),
        }
    }
}
