use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type JQuery;

    #[wasm_bindgen(js_name = "jQuery")]
    pub fn jquery(selector: &str) -> JQuery; //Ignore context

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get(this: &JQuery, index: u32) -> Element;

    #[wasm_bindgen(method, getter)]
    pub fn length(this: &JQuery) -> u32;
}

impl JQuery {
    /// Returns an iterator over the element of the jQuery object.
    pub fn iter(&self) -> JQueryIter<'_> {
        JQueryIter {
            range: 0..self.length(),
            jquery: self,
        }
    }

    /// Converts the content of the jQuery object into a new Vec.
    pub fn to_vec(&self) -> Vec<Element> {
        let len = self.length();

        let mut output = Vec::with_capacity(len as usize);

        for i in 0..len {
            output.push(self.get(i));
        }

        output
    }
}

#[derive(Debug, Clone)]
pub struct JQueryIter<'a> {
    range: std::ops::Range<u32>,
    jquery: &'a JQuery,
}

impl<'a> std::iter::Iterator for JQueryIter<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        Some(self.jquery.get(index))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<'a> std::iter::DoubleEndedIterator for JQueryIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        Some(self.jquery.get(index))
    }
}

impl<'a> std::iter::FusedIterator for JQueryIter<'a> {}

impl<'a> std::iter::ExactSizeIterator for JQueryIter<'a> {}
