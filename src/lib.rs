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
use std::borrow::Cow;

use cfg_if::cfg_if;
use ld_md::RecipeMarkdownBuilder;
use scraper::{Html, Selector};
use serde_json::Value;
use sites::{LdRecipe, LdRecipe2};
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
    builder.build().into()
}

// Can we do this as a combinator?
// I.e. in `traverse_for_type_recipe`
// sub-function utilizing `Value`
// get(val, index) -> Result<Value, Value>

//get(@type).or(get("@graph"))

// 1. Find recipe
// 2. get name
// 3. description (optional)
// 4. image
// 5. total time
// 6.

fn construct_ld_recipe<'c>(ld_json: Value) -> LdRecipe2<'c> {
    use crate::sites::LdRecipe2Builder;
    let name = Cow::from(get(&ld_json, "name").to_string());
    let description = ld_json
        .get("description")
        .map(|x| x.to_string())
        .map(Cow::from);
    let author = Cow::from(get(get_val_or_first(&ld_json, "author"), "name").to_string());
    let image = Cow::from({
        let image_key = get_val_or_first(&ld_json, "image");
        match image_key.as_str() {
            Some(val) => val.to_string(),
            None => image_key
                .get("url")
                .expect("image objects have a url!")
                .to_string(),
        }
        // TODO: Sort for largest image to be returned
    });
    let total_time = ld_json
        .get("totalTime")
        .map(|x| x.to_string())
        .map(Cow::from);
    let recipe_yield = Cow::from({
        let yield_key = get_val_or_first(&ld_json, "recipeYield");
        match yield_key.as_str() {
            Some(val) => val.to_string(),
            None => yield_key.as_i64().unwrap().to_string(),
        }
    });
    let recipe_ingredient: Vec<Cow<'c, str>> = get(&ld_json, "recipeIngredient")
        .as_array()
        .expect("Ingredients list is always an array")
        .iter()
        .map(|x| x.to_string())
        .map(Cow::from)
        .collect();
    let recipe_instructions: Vec<Cow<'c, str>> = {
        let mut ret_val = Vec::with_capacity(0);
        let instructions_key = get(&ld_json, "recipeInstructions");
        if instructions_key.is_string() {
            ret_val = vec![instructions_key
                .to_string()
                .split('\n')
                .map(|x| x.to_string())
                .map(Cow::from)
                .collect()];
        }
        ret_val = match instructions_key.as_array() {
            Some(instructions) => instructions
                .iter()
                .map(|val| {
                    if val.is_string() {
                        return vec![Cow::from(val.to_string())];
                    }
                    match val.get("instructions") {
                        Some(val) => val
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|x| x.to_string())
                            .map(Cow::from)
                            .collect(),
                        None => vec![Cow::from(val.get("text").unwrap().to_string())],
                    }
                })
                .collect(),
            None => panic!("Instructions are neither a string or a list"),
        };
        ret_val.into_iter().flatten().collect()
    };
    let video = ld_json
        .get("video")
        .map(|x| x.get("thumbnailUrl").unwrap().to_string())
        .map(Cow::from);
    LdRecipe2Builder::default()
        .name(name)
        .description(description)
        .author(author)
        .image(image)
        .total_time(total_time)
        .recipe_yield(recipe_yield)
        .recipe_instructions(recipe_instructions)
        .recipe_ingredient(recipe_ingredient)
        .video(video)
        .build()
        .unwrap()
}

fn get_val_or_first<'l>(ld_json: &'l Value, index: &'l str) -> &'l Value {
    let maybe_vec = get(ld_json, index);
    match maybe_vec.as_array() {
        Some(vec) => vec.first().expect("Array must have at least one object"),
        None => &maybe_vec,
    }
}

fn get<'l>(ld_json: &'l Value, index: &'l str) -> &'l Value {
    ld_json
        .get(index)
        .unwrap_or_else(|| panic!("All recipes have have a {}!", index))
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

    #[test]
    fn chocolate_olive_oil() {
        let src = include_str!("../tests/chocolate_olive_oil.html");
        let expected = include_str!("../tests/chocolate_olive_oil.md");
        assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn meringue() {
        let src = include_str!("../tests/chocolate-hazelnut-meringue.html");
        let expected = include_str!("../tests/meringue.md");
        assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn eggplant() {
        let src = include_str!("../tests/eggplant-pizza.html");
        let expected = include_str!("../tests/eggplant-pizza.md");
        assert_eq!(get_ld_json(src), expected);
    }
}

#[cfg(test)]
mod logic {
    use super::construct_ld_recipe;
    #[test]
    fn construct() {
        let src = include_str!("../tests/eggplant-pizza.json");
        let src = serde_json::from_str(src).unwrap();
        dbg!(construct_ld_recipe(src));
    }
}
