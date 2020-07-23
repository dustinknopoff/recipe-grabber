// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use crate::ld_md::LdJson;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NYTCRecipe<'r> {
    #[serde(borrow)]
    pub(crate) name: Cow<'r, str>,
    #[serde(borrow)]
    pub(crate) description: Cow<'r, str>,
    pub(crate) author: Author<'r>,
    #[serde(borrow)]
    pub(crate) image: Cow<'r, str>,
    #[serde(rename = "totalTime")]
    #[serde(borrow)]
    pub(crate) total_time: Option<Cow<'r, str>>,
    #[serde(rename = "recipeYield")]
    #[serde(borrow)]
    pub(crate) recipe_yield: Option<Cow<'r, str>>,
    #[serde(rename = "recipeCuisine")]
    #[serde(borrow)]
    pub(crate) recipe_cuisine: Option<Cow<'r, str>>,
    #[serde(rename = "recipeCategory")]
    #[serde(borrow)]
    pub(crate) recipe_category: Option<Cow<'r, str>>,
    #[serde(rename = "recipeIngredient")]
    #[serde(borrow)]
    pub(crate) recipe_ingredient: Vec<Cow<'r, str>>,
    #[serde(rename = "recipeInstructions")]
    pub(crate) recipe_instructions: Vec<RecipeInstruction<'r>>,
    pub(crate) video: Option<Video<'r>>,
}

impl<'r> NYTCRecipe<'r> {
    fn clean_total_time(&self) -> Option<Cow<'_, str>> {
        self.total_time
            .clone()
            .map(|x| x.replace("PT", "").replace("H", "h ").replace("M", "m"))
            .map(Cow::from)
    }

    fn categories_as_tags(&self) -> Option<String> {
        self.recipe_category.clone().map(|x| {
            x.split(", ")
                .map(|y| format!("#{}", y.replace(" ", "_")))
                .collect::<Vec<_>>()
                .join(" ")
        })
    }
}

impl<'r> LdJson for NYTCRecipe<'r> {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        self.name.to_owned()
    }
    fn description(&self) -> std::borrow::Cow<'_, str> {
        self.description.to_owned()
    }
    fn author(&self) -> std::borrow::Cow<'_, str> {
        self.author.name.to_owned()
    }
    fn image(&self) -> std::borrow::Cow<'_, str> {
        self.image.to_owned()
    }
    fn total_time(&self) -> Option<std::borrow::Cow<'_, str>> {
        self.clean_total_time()
    }
    fn recipe_yield(&self) -> Option<std::borrow::Cow<'_, str>> {
        self.recipe_yield.clone()
    }
    fn cuisine(&self) -> Option<std::borrow::Cow<'_, str>> {
        if let Some(cuisine) = &self.recipe_cuisine {
            Some(Cow::Owned(format!("#{}", cuisine)))
        } else {
            None
        }
    }
    fn category(&self) -> Option<std::borrow::Cow<'_, str>> {
        self.categories_as_tags().map(Cow::Owned)
    }
    fn ingredients(&self) -> Vec<std::borrow::Cow<'_, str>> {
        self.recipe_ingredient.clone()
    }

    fn instructions(&self) -> Vec<Cow<'_, str>> {
        self.recipe_instructions
            .iter()
            .map(|x| x.simplify())
            .collect::<Vec<_>>()
    }

    fn video(&self) -> Option<std::borrow::Cow<'_, str>> {
        if let Some(vid) = &self.video {
            Some(vid.thumbnail_url.to_owned())
        } else {
            None
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AggregateRating {
    #[serde(rename = "@type")]
    pub(crate) aggregate_rating_type: String,
    #[serde(rename = "ratingValue")]
    pub(crate) rating_value: i64,
    #[serde(rename = "ratingCount")]
    pub(crate) rating_count: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Author<'a> {
    #[serde(rename = "@type")]
    #[serde(borrow)]
    pub(crate) author_type: Cow<'a, str>,
    #[serde(borrow)]
    pub(crate) name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Nutrition {
    #[serde(rename = "@context")]
    pub(crate) context: String,
    #[serde(rename = "@type")]
    pub(crate) nutrition_type: String,
    pub(crate) calories: i64,
    #[serde(rename = "unsaturatedFatContent")]
    pub(crate) unsaturated_fat_content: String,
    #[serde(rename = "carbohydrateContent")]
    pub(crate) carbohydrate_content: String,
    #[serde(rename = "cholesterolContent")]
    pub(crate) cholesterol_content: Option<String>,
    #[serde(rename = "fatContent")]
    pub(crate) fat_content: String,
    #[serde(rename = "fiberContent")]
    pub(crate) fiber_content: String,
    #[serde(rename = "proteinContent")]
    pub(crate) protein_content: String,
    #[serde(rename = "saturatedFatContent")]
    pub(crate) saturated_fat_content: String,
    #[serde(rename = "sodiumContent")]
    pub(crate) sodium_content: String,
    #[serde(rename = "sugarContent")]
    pub(crate) sugar_content: String,
    #[serde(rename = "transFatContent")]
    pub(crate) trans_fat_content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RecipeInstruction<'i> {
    #[serde(rename = "@type")]
    #[serde(borrow)]
    pub(crate) recipe_instruction_type: Cow<'i, str>,
    #[serde(borrow)]
    pub(crate) text: Cow<'i, str>,
}

impl<'i> RecipeInstruction<'i> {
    fn simplify(&self) -> Cow<'_, str> {
        self.text.to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Video<'v> {
    #[serde(borrow)]
    pub(crate) name: Cow<'v, str>,
    #[serde(borrow)]
    pub(crate) description: Cow<'v, str>,
    #[serde(rename = "thumbnailUrl")]
    #[serde(borrow)]
    pub(crate) thumbnail_url: Cow<'v, str>,
}

#[cfg(test)]
mod tests {

    use super::NYTCRecipe;
    use crate::ld_md::RecipeMarkdownBuilder;

    #[test]
    fn ragu() {
        let src = include_str!("../../tests/ragu.json");
        let as_recipe: NYTCRecipe<'_> = serde_json::from_str(&src).unwrap();
        let mut builder = RecipeMarkdownBuilder::new(&as_recipe);
        let expected = include_str!("../../tests/ragu.md");
        assert_eq!(builder.build(), expected);
    }
}
