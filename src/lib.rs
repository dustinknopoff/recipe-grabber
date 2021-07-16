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

pub(crate) mod duration;
pub(crate) mod ld_md;
pub(crate) mod sites;
pub(crate) mod utils;
use cfg_if::cfg_if;
use ld_md::RecipeMarkdownBuilder;
use scraper::{Html, Selector};
use serde_json::Value;
use sites::LdRecipe;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;
    }
}

macro_rules! res_unwrap {
    ($val: expr) => {
        match $val {
            Ok(val) => val,
            Err(e) => {
                dbg!(e);
                return String::from(
                    "Whoops! Something went wrong. This worker does not support that url :(.",
                );
            }
        }
    };
}

// macro_rules! opt_unwrap {
//     ($val: expr) => {
//         match $val {
//             Some(val) => val,
//             None => {
//                 return String::from(
//                     "Whoops! Something went wrong. This worker does not support that url :(.",
//                 )
//             }
//         }
//     };
// }

#[wasm_bindgen]
/// Given the contents of a website, The `application/ld+json` attribute is extracted,
/// parsed, and converted in to a markdown document.
pub fn get_ld_json(contents: &str) -> String {
    let document = Html::parse_document(contents);
    let selector = res_unwrap! { Selector::parse(r#"script[type="application/ld+json"]"#) };
    let ctx: Vec<String> = document
        .select(&selector)
        .map(|ctx| {
            let text = ctx.text().collect::<Vec<_>>();
            text.join("")
        })
        .collect();
    let as_txt = traverse_for_type_recipe(&ctx);
    let as_recipe: LdRecipe<'_> = res_unwrap! { serde_json::from_str(&as_txt) };
    let mut builder = RecipeMarkdownBuilder::new(&as_recipe);
    let markdown: String = builder.build().into();
    markdown.replace("\r\n", "\n")
}

fn traverse_for_type_recipe(ld_jsons: &[String]) -> String {
    let _recipe_str = serde_json::json!("Recipe");
    // Example: tests/ragu.json
    for content in ld_jsons {
        let tree: serde_json::Value = serde_json::from_str(content).unwrap();
        if let Some(val) = tree.get("@type") {
            if val == &_recipe_str {
                return content.to_string();
            }
        }
        // Example: tests/chocolate_olive_oil.json
        let val: &Value = if let Some(val) = tree.get("@graph") {
            val
        } else if tree.is_array() {
            &tree
        } else {
            continue;
        };
        return val
            .as_array()
            .unwrap()
            .iter()
            .filter(|graph_item| graph_item.get("@type") == Some(&_recipe_str))
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .to_string();
    }
    panic!("Invalid recipe!")
}

#[cfg(test)]
mod tests {
    use crate::get_ld_json;

    #[macro_export]
    /// Same semantics as [`assert_eq`] with one major distinction.
    /// This only works on `AsRef<str>` and uses the [dissimilar](https://crates.io/dissimilar)
    /// lib to produce the output as chunks of Equal, Insert, Delete
    macro_rules! str_assert_eq {
        ($left:expr, $right:expr $(,)?) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        panic!("{:#?}", dissimilar::diff(&*left_val, &*right_val))
                    }
                }
            }
        }};
        ($left:expr, $right:expr, $($arg:tt)+) => {{
            match (&($left), &($right)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        panic!("{:#?}", dissimilar::diff(&*left_val, &*right_val))
                    }
                }
            }
        }};
    }

    #[test]
    fn hummus() {
        let src = include_str!("../tests/hummus.html");
        let expected = include_str!("../tests/hummus.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn ragu() {
        let src = include_str!("../tests/ragu.html");
        let expected = include_str!("../tests/ragu.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn chocolate_olive_oil() {
        let src = include_str!("../tests/chocolate_olive_oil.html");
        let expected = include_str!("../tests/chocolate_olive_oil.md");
        let actual = get_ld_json(src);
        str_assert_eq!(actual, expected);
    }

    #[test]
    fn meringue() {
        let src = include_str!("../tests/chocolate-hazelnut-meringue.html");
        let expected = include_str!("../tests/meringue.md");
        let actual = get_ld_json(src);
        str_assert_eq!(actual, expected);
    }

    #[test]
    fn eggplant() {
        let src = include_str!("../tests/eggplant-pizza.html");
        let expected = include_str!("../tests/eggplant-pizza.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn tenders() {
        let src = include_str!("../tests/bacon-wrapped-chicken-tenders.html");
        let expected = include_str!("../tests/tenders.md");
        str_assert_eq!(get_ld_json(src), expected);
    }
}
