use crate::duration::Duration;
use crate::ld_md::LdJson;
pub use media::*;
use serde::{Deserialize, Serialize};
pub use sub_objects::*;

use std::borrow::Cow;
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum TypesOrArray<'t> {
    String(Cow<'t, str>),
    Number(usize),
    Array(Vec<Cow<'t, str>>),
}

impl<'t> TypesOrArray<'t> {
    fn get(&self) -> Cow<'t, str> {
        match self {
            TypesOrArray::String(val) => val.clone(),
            TypesOrArray::Number(val) => Cow::from(val.to_string()),
            TypesOrArray::Array(val) => val.first().unwrap().clone(),
        }
    }
}

pub mod media {
    use std::cmp::Ordering;

    use super::*;
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    #[serde(untagged)]
    pub enum Image<'r> {
        String(Cow<'r, str>),
        Array(Vec<Cow<'r, str>>),
        ImgArray(Vec<ImageObject<'r>>),
        Url(ImageObject<'r>),
        Id(IdImage<'r>),
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    pub struct IdImage<'r> {
        #[serde(rename = "@id")]
        id: Cow<'r, str>,
    }

    impl<'r> Image<'r> {
        pub fn get(&self) -> Cow<'r, str> {
            match self {
                Image::String(val) => val.clone(),
                Image::Array(val) => val.first().unwrap().clone(),
                Image::Url(val) => val.url.clone(),
                Image::Id(val) => val.id.clone(),
                Image::ImgArray(val) => {
                    let mut val = val.clone();
                    val.sort();
                    val.last().unwrap().url.clone()
                }
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
    pub struct ImageObject<'i> {
        #[serde(rename = "url")]
        pub(crate) url: Cow<'i, str>,
        pub(crate) width: Option<usize>,
        pub(crate) height: Option<usize>,
    }

    impl<'i> PartialOrd for ImageObject<'i> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(match (self.width, other.width) {
                (None, None) => Ordering::Equal,
                (Some(width), Some(o_width)) => width.cmp(&o_width),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
            })
        }
    }

    impl<'i> Ord for ImageObject<'i> {
        fn cmp(&self, other: &Self) -> Ordering {
            match (self.width, other.width) {
                (None, None) => Ordering::Equal,
                (Some(width), Some(o_width)) => width.cmp(&o_width),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LdRecipe<'r> {
    #[serde(borrow)]
    pub(crate) name: Cow<'r, str>,
    #[serde(borrow)]
    pub(crate) description: Option<Cow<'r, str>>,
    pub(crate) author: Author<'r>,
    #[serde(borrow)]
    pub(crate) image: Image<'r>,
    #[serde(borrow)]
    pub(crate) total_time: Option<Cow<'r, str>>,
    #[serde(borrow)]
    pub(crate) recipe_yield: Option<TypesOrArray<'r>>,
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
        self.name.clone()
    }
    fn description(&self) -> std::borrow::Cow<'_, str> {
        if let Some(desc) = &self.description {
            desc.clone()
        } else {
            Cow::from("")
        }
    }
    fn author(&self) -> std::borrow::Cow<'_, str> {
        match &self.author {
            Author::String(val) => val.clone(),
            Author::AuthorObject(author) => author.get().name,
        }
    }
    fn image(&self) -> std::borrow::Cow<'_, str> {
        self.image.get()
    }
    fn total_time(&self) -> Option<std::borrow::Cow<'_, str>> {
        self.clean_total_time()
    }
    fn recipe_yield(&self) -> Option<std::borrow::Cow<'_, str>> {
        self.recipe_yield.as_ref().map(|inner| {
            Cow::Owned(
                inner
                    .get()
                    .clone()
                    .replace("Serves : ", "")
                    .trim()
                    .to_string(),
            )
        })
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
        self.video.as_ref().map(|vid| vid.thumbnail_url.clone())
    }
}

pub mod sub_objects {
    use super::*;
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    pub struct AuthorObject<'a> {
        #[serde(borrow)]
        pub(crate) name: Cow<'a, str>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    #[serde(untagged)]
    pub enum Author<'a> {
        #[serde(borrow)]
        AuthorObject(SingleOrArray<AuthorObject<'a>>),
        String(Cow<'a, str>),
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    pub struct RecipeInstruction<'i> {
        #[serde(borrow)]
        pub text: Cow<'i, str>,
    }

    impl<'i> RecipeInstruction<'i> {
        pub fn simplify(self) -> Cow<'i, str> {
            self.text
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    pub struct HowToSection<'i> {
        #[serde(borrow)]
        pub(crate) name: Cow<'i, str>,
        #[serde(rename = "itemListElement")]
        pub(crate) instructions: Vec<RecipeInstruction<'i>>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    #[serde(untagged)]
    pub enum RecipeInstructionKinds<'r> {
        #[serde(borrow)]
        String(Cow<'r, str>),
        #[serde(borrow)]
        StringInstruction(Vec<Cow<'r, str>>),
        #[serde(borrow)]
        Instruction(Vec<RecipeInstruction<'r>>),
        #[serde(borrow)]
        NestedInstruction(Vec<Vec<RecipeInstruction<'r>>>),
        #[serde(borrow)]
        Sectioned(Vec<HowToSection<'r>>),
    }

    impl<'r> RecipeInstructionKinds<'r> {
        pub fn get(&self) -> RecipeInstructionKinds<'_> {
            self.clone()
        }
    }
}
