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
#[serde(untagged)]
pub enum FWEnum<'r> {
    #[serde(borrow)]
    Recipe(FoodAndWineRecipe<'r>),
    Breadcrumb(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FoodAndWineLd<'l>(#[serde(borrow)] Vec<FWEnum<'l>>);

impl<'l> FoodAndWineLd<'l> {
    fn get_recipe(&self) -> Option<&FoodAndWineRecipe<'_>> {
        for x in self.0.iter() {
            if let FWEnum::Recipe(r) = &x {
                return Some(r);
            }
        }
        None
    }
}

impl<'r> LdJson for FoodAndWineLd<'r> {
    fn name(&self) -> Cow<'_, str> {
        self.get_recipe().unwrap().name()
    }
    fn description(&self) -> Cow<'_, str> {
        self.get_recipe().unwrap().description()
    }
    fn author(&self) -> Cow<'_, str> {
        self.get_recipe().unwrap().author()
    }
    fn image(&self) -> Cow<'_, str> {
        self.get_recipe().unwrap().image()
    }
    fn total_time(&self) -> Option<Cow<'_, str>> {
        self.get_recipe().unwrap().total_time()
    }
    fn recipe_yield(&self) -> Option<Cow<'_, str>> {
        self.get_recipe().unwrap().recipe_yield()
    }
    fn cuisine(&self) -> Option<Cow<'_, str>> {
        self.get_recipe().unwrap().cuisine()
    }
    fn category(&self) -> Option<Cow<'_, str>> {
        self.get_recipe().unwrap().category()
    }
    fn ingredients(&self) -> Vec<Cow<'_, str>> {
        self.get_recipe().unwrap().ingredients()
    }
    fn instructions(&self) -> Vec<Cow<'_, str>> {
        self.get_recipe().unwrap().instructions()
    }
    fn video(&self) -> Option<Cow<'_, str>> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FoodAndWineRecipe<'r> {
    #[serde(rename = "mainEntityOfPage")]
    #[serde(borrow)]
    pub(crate) main_entity_of_page: Cow<'r, str>,

    #[serde(rename = "name")]
    #[serde(borrow)]
    pub(crate) name: Cow<'r, str>,

    #[serde(rename = "image")]
    pub(crate) image: Image<'r>,

    #[serde(rename = "datePublished")]
    #[serde(borrow)]
    pub(crate) date_published: Cow<'r, str>,

    #[serde(rename = "description")]
    #[serde(borrow)]
    pub(crate) description: Cow<'r, str>,

    #[serde(rename = "totalTime")]
    #[serde(borrow)]
    pub(crate) total_time: Cow<'r, str>,

    #[serde(rename = "recipeYield")]
    #[serde(borrow)]
    pub(crate) recipe_yield: Cow<'r, str>,

    #[serde(rename = "recipeIngredient")]
    #[serde(borrow)]
    pub(crate) recipe_ingredient: Vec<Cow<'r, str>>,

    #[serde(rename = "recipeInstructions")]
    #[serde(borrow)]
    pub(crate) recipe_instructions: Cow<'r, str>,

    #[serde(rename = "recipeCategory")]
    #[serde(borrow)]
    pub(crate) recipe_category: Vec<Cow<'r, str>>,

    #[serde(rename = "recipeCuisine")]
    #[serde(borrow)]
    pub(crate) recipe_cuisine: Vec<Cow<'r, str>>,

    #[serde(rename = "author")]
    pub(crate) author: Vec<Author<'r>>,
}

impl<'r> FoodAndWineRecipe<'r> {
    fn clean_total_time(&self) -> String {
        self.total_time
            .to_owned()
            .replace("PT", "")
            .replace("H", "h ")
            .replace("M", "m")
    }

    fn categories_as_tags(&self) -> String {
        self.recipe_category
            .iter()
            .map(|x| format!("#{}", x.replace(" ", "_")))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn cuisines_as_tags(&self) -> String {
        self.recipe_cuisine
            .iter()
            .map(|x| format!("#{}", x.replace(" ", "_")))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<'r> LdJson for FoodAndWineRecipe<'r> {
    fn name(&self) -> Cow<'_, str> {
        self.name.to_owned()
    }
    fn description(&self) -> Cow<'_, str> {
        Cow::Owned(self.description.to_owned().replace("\r", ""))
    }
    fn author(&self) -> Cow<'_, str> {
        Cow::Owned(
            self.author
                .iter()
                .map(|x| x.name.to_owned())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
    fn image(&self) -> Cow<'_, str> {
        self.image.url.to_owned()
    }
    fn total_time(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Owned(self.clean_total_time()))
    }
    fn recipe_yield(&self) -> Option<Cow<'_, str>> {
        Some(Cow::Owned(
            self.recipe_yield
                .to_owned()
                .replace("Serves : ", "")
                .trim()
                .to_string(),
        ))
    }
    fn cuisine(&self) -> Option<Cow<'_, str>> {
        if self.recipe_cuisine.is_empty() {
            None
        } else {
            Some(Cow::Owned(self.cuisines_as_tags()))
        }
    }
    fn category(&self) -> Option<Cow<'_, str>> {
        if self.recipe_category.is_empty() {
            None
        } else {
            Some(Cow::Owned(self.categories_as_tags()))
        }
    }
    fn ingredients(&self) -> Vec<Cow<'_, str>> {
        self.recipe_ingredient
            .iter()
            .map(|x| x.trim().to_string())
            .map(Cow::from)
            .collect()
    }
    fn instructions(&self) -> Vec<Cow<'_, str>> {
        self.recipe_instructions
            .split('.')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| Cow::Owned(x.trim().to_string()))
            .collect()
    }
    fn video(&self) -> Option<Cow<'_, str>> {
        None
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Author<'a> {
    #[serde(rename = "name")]
    #[serde(borrow)]
    pub(crate) name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Image<'i> {
    #[serde(rename = "url")]
    pub(crate) url: Cow<'i, str>,
}

#[cfg(test)]
mod tests {

    use super::{FoodAndWineLd, FoodAndWineRecipe};
    use crate::ld_md::RecipeMarkdownBuilder;

    #[test]
    fn hummus() {
        let src = include_str!("../../tests/hummus.json");
        let as_recipe: FoodAndWineRecipe = serde_json::from_str(&src).unwrap();
        let mut builder = RecipeMarkdownBuilder::new(&as_recipe);
        let expected = include_str!("../../tests/hummus.md");
        assert_eq!(builder.build(), expected);
    }

    #[test]
    fn full_hummus() {
        let src = include_str!("../../tests/full_hummus.json");
        let as_recipe: FoodAndWineLd = serde_json::from_str(&src).unwrap();
        let mut builder = RecipeMarkdownBuilder::new(&as_recipe);
        let expected = include_str!("../../tests/hummus.md");
        assert_eq!(builder.build(), expected);
    }
}
