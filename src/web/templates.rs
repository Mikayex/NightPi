use askama::Template;

#[derive(Template)]
#[template(path = "index.html", print = "code")]
pub struct IndexTemplate {}

impl IndexTemplate {
    pub fn new() -> Self {
        Self {}
    }
}
