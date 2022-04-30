#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![warn(clippy::all)]
//! # Recipe Grabber
//!
//! ![build-and-check](https://github.com/dustinknopoff/recipe-grabber/workflows/build-and-check/badge.svg)
//!
//! Deployed to [Cloudflare](https://recipe-grabber.knopoff.workers.dev)
//!
//! Pass `/?url=<url>` to produce a markdown representation
//!
//! ## Currently supported sites:
//!
//! - [NYTimes Cooking](https://cooking.nytimes.com)
//! - [Food and Wine](https://foodandwine.com) (Some links)
//! - [Food52](https://food52.com)
//! - [AllRecipes](https://allrecipes.com)
//! - [BBC Good Food](https://www.bbcgoodfood.com)
//! - [Simply So Healthy](https://simplysohealthy.com)
//!
//! Most sites _should_ work
//!

pub(crate) mod duration;
pub(crate) mod ld_md;
pub(crate) mod sites;
pub(crate) mod utils;
use anyhow::bail;
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

use thiserror::Error;

/// Catch All error for bad libraries
#[derive(Error, Debug)]
pub enum RecipeError {
    /// Unknown error, provide a `context()`
    #[error("unknown error")]
    Unknown,
}

#[wasm_bindgen]
/// Wrapper around [`_get_ld_json`](crate::_get_ld_json) which HTML-ifies the result of extracting a recipe
///
/// Makes interop with wasm easier
pub fn get_ld_json(contents: &str) -> String {
    match _get_ld_json(contents) {
        Ok(val) => val,
        Err(e) => {
            dbg!(&e);
            format!(
                r#"<p>Whoops! Something went wrong. This worker does not support that url :(.</p>
<hr />
<p>Technical Readout:</p>
<pre>{}</pre>"#,
                e
            )
        }
    }
}

/// Given the contents of a website, The `application/ld+json` attribute is extracted,
/// parsed, and converted in to a markdown document.
pub fn _get_ld_json(contents: &str) -> anyhow::Result<String> {
    let document = Html::parse_document(contents);
    let selector = match Selector::parse(r#"script[type="application/ld+json"]"#) {
        Ok(val) => val,
        Err(_) => bail!("Could not parse XPath Selector"),
    };
    let ctx: Vec<String> = document
        .select(&selector)
        .map(|ctx| {
            let text = ctx.text().collect::<Vec<_>>();
            text.join("")
        })
        .collect();
    let as_txt = traverse_for_type_recipe(&ctx)?;
    let as_recipe: LdRecipe<'_> = serde_json::from_str(&as_txt)?;
    let mut builder = RecipeMarkdownBuilder::new(&as_recipe);
    let markdown: String = builder.build().into();
    Ok(markdown.replace("\r\n", "\n"))
}

