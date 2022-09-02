use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Species {
    name: String,
    designation: String,
    classification: String,
    homeworld: String,
    language: String,
    average_height: String,
    average_lifespan: String,
    eye_colors: String,
    hair_colors: String,
    skin_colors: String,
    people: Vec<String>,
    films: Vec<String>,
    created: String,
    edited: String,
    url: String,
}

/// Text representation for `Species`
impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "********************")?;
        writeln!(f, "{}", self.name)?;
        writeln!(f, "********************")?;
        writeln!(f, " -> Designation: {}", self.designation)?;
        writeln!(f, " -> Classification: {}", self.classification)?;
        writeln!(f, " -> Homeworld: {}", self.homeworld)?;
        writeln!(f, " -> Language: {}", self.language)?;
        writeln!(f, " -> Average Height: {}cm", self.average_height)?;
        writeln!(f, " -> Average Lifespan: {}yrs", self.average_lifespan)?;
        writeln!(f, " -> Possible Eye Colors: {}", self.eye_colors)?;
        writeln!(f, " -> Possible Hair Colors: {}", self.hair_colors)?;
        writeln!(f, " -> Possible Skin Colors: {}", self.skin_colors)?;
        
        if self.people.is_empty() {
            writeln!(f, " -> Notable Individuals: None Listed")?;
        } else {
            writeln!(f, " -> Notable Individuals:")?;
            for person in self.people.iter() {
                writeln!(f, "    -> {}", person)?;
            }
        }

        Ok(())
    }
}