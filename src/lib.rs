mod ld_md;
mod sites;
mod utils;
use cfg_if::cfg_if;
use ld_md::RecipeMarkdownBuilder;
use scraper::{Html, Selector};
use wasm_bindgen::prelude::*;
use crate::sites::Sites;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn get_ld_json(contents: &str) -> String {
    let document = Html::parse_document(contents);
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
    let ctx = document.select(&selector).next().unwrap();
    let text = ctx.text().collect::<Vec<_>>();
    let as_txt = text.join("");
    let as_recipe: Sites = serde_json::from_str(&as_txt).unwrap();
    let mut builder = RecipeMarkdownBuilder::new(&as_recipe);
    builder.build().into()
}

#[cfg(test)]
mod tests {

    use crate::get_ld_json;

    #[test]
    fn hummus() {
        let src = include_str!("../tests/hummus.html");
        let expected = include_str!("../tests/hummus.md");
        assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn ragu() {
        let src = include_str!("../tests/ragu.html");
        let expected = include_str!("../tests/ragu.md");
        assert_eq!(get_ld_json(src), expected);
    }
}