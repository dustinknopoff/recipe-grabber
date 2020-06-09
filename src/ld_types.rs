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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "@context")]
    pub(crate) context: String,
    #[serde(rename = "@type")]
    pub(crate) recipe_type: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) author: Author,
    pub(crate) image: String,
    #[serde(rename = "totalTime")]
    pub(crate) total_time: Option<String>,
    #[serde(rename = "recipeYield")]
    pub(crate) recipe_yield: Option<String>,
    #[serde(rename = "recipeCuisine")]
    pub(crate) recipe_cuisine: Option<String>,
    #[serde(rename = "recipeCategory")]
    pub(crate) recipe_category: Option<String>,
    pub(crate) nutrition: Option<Nutrition>,
    #[serde(rename = "recipeIngredient")]
    pub(crate) recipe_ingredient: Vec<String>,
    #[serde(rename = "recipeInstructions")]
    pub(crate) recipe_instructions: Vec<RecipeInstruction>,
    pub(crate) video: Option<Video>,
}

fn option_or_empty(val: Option<String>) -> String {
    if let Some(i) = val {
        i
    } else {
        String::new()
    }
}

impl Recipe {
    pub fn as_md(&mut self) -> String {
        self.clean_total_time();
        self.categories_as_tags();
        format!(
            r#"# {}

By: {}

![]({})

{}

Yields: {} in {}

#{} {}

{}

{}

{}

Source: [{}]"#,
            self.name,
            self.author.name,
            self.image,
            self.description,
            option_or_empty(self.recipe_yield.clone()),
            option_or_empty(self.total_time.clone()),
            option_or_empty(self.recipe_cuisine.clone()),
            option_or_empty(self.recipe_category.clone()),
            Self::ul(self.recipe_ingredient.clone(), String::from("Ingredients")),
            Self::ol(
                self.recipe_instructions
                    .iter()
                    .map(|x| x.simplify())
                    .collect::<Vec<_>>(),
                String::from("Instructions")
            ),
            option_or_empty(self.nutrition.clone().map(|x| x.as_md())),
            self.name
        )
    }

    fn clean_total_time(&mut self) {
        self.total_time = self
            .total_time
            .clone()
            .map(|x| x.replace("PT", "").replace("H", "h ").replace("M", "m"));
    }

    fn categories_as_tags(&mut self) {
        self.recipe_category = self.recipe_category.clone().map(|x| {
            x.split(", ")
                .map(|y| format!("#{}", y.replace(" ", "_")))
                .collect::<Vec<_>>()
                .join(" ")
        });
    }

    fn ul(list: Vec<String>, title: String) -> String {
        let mut out = format!("## {}\n", title);
        for item in list.iter() {
            out.push_str(&format!("- {}\n", item))
        }
        out
    }
    fn ol(list: Vec<String>, title: String) -> String {
        let mut out = format!("## {}\n", title);
        for (idx, item) in list.iter().enumerate() {
            out.push_str(&format!("{}. {}\n", idx + 1, item))
        }
        out
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregateRating {
    #[serde(rename = "@type")]
    pub(crate) aggregate_rating_type: String,
    #[serde(rename = "ratingValue")]
    pub(crate) rating_value: i64,
    #[serde(rename = "ratingCount")]
    pub(crate) rating_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    #[serde(rename = "@type")]
    pub(crate) author_type: String,
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Nutrition {
    fn as_md(&self) -> String {
        format!(
            r#"### Nutrition
- calories: {}
- unsaturated fat: {}
- carbohydrate: {}
- cholesterol: {}
- fat: {}
- fiber: {}
- protein: {}
- saturated fat: {}
- sodium: {}
- sugar: {}
- trans fat: {}
        "#,
            self.calories,
            self.unsaturated_fat_content,
            self.carbohydrate_content,
            option_or_empty(self.cholesterol_content.clone()),
            self.fat_content,
            self.fiber_content,
            self.protein_content,
            self.saturated_fat_content,
            self.sodium_content,
            self.sugar_content,
            option_or_empty(self.trans_fat_content.clone())
        )
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeInstruction {
    #[serde(rename = "@context")]
    pub(crate) context: String,
    #[serde(rename = "@type")]
    pub(crate) recipe_instruction_type: String,
    pub(crate) text: String,
}

impl RecipeInstruction {
    fn simplify(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    #[serde(rename = "@context")]
    pub(crate) context: String,
    #[serde(rename = "@type")]
    pub(crate) video_type: String,
    pub(crate) name: String,
    pub(crate) description: String,
    #[serde(rename = "thumbnailUrl")]
    pub(crate) thumbnail_url: String,
    #[serde(rename = "uploadDate")]
    pub(crate) upload_date: String,
    pub(crate) duration: String,
}

#[cfg(test)]
mod tests {

    use super::Recipe;

    #[test]
    fn ragu() {
        let src = include_str!("../tests/ragu.json");
        let mut as_recipe: Recipe = serde_json::from_str(&src).unwrap();
        eprintln!("{}", as_recipe.as_md());
    }
}
