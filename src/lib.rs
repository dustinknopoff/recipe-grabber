mod utils;
mod ld_types;
use cfg_if::cfg_if;
use scraper::{Html, Selector};
use wasm_bindgen::prelude::*;
use ld_types::Recipe;

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
    let mut as_recipe: Recipe = serde_json::from_str(&as_txt).unwrap();
    as_recipe.as_md()
    // as_txt
}
