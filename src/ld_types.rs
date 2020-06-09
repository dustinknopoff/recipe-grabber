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
        let expected = r#"# Slow Cooker Pork Puttanesca Ragù

By: Sarah DiGregorio

![](https://static01.nyt.com/images/2018/11/29/dining/sg-slow-cooker-puttanesca-pork-ragu/sg-slow-cooker-puttanesca-pork-ragu-articleLarge.jpg)

This hearty ragù has all the punchy, briny flavors of traditional puttanesca (tomato, anchovies, capers, olives and red-pepper flakes), and introduces pork shoulder to the equation, making a particularly rich and meaty Sunday sauce. Deep flavor is built by starting the dish in a skillet, searing the pork and caramelizing the tomato paste until concentrated. The mixture might look dry as it gets transferred to the slow cooker, but as it cooks, the pork tenderizes and releases its juices. Before serving, add more tomato, along with lemon and parsley, to balance the deep, long-simmered flavors with fresh ones.

Yields: 6 to 8 servings in 2h 

#italian #dinner #weekday #meat #pastas #main_course

## Ingredients
- 3 to 3 1/2 pounds boneless, skinless pork shoulder
- Kosher salt
- 1 tablespoon olive oil, plus more as needed
- 8 large garlic cloves, roughly chopped
- 4 anchovy fillets, finely chopped, or 1 tablespoon anchovy paste
- 2 (6-ounce) cans tomato paste
- 1/3 cup pitted kalamata olives
- 1/4 cup drained capers
- 1 tablespoon red wine vinegar
- 2 teaspoons red-pepper flakes, plus more to taste
- 1 teaspoon dried oregano
- Freshly ground black pepper
- 1 (14.5-ounce) can whole or crushed tomatoes
- 2 tablespoons fresh lemon juice (about 1/2 lemon)
- 1 cup chopped flat-leaf parsley, lightly packed
- Grated Parmigiano-Reggiano, for serving


## Instructions
1. Using a sharp knife, trim and discard the large hunks of fat from the pork shoulder then cut the meat into 4 even pieces. Season the pork generously on all sides with salt. Heat the olive oil in a large skillet over medium-high. Working in two batches if necessary, brown the pork on two sides, about 5 minutes per side. Using tongs, transfer the pork to a 5- to 8-quart slow cooker.
2. Add the garlic and anchovies to the skillet, along with more oil if needed, and cook over medium, stirring, until fragrant, about 2 minutes. Add the tomato paste and cook, stirring constantly and scraping up any browned bits on the bottom of the pan, until fragrant and slightly darkened in color, about 3 minutes. Turn off the heat and stir in the olives, capers, vinegar, red-pepper flakes, oregano and a generous amount of black pepper. (Do not add more salt at this point because anchovies, olives and capers can be quite salty.) Scrape the mixture into the slow cooker with the pork and stir until combined.
3. Cover the slow cooker and cook on low until the pork is fork-tender and the sauce deepens in color, about 10 hours.
4. Using two forks, coarsely shred the pork. Pour the can of tomatoes and juices into the slow cooker, crushing the tomatoes with your hands, if using whole. Add the parsley and lemon juice. Taste and add more red-pepper flakes or salt if necessary.
5. Serve the ragù over polenta or sturdy pasta, like rigatoni or pappardelle, topped with Parmesan to taste. (If serving the ragù with pasta, loosen the ragù with a bit of pasta cooking water, adding it spoonful by spoonful, to help the sauce coat the pasta.)


### Nutrition
- calories: 526
- unsaturated fat: 21 grams
- carbohydrate: 15 grams
- cholesterol: 
- fat: 36 grams
- fiber: 4 grams
- protein: 36 grams
- saturated fat: 12 grams
- sodium: 750 milligrams
- sugar: 8 grams
- trans fat: 
        

Source: [Slow Cooker Pork Puttanesca Ragù]"#;
        assert_eq!(expected, as_recipe.as_md());
    }
}
