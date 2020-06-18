use wasm_bindgen::prelude::*;
use web_sys::console;

mod jquery;
use jquery::jquery;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn navbar_update_active(page: &str) {
    let selector = format!(".nav-link[data-page=\"{}\"]", page);
    let item = jquery(&selector);
    match item.length() {
        0 => {
            console::error_2(&"No item found matching".into(), &selector.into());
        }
        1 => {
            let element = item.get(0);
            element.class_list().add_1("active").unwrap();
            let mut content = element.inner_html();
            content.push_str("<span class=\"sr-only\">(current)</span>");
            element.set_inner_html(content.as_str());
        }
        _ => {
            console::error_2(&"Too many items found matching".into(), &selector.into());
        }
    }
}
