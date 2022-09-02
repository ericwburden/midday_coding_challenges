use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct People {
    name: String,
    gender: String,
    birth_year: String,
    hair_color: String,
    eye_color: String,
    skin_color: String,
    height: String,
    mass: String,
    homeworld: String,
    species: Vec<String>,
    starships: Vec<String>,
    vehicles: Vec<String>,
    films: Vec<String>,
    url: String,
    created: String,
    edited: String,
}

/// Text representation for `People`
impl std::fmt::Display for People {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "********************")?;
        writeln!(f, "{}", self.name)?;
        writeln!(f, "********************")?;
        writeln!(f, " -> Gender: {}", self.gender)?;
        writeln!(f, " -> Born In: {}", self.birth_year)?;
        writeln!(f, " -> Hair: {}", self.hair_color)?;
        writeln!(f, " -> Eyes: {}", self.eye_color)?;
        writeln!(f, " -> Complexion: {}", self.skin_color)?;
        writeln!(f, " -> Height: {}cm", self.height)?;
        writeln!(f, " -> Weight: {}kg", self.mass)?;
        writeln!(f, " -> Hails From: {}", self.homeworld)?;
        
        if self.species.is_empty() {
            writeln!(f, " -> Species: None Listed")?;
        } else {
            writeln!(f, " -> Species:")?;
            for species in self.species.iter() {
                writeln!(f, "    -> {}", species)?;
            }
        }

        if self.starships.is_empty() {
            writeln!(f, " -> Starships: None Listed")?;
        } else {
            writeln!(f, " -> Starships:")?;
            for starship in self.starships.iter() {
                writeln!(f, "    -> {}", starship)?;
            }
        }

        if self.vehicles.is_empty() {
            writeln!(f, " -> Vehicles: None Listed")?;
        } else {
            writeln!(f, " -> Vehicles:")?;
            for vehicle in self.vehicles.iter() {
                writeln!(f, "    -> {}", vehicle)?;
            }
        }
        
        if self.films.is_empty() {
            writeln!(f, " -> Films Seen In: None Listed")?;
        } else {
            writeln!(f, " -> Films Seen In:")?;
            for film in self.films.iter() {
                writeln!(f, "    -> {}", film)?;
            }
        }

        writeln!(f, " -> Resource URL: {}", self.url)?;
        writeln!(f, " -> Record Created: {}", self.created)?;
        writeln!(f, " -> Last Edited: {}", self.edited)?;

        Ok(())
    }
}

