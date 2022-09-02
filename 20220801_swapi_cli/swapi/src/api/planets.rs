use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Planets {
    name: String,
    diameter: String,
    rotation_period: String,
    orbital_period: String,
    gravity: String,
    population: String,
    climate: String,
    terrain: String,
    surface_water: String,
    residents: Vec<String>,
    films: Vec<String>,
    url: String,
    created: String,
    edited: String,
}

/// Text representation for `Planets`
impl std::fmt::Display for Planets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "********************")?;
        writeln!(f, "{}", self.name)?;
        writeln!(f, "********************")?;
        writeln!(f, " -> Diameter: {} kg", self.diameter)?;
        writeln!(f, " -> Rotation Period: {} hours", self.rotation_period)?;
        writeln!(f, " -> Orbital Period: {} days", self.orbital_period)?;
        writeln!(f, " -> Gravity: {} G", self.gravity)?;
        writeln!(f, " -> Population: {}", self.population)?;
        writeln!(f, " -> Climate(s): {}", self.climate)?;
        writeln!(f, " -> Terrain(s): {}", self.terrain)?;
        writeln!(f, " -> Surface Water: {}%", self.surface_water)?;

        if self.residents.is_empty() {
            writeln!(f, " -> Notable Residents: None Listed")?;
        } else {
            writeln!(f, " -> Notable Residents:")?;
            for resident in self.residents.iter() {
                writeln!(f, "    -> {}", resident)?;
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

