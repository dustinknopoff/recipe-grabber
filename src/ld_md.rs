use std::borrow::Cow;

use crate::sites::RecipeInstructionKinds;

pub trait LdJson: Sized {
    /// A recipe title
    fn name(&self) -> Cow<'_, str>;

    /// A recipe description
    fn description(&self) -> Cow<'_, str>;

    /// A recipe's author(s)
    fn author(&self) -> Cow<'_, str>;

    /// A recipe image
    fn image(&self) -> Cow<'_, str>;

    /// A recipe total time
    fn total_time(&self) -> Option<Cow<'_, str>>;

    /// A recipe yield
    fn recipe_yield(&self) -> Option<Cow<'_, str>>;

    /// A recipe ingredients
    fn ingredients(&self) -> Vec<Cow<'_, str>>;

    /// A recipe ingredients
    fn instructions(&self) -> RecipeInstructionKinds<'_>;

    /// A recipe video
    fn video(&self) -> Option<Cow<'_, str>>;
}

pub struct RecipeMarkdownBuilder<'r, T> {
    recipe: &'r T,
    markdown: Cow<'r, str>,
}

impl<'r, T: LdJson> RecipeMarkdownBuilder<'r, T> {
    pub fn new(recipe: &'r T) -> Self {
        Self {
            recipe,
            markdown: Cow::Owned(String::new()),
        }
    }

    fn add_title(&mut self) -> &mut Self {
        self.markdown
            .to_mut()
            .push_str(&format!("# {}\n\n", self.recipe.name()));
        self
    }

    fn add_authors(&mut self) -> &mut Self {
        self.markdown
            .to_mut()
            .push_str(&format!("By: {}\n\n", self.recipe.author()));
        self
    }

    fn add_image(&mut self) -> &mut Self {
        if let Some(val) = self.recipe.video() {
            self.markdown
                .to_mut()
                .push_str(&format!("![]({})\n\n", val))
        } else {
            self.markdown
                .to_mut()
                .push_str(&format!("![]({})\n\n", self.recipe.image()))
        };
        self
    }

    fn add_description(&mut self) -> &mut Self {
        self.markdown
            .to_mut()
            .push_str(&format!("{}\n\n", self.recipe.description().trim()));
        self
    }

    fn add_yield(&mut self) -> &mut Self {
        if let (Some(r_yield), Some(r_total_time)) =
            (self.recipe.recipe_yield(), self.recipe.total_time())
        {
            self.markdown
                .to_mut()
                .push_str(&format!("Yields: {} in {}\n\n", r_yield, r_total_time));
        };
        self
    }

    fn add_ingredients(&mut self) -> &mut Self {
        let mut out = String::from("## Ingredients\n");
        for item in self.recipe.ingredients().iter() {
            out.push_str(&format!("- {}\n", item.trim()))
        }
        out.push('\n');
        self.markdown.to_mut().push_str(&out);
        self
    }

    fn add_instructions(&mut self) -> &mut Self {
        let mut out = String::from("## Instructions\n\n");
        if let RecipeInstructionKinds::Sectioned(val) = self.recipe.instructions() {
            val.into_iter().for_each(|section| {
                out.push_str(&format!("### {}\n\n", section.name));
                for (idx, item) in section.instructions.iter().enumerate() {
                    out.push_str(&format!("{}. {}\n", idx + 1, item.text.trim()))
                }
            })
        } else {
            let val = self.recipe.instructions();
            let val: Vec<_> = match val {
                RecipeInstructionKinds::String(ref val) => {
                    val.split(". ").map(|v| v.trim()).map(Cow::from).collect()
                }
                RecipeInstructionKinds::Instruction(val) => {
                    val.into_iter().map(|x| x.simplify()).collect::<Vec<_>>()
                }
                RecipeInstructionKinds::NestedInstruction(val) => val
                    .into_iter()
                    .flat_map(|x| x.into_iter().map(|y| y.simplify()))
                    .collect::<Vec<_>>(),
                RecipeInstructionKinds::StringInstruction(val) => val,
                RecipeInstructionKinds::Sectioned(_) => {
                    unreachable!()
                }
            };
            for (idx, item) in val.iter().enumerate() {
                out.push_str(&format!("{}. {}\n", idx + 1, item.trim()))
            }
        }
        out.push('\n');
        self.markdown.to_mut().push_str(&out);
        self
    }

    fn add_source_fragment(&mut self) -> &mut Self {
        self.markdown
            .to_mut()
            .push_str(&format!("Source: [{}]", self.recipe.name()));
        self
    }

    pub fn build(&mut self) -> Cow<'r, str> {
        self.add_title()
            .add_authors()
            .add_image()
            .add_description()
            .add_yield()
            .add_ingredients()
            .add_instructions()
            .add_source_fragment();
        self.markdown.to_owned()
    }
}
