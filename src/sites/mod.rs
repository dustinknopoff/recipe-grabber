use crate::ld_md::LdJson;
use serde::{Deserialize, Serialize};

use std::borrow::Cow;
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum SingleOrArray<T: Clone> {
    Single(T),
    Array(Vec<T>),
}

impl<T: Clone> SingleOrArray<T> {
    fn get(&self) -> T {
        match self {
            SingleOrArray::Single(val) => val.clone(),
            SingleOrArray::Array(val) => val.first().unwrap().to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Image<'r> {
    String(Cow<'r, str>),
    Array(Vec<Cow<'r, str>>),
    Url(ImageObject<'r>),
}

impl<'r> Image<'r> {
    fn get(&self) -> Cow<'r, str> {
        match self {
            Image::String(val) => val.to_owned(),
            Image::Array(val) => val.first().unwrap().to_owned(),
            Image::Url(val) => val.url.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageObject<'i> {
    #[serde(rename = "url")]
    pub(crate) url: Cow<'i, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LdRecipe<'r> {
    #[serde(borrow)]
    pub(crate) name: Cow<'r, str>,
    #[serde(borrow)]
    pub(crate) description: Cow<'r, str>,
    pub(crate) author: SingleOrArray<Author<'r>>,
    #[serde(borrow)]
    pub(crate) image: Image<'r>,
    #[serde(rename = "totalTime")]
    #[serde(borrow)]
    pub(crate) total_time: Option<Cow<'r, str>>,
    #[serde(rename = "recipeYield")]
    #[serde(borrow)]
    pub(crate) recipe_yield: SingleOrArray<Cow<'r, str>>,
    #[serde(rename = "recipeIngredient")]
    #[serde(borrow)]
    pub(crate) recipe_ingredient: Vec<Cow<'r, str>>,
    #[serde(rename = "recipeInstructions")]
    pub(crate) recipe_instructions: StringOrInstruction<'r>,
    pub(crate) video: Option<Video<'r>>,
}

impl<'r> LdRecipe<'r> {
    fn clean_total_time(&self) -> Option<Cow<'_, str>> {
        self.total_time
            .clone()
            .map(|x| x.replace("PT", "").replace("H", "h ").replace("M", "m"))
            .map(Cow::from)
    }
}

impl<'r> LdJson for LdRecipe<'r> {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        self.name.to_owned()
    }
    fn description(&self) -> std::borrow::Cow<'_, str> {
        self.description.to_owned()
    }
    fn author(&self) -> std::borrow::Cow<'_, str> {
        self.author.get().name
    }
    fn image(&self) -> std::borrow::Cow<'_, str> {
        self.image.get()
    }
    fn total_time(&self) -> Option<std::borrow::Cow<'_, str>> {
        self.clean_total_time()
    }
    fn recipe_yield(&self) -> Option<std::borrow::Cow<'_, str>> {
        Some(Cow::Owned(
            self.recipe_yield
                .get()
                .to_owned()
                .replace("Serves : ", "")
                .trim()
                .to_string(),
        ))
    }

    fn ingredients(&self) -> Vec<std::borrow::Cow<'_, str>> {
        self.recipe_ingredient
            .iter()
            .map(|x| x.trim().to_string())
            .map(Cow::from)
            .collect()
    }

    fn instructions(&self) -> Vec<Cow<'_, str>> {
        let val = self.recipe_instructions.get();
        val.into_iter().map(|x| x.simplify()).collect::<Vec<_>>()
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Author<'a> {
    #[serde(rename = "@type")]
    #[serde(borrow)]
    pub(crate) author_type: Cow<'a, str>,
    #[serde(borrow)]
    pub(crate) name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecipeInstruction<'i> {
    #[serde(rename = "@type")]
    #[serde(borrow)]
    pub(crate) recipe_instruction_type: Cow<'i, str>,
    #[serde(borrow)]
    pub(crate) text: Cow<'i, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrInstruction<'r> {
    #[serde(borrow)]
    String(Cow<'r, str>),
    #[serde(borrow)]
    Instruction(Vec<RecipeInstruction<'r>>),
}

impl<'r> StringOrInstruction<'r> {
    fn get(&self) -> Vec<RecipeInstruction<'_>> {
        match self {
            StringOrInstruction::String(val) => val
                .split(". ")
                .map(|v| v.trim())
                .map(|v| RecipeInstruction {
                    recipe_instruction_type: Cow::from("@type"),
                    text: Cow::from(v),
                })
                .collect(),
            StringOrInstruction::Instruction(val) => val.clone(),
        }
    }
}

impl<'i> RecipeInstruction<'i> {
    fn simplify(self) -> Cow<'i, str> {
        self.text
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExRecipe<'r> {
    #[serde(borrow)]
    pub(crate) name: Cow<'r, str>,
    #[serde(borrow)]
    pub(crate) description: Cow<'r, str>,
    pub(crate) author: Author<'r>,
    // #[serde(borrow)]
    // pub(crate) image: Image<'r>,
    #[serde(rename = "totalTime")]
    #[serde(borrow)]
    pub(crate) total_time: Option<Cow<'r, str>>,
    // #[serde(rename = "recipeYield")]
    // #[serde(borrow)]
    // pub(crate) recipe_yield: StringOrArray<'r>,
    #[serde(rename = "recipeIngredient")]
    #[serde(borrow)]
    pub(crate) recipe_ingredient: Vec<Cow<'r, str>>,
    #[serde(rename = "recipeInstructions")]
    pub(crate) recipe_instructions: StringOrInstruction<'r>,
    pub(crate) video: Option<Video<'r>>,
}
