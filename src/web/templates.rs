use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "navbar.html", print = "code")]
pub struct NavbarTemplate<'a> {
    current_page: &'a str,
}

impl<'a> NavbarTemplate<'a> {
    fn new(current_page: &'a str) -> Self {
        Self { current_page }
    }

    #[inline]
    fn current_page_active(&self, endpoint: &&str) -> bool {
        self.current_page == *endpoint
    }
}
