//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use recipe_grabber::get_ld_json;

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

wasm_bindgen_test_configure!(run_in_browser);

 #[wasm_bindgen_test]
    fn hummus() {
        let src = include_str!("./hummus.html");
        let expected = include_str!("./hummus.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[wasm_bindgen_test]
    fn ragu() {
        let src = include_str!("./ragu.html");
        let expected = include_str!("./ragu.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[wasm_bindgen_test]
    fn chocolate_olive_oil() {
        let src = include_str!("./chocolate_olive_oil.html");
        let expected = include_str!("./chocolate_olive_oil.md");
        let actual = get_ld_json(src);
        str_assert_eq!(actual, expected);
    }

    #[wasm_bindgen_test]
    fn meringue() {
        let src = include_str!("./chocolate-hazelnut-meringue.html");
        let expected = include_str!("./meringue.md");
        let actual = get_ld_json(src);
        str_assert_eq!(actual, expected);
    }

    #[wasm_bindgen_test]
    fn eggplant() {
        let src = include_str!("./eggplant-pizza.html");
        let expected = include_str!("./eggplant-pizza.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[wasm_bindgen_test]
    fn tenders() {
        let src = include_str!("./bacon-wrapped-chicken-tenders.html");
        let expected = include_str!("./tenders.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[wasm_bindgen_test]
    fn biscotti() {
        let src = include_str!("./biscotti.html");
        let expected = include_str!("./biscotti.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[wasm_bindgen_test]
    fn ottolenghi() {
        let src = include_str!("./ottolenghi.html");
        let expected = include_str!("./ottolenghi.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[wasm_bindgen_test]
    fn wavecake() {
        let src = include_str!("./wave-cake.html");
        let expected = include_str!("./wave-cake.md");
        str_assert_eq!(get_ld_json(src), expected);
    }

    #[wasm_bindgen_test]
    fn will_fail() {
        let src = include_str!("./will_fail.html");
        let actual = get_ld_json(src);
        let expected = r#"<p>Whoops! Something went wrong. This worker does not support that url :(.</p>
<hr />
<p>Technical Readout:</p>
<pre>Site contains no Recipe Schema.</pre>"#;
        assert_eq!(actual, expected);
    }

    #[wasm_bindgen_test]
    fn will_fail_2() {
        let src = include_str!("./will_fail_2.html");
        let actual = get_ld_json(src);
        let expected = r#"<p>Whoops! Something went wrong. This worker does not support that url :(.</p>
<hr />
<p>Technical Readout:</p>
<pre>Recipe not found in ld+json
{"@context":"https://schema.org","@graph":[{"@type":"Organization","@id":"https://butternutbakeryblog.com/#organization","name":"Butternut Bakery","url":"https://butternutbakeryblog.com/","sameAs":["https://www.facebook.com/butternutbakeryblog","https://www.instagram.com/butternutbakery/","https://www.pinterest.com/butternutbakery/"],"logo":{"@type":"ImageObject","@id":"https://butternutbakeryblog.com/#logo","inLanguage":"en-US","url":"https://butternutbakeryblog.com/wp-content/uploads/2018/05/Untitled-5-1.jpg","width":640,"height":340,"caption":"Butternut Bakery"},"image":{"@id":"https://butternutbakeryblog.com/#logo"}},{"@type":"WebSite","@id":"https://butternutbakeryblog.com/#website","url":"https://butternutbakeryblog.com/","name":"Butternut Bakery","description":"A Baking Blog Sharing Indulgent Recipes","publisher":{"@id":"https://butternutbakeryblog.com/#organization"},"potentialAction":[{"@type":"SearchAction","target":"https://butternutbakeryblog.com/?s={search_term_string}","query-input":"required name=search_term_string"}],"inLanguage":"en-US"},{"@type":"ImageObject","@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#primaryimage","inLanguage":"en-US","url":"https://butternutbakeryblog.com/wp-content/uploads/2020/04/flourless-chocolate-cake.jpg","width":1200,"height":1800,"caption":"flourless chocolate cake sliced on parchment paper"},{"@type":"WebPage","@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#webpage","url":"https://butternutbakeryblog.com/flourless-chocolate-cake/","name":"Flourless Olive Oil Chocolate Cake | Butternut Bakery","isPartOf":{"@id":"https://butternutbakeryblog.com/#website"},"primaryImageOfPage":{"@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#primaryimage"},"datePublished":"2020-04-27T03:41:08+00:00","dateModified":"2020-04-27T03:41:10+00:00","inLanguage":"en-US","potentialAction":[{"@type":"ReadAction","target":["https://butternutbakeryblog.com/flourless-chocolate-cake/"]}]},{"@type":"Article","@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#article","isPartOf":{"@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#webpage"},"author":{"@id":"https://butternutbakeryblog.com/#/schema/person/bc97cdfa6d8ac72d58a912b59a782147"},"headline":"Flourless Olive Oil Chocolate Cake","datePublished":"2020-04-27T03:41:08+00:00","dateModified":"2020-04-27T03:41:10+00:00","commentCount":"6","publisher":{"@id":"https://butternutbakeryblog.com/#organization"},"image":{"@id":"https://butternutbakeryblog.com/flourless-chocolate-cake/#primaryimage"},"articleSection":"Cakes and Cupcakes,Dairy Free or Vegan,Gluten Free","inLanguage":"en-US","potentialAction":[{"@type":"CommentAction","name":"Comment","target":["https://butternutbakeryblog.com/flourless-chocolate-cake/#respond"]}]},{"@type":["Person"],"@id":"https://butternutbakeryblog.com/#/schema/person/bc97cdfa6d8ac72d58a912b59a782147","name":"Jenna","image":{"@type":"ImageObject","@id":"https://butternutbakeryblog.com/#personlogo","inLanguage":"en-US","url":"https://secure.gravatar.com/avatar/895fe079b97e47c9f1618c05ad517df6?s=96&d=mm&r=g","caption":"Jenna"}}]}</pre>"#;
        str_assert_eq!(actual, expected);
    }