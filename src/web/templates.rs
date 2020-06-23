use askama::Template;

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
pub struct SettingsTemplate {}

impl SettingsTemplate {
    pub fn new() -> Self {
        Self {}
    }
}
