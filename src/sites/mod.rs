use crate::ld_md::LdJson;
use crate::sites::faw_types::FoodAndWineLd;
use crate::sites::nytc_types::NYTCRecipe;
use serde::{Deserialize, Serialize};
use wasm_bindgen::__rt::std::borrow::Cow;

pub(crate) mod faw_types;
pub(crate) mod nytc_types;

// Justification: Lives for a very short amount of time
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Sites<'s> {
    #[serde(borrow)]
    FoodAndWine(FoodAndWineLd<'s>),
    #[serde(borrow)]
    NYTCooking(NYTCRecipe<'s>),
}

impl<'s> LdJson for Sites<'s> {
    fn name(&self) -> Cow<'_, str> {
        match self {
            Self::FoodAndWine(recipe) => recipe.name(),
            Self::NYTCooking(recipe) => recipe.name(),
        }
    }

    fn description(&self) -> Cow<'_, str> {
        match self {
            Self::FoodAndWine(recipe) => recipe.description(),
            Self::NYTCooking(recipe) => recipe.description(),
        }
    }

    fn author(&self) -> Cow<'_, str> {
        match self {
            Self::FoodAndWine(recipe) => recipe.author(),
            Self::NYTCooking(recipe) => recipe.author(),
        }
    }

    fn image(&self) -> Cow<'_, str> {
        match self {
            Self::FoodAndWine(recipe) => recipe.image(),
            Self::NYTCooking(recipe) => recipe.image(),
        }
    }

    fn total_time(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::FoodAndWine(recipe) => recipe.total_time(),
            Self::NYTCooking(recipe) => recipe.total_time(),
        }
    }

    fn recipe_yield(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::FoodAndWine(recipe) => recipe.recipe_yield(),
            Self::NYTCooking(recipe) => recipe.recipe_yield(),
        }
    }

    fn cuisine(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::FoodAndWine(recipe) => recipe.cuisine(),
            Self::NYTCooking(recipe) => recipe.cuisine(),
        }
    }

    fn category(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::FoodAndWine(recipe) => recipe.category(),
            Self::NYTCooking(recipe) => recipe.category(),
        }
    }

    fn ingredients(&self) -> Vec<Cow<'_, str>> {
        match self {
            Self::FoodAndWine(recipe) => recipe.ingredients(),
            Self::NYTCooking(recipe) => recipe.ingredients(),
        }
    }

    fn instructions(&self) -> Vec<Cow<'_, str>> {
        match self {
            Self::FoodAndWine(recipe) => recipe.instructions(),
            Self::NYTCooking(recipe) => recipe.instructions(),
        }
    }

    fn video(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::FoodAndWine(recipe) => recipe.video(),
            Self::NYTCooking(recipe) => recipe.video(),
        }
    }
}
