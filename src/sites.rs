use crate::duration::Duration;
use crate::ld_md::LdJson;
pub use media::*;
use serde::{Deserialize, Serialize};
pub use sub_objects::*;

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

pub mod media {
    use super::*;
    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(untagged)]
    pub enum Image<'r> {
        String(Cow<'r, str>),
        Array(Vec<Cow<'r, str>>),
        Url(ImageObject<'r>),
    }

    impl<'r> Image<'r> {
        pub fn get(&self) -> Cow<'r, str> {
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LdRecipe<'r> {
    #[serde(borrow)]
    pub(crate) name: Cow<'r, str>,
    #[serde(borrow)]
    pub(crate) description: Cow<'r, str>,
    pub(crate) author: SingleOrArray<Author<'r>>,
    #[serde(borrow)]
    pub(crate) image: Image<'r>,
    #[serde(borrow)]
    pub(crate) total_time: Option<Cow<'r, str>>,
    #[serde(borrow)]
    pub(crate) recipe_yield: SingleOrArray<Cow<'r, str>>,
    #[serde(borrow)]
    pub(crate) recipe_ingredient: Vec<Cow<'r, str>>,
    pub(crate) recipe_instructions: RecipeInstructionKinds<'r>,
    pub(crate) video: Option<Video<'r>>,
}

impl<'r> LdRecipe<'r> {
    fn clean_total_time(&self) -> Option<Cow<'_, str>> {
        self.total_time
            .clone()
            .map(|x| Duration::parse(x.as_ref()).unwrap())
            .map(|x| x.to_string())
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

    fn instructions(&self) -> RecipeInstructionKinds<'_> {
        self.recipe_instructions.get()
    }

    fn video(&self) -> Option<std::borrow::Cow<'_, str>> {
        if let Some(vid) = &self.video {
            Some(vid.thumbnail_url.to_owned())
        } else {
            None
        }
    }
}

pub mod sub_objects {
    use super::*;
    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct Author<'a> {
        #[serde(borrow)]
        pub(crate) name: Cow<'a, str>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct RecipeInstruction<'i> {
        #[serde(borrow)]
        pub text: Cow<'i, str>,
    }

    impl<'i> RecipeInstruction<'i> {
        pub fn simplify(self) -> Cow<'i, str> {
            self.text
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    pub struct HowToSection<'i> {
        #[serde(borrow)]
        pub(crate) name: Cow<'i, str>,
        #[serde(rename = "itemListElement")]
        pub(crate) instructions: Vec<RecipeInstruction<'i>>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    #[serde(untagged)]
    pub enum RecipeInstructionKinds<'r> {
        #[serde(borrow)]
        String(Cow<'r, str>),
        #[serde(borrow)]
        Instruction(Vec<RecipeInstruction<'r>>),
        #[serde(borrow)]
        Sectioned(Vec<HowToSection<'r>>),
    }

    impl<'r> RecipeInstructionKinds<'r> {
        pub fn get(&self) -> RecipeInstructionKinds<'_> {
            self.clone()
        }
    }
}
