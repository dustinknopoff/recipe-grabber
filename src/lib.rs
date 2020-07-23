#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![warn(clippy::all)]
//! # Recipe Grabber
//!
//! ![build-and-check](https://github.com/dustinknopoff/nytcooking-grabber/workflows/build-and-check/badge.svg)
//!
//! Deployed to [Cloudflare](https://nytcooking-grabber.knopoff.workers.dev)
//!
//! Pass `/?url=<url>` to produce a markdown representation
//!
//! ## Currently supported sites:
//!
//! - [NYTimes Cooking](https://cooking.nytimes.com)
//! - [Food and Wine](https://foodandwine.com)
//!

mod ld_md;
mod sites;
mod utils;
use crate::sites::Sites;
use cfg_if::cfg_if;
use ld_md::RecipeMarkdownBuilder;
use scraper::{Html, Selector};
use wasm_bindgen::prelude::*;

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
/// Given the contents of a website, The `application/ld+json` attribute is extracted,
/// parsed, and converted in to a markdown document.
pub fn get_ld_json(contents: &str) -> String {
    let document = Html::parse_document(contents);
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
    let ctx = document.select(&selector).next().unwrap();
    let text = ctx.text().collect::<Vec<_>>();
    let as_txt = text.join("");
    let as_recipe: Sites<'_> = serde_json::from_str(&as_txt).unwrap();
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