fn traverse_for_type_recipe(ld_jsons: &[String]) -> anyhow::Result<String> {
    let _recipe_str = serde_json::json!("Recipe");
    // Example: tests/ragu.json
    for content in ld_jsons {
        let tree: serde_json::Value = serde_json::from_str(content)?;
        if let Some(val) = tree.get("@type") {
            if val == &_recipe_str {
                return Ok(content.to_string());
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
        let recipe_in_graph = val
            .as_array()
            .unwrap()
            .iter()
            .filter(|graph_item| graph_item.get("@type") == Some(&_recipe_str))
            .collect::<Vec<_>>();
        let recipe_in_graph = recipe_in_graph.first();
        if let Some(recipe) = recipe_in_graph {
            return Ok(recipe.to_string());
        } else {
            anyhow::bail!("Recipe not found in ld+json\n{}", ld_jsons.join("\n"))
        }
    }
    let context = ld_jsons.join("\n");
    if context.is_empty() {
        anyhow::bail!("Site contains no Recipe Schema.")
    } else {
        anyhow::bail!("Recipe not found in ld+json\n{}", context)
    }
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

    #[test]
    fn biscotti() {
        let src = include_str!("../tests/biscotti.html");
        let expected = include_str!("../tests/biscotti.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn ottolenghi() {
        let src = include_str!("../tests/ottolenghi.html");
        let expected = include_str!("../tests/ottolenghi.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn wavecake() {
        let src = include_str!("../tests/wave-cake.html");
        let expected = include_str!("../tests/wave-cake.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[test]
    fn brisket() {
        let src = include_str!("../tests/brisket.html");
        let expected = include_str!("../tests/brisket.md");
        str_assert_eq!(dbg!(get_ld_json(src)), expected);
    }

    #[test]
    fn will_fail() {
        let src = include_str!("../tests/will_fail.html");
        let actual = get_ld_json(src);
        let expected = r#"<p>Whoops! Something went wrong. This worker does not support that url :(.</p>
<hr />
<p>Technical Readout:</p>
<pre>Site contains no Recipe Schema.</pre>"#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn will_fail_2() {
        let src = include_str!("../tests/will_fail_2.html");
        let actual = get_ld_json(src);
        let expected = r#"<p>Whoops! Something went wrong. This worker does not support that url :(.</p>
<hr />
<p>Technical Readout:</p>
<pre>Recipe not found in ld+json
{"@context":"https://schema.org","@graph":[{"@type":"Organization","@id":"https://butternutbakeryblog.com/#organization","name":"Butternut Bakery","url":"https://butternutbakeryblog.com/","sameAs":["https://www.facebook.com/butternutbakeryblog","https://www.instagram.com/butternutbakery/","https://www.pinterest.com/butternutbakery/"],"logo":{"@type":"ImageObject","@id":"https://butternutbakeryblog.com/#logo","inLanguage":"en-US","url":"https://butternutbakeryblog.com/wp-content/uploads/2018/05/Untitled-5-1.jpg","width":640,"height":340,"caption":"Butternut Bakery"},"image":{"@id":"https://butternutbakeryblog.com/#logo"}},{"@type":"WebSite","@id":"https://butternutbakeryblog.com/#website","url":"https://butternutbakeryblog.com/","name":"Butternut Bakery","description":"A Baking Blog Sharing Indulgent Recipes","publisher":{"@id":"https://butternutbakeryblog.com/#organization"},"potentialAction":[{"@type":"SearchAction","target":"https://butternutbakeryblog.com/?s={search_term_string}","query-input":"required name=search_term_string"}],"inLanguage":"en-US"},{"@type":"ImageObject","@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#primaryimage","inLanguage":"en-US","url":"https://butternutbakeryblog.com/wp-content/uploads/2020/04/flourless-chocolate-cake.jpg","width":1200,"height":1800,"caption":"flourless chocolate cake sliced on parchment paper"},{"@type":"WebPage","@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#webpage","url":"https://butternutbakeryblog.com/flourless-chocolate-cake/","name":"Flourless Olive Oil Chocolate Cake | Butternut Bakery","isPartOf":{"@id":"https://butternutbakeryblog.com/#website"},"primaryImageOfPage":{"@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#primaryimage"},"datePublished":"2020-04-27T03:41:08+00:00","dateModified":"2020-04-27T03:41:10+00:00","inLanguage":"en-US","potentialAction":[{"@type":"ReadAction","target":["https://butternutbakeryblog.com/flourless-chocolate-cake/"]}]},{"@type":"Article","@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#article","isPartOf":{"@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#webpage"},"author":{"@id":"https://butternutbakeryblog.com/#/schema/person/bc97cdfa6d8ac72d58a912b59a782147"},"headline":"Flourless Olive Oil Chocolate Cake","datePublished":"2020-04-27T03:41:08+00:00","dateModified":"2020-04-27T03:41:10+00:00","commentCount":"6","publisher":{"@id":"https://butternutbakeryblog.com/#organization"},"image":{"@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#primaryimage"},"articleSection":"Cakes and Cupcakes,Dairy Free or Vegan,Gluten Free","inLanguage":"en-US","potentialAction":[{"@type":"CommentAction","name":"Comment","target":["https://butternutbakeryblog.com/flourless-chocolate-cake/#respond"]}]},{"@type":["Person"],"@id":"https://butternutbakeryblog.com/#/schema/person/bc97cdfa6d8ac72d58a912b59a782147","name":"Jenna","image":{"@type":"ImageObject","@id":"https://butternutbakeryblog.com/#personlogo","inLanguage":"en-US","url":"https://secure.gravatar.com/avatar/895fe079b97e47c9f1618c05ad517df6?s=96&d=mm&r=g","caption":"Jenna"}}]}</pre>"#;
        str_assert_eq!(actual, expected);
    }
}
