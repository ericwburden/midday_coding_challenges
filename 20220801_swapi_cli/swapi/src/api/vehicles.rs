use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Vehicles {
    name: String,
    model: String,
    vehicle_class: String,
    manufacturer: String,
    length: String,
    cost_in_credits: String,
    crew: String,
    passengers: String,
    max_atmosphering_speed: String,
    cargo_capacity: String,
    consumables: String,
    films: Vec<String>,
    pilots: Vec<String>,
    created: String,
    edited: String,
    url: String,
}

/// Text representation for `Vehicles`
impl std::fmt::Display for Vehicles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "********************")?;
        writeln!(f, "{}", self.name)?;
        writeln!(f, "********************")?;
        writeln!(f, " -> Model: {}", self.model)?;
        writeln!(f, " -> Class: {}", self.vehicle_class)?;
        writeln!(f, " -> Manufactured By: {}", self.manufacturer)?;
        writeln!(f, " -> Cost: {} credits", self.cost_in_credits)?;
        writeln!(f, " -> Length: {}m", self.length)?;
        writeln!(f, " -> Crew Size: {}", self.crew)?;
        writeln!(f, " -> Max Passengers: {}", self.passengers)?;
        writeln!(f, " -> Atmospheric Top Speed: {}", self.max_atmosphering_speed)?;
        writeln!(f, " -> Cargo Capacity: {} kg", self.cargo_capacity)?;
        writeln!(f, " -> Resupply Time: {}", self.consumables)?;
        
        if self.pilots.is_empty() {
            writeln!(f, " -> Known Pilots: None Listed")?;
        } else {
            writeln!(f, " -> Known Pilots:")?;
            for pilot in self.pilots.iter() {
                writeln!(f, "    -> {}", pilot)?;
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