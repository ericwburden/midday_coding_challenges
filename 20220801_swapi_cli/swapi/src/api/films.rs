use serde::Deserialize;
use textwrap::indent;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Films {
    title: String,
    episode_id: u64,
    opening_crawl: String,
    director: String,
    producer: String,
    release_date: String,
    species: Vec<String>,
    starships: Vec<String>,
    vehicles: Vec<String>,
    characters: Vec<String>,
    planets: Vec<String>,
    url: String,
    created: String,
    edited: String,
}

/// Text representation for `Films`
impl std::fmt::Display for Films {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "********************")?;
        writeln!(f, "{}", self.title)?;
        writeln!(f, "********************")?;
        writeln!(f, " -> Episode: {}", self.episode_id)?;
        writeln!(f, " -> Directed By: {}", self.director)?;
        writeln!(f, " -> Produced By: {}", self.producer)?;
        writeln!(f, " -> Released: {}", self.release_date)?;
        
        if self.species.is_empty() {
            writeln!(f, " -> Species Shown: None Listed")?;
        } else {
            writeln!(f, " -> Species Shown:")?;
            for species in self.species.iter() {
                writeln!(f, "    -> {}", species)?;
            }
        }

        if self.starships.is_empty() {
            writeln!(f, " -> Starships Shown: None Listed")?;
        } else {
            writeln!(f, " -> Starships Shown:")?;
            for starship in self.starships.iter() {
                writeln!(f, "    -> {}", starship)?;
            }
        }

        if self.vehicles.is_empty() {
            writeln!(f, " -> Vehicles Shown: None Listed")?;
        } else {
            writeln!(f, " -> Vehicles Shown:")?;
            for vehicle in self.vehicles.iter() {
                writeln!(f, "    -> {}", vehicle)?;
            }
        }

        if self.planets.is_empty() {
            writeln!(f, " -> Planets Shown: None Listed")?;
        } else {
            writeln!(f, " -> Planets Shown:")?;
            for planet in self.planets.iter() {
                writeln!(f, "    -> {}", planet)?;
            }
        }
        
        if self.characters.is_empty() {
            writeln!(f, " -> Major Characters: None Listed")?;
        } else {
            writeln!(f, " -> Major Characters:")?;
            for character in self.characters.iter() {
                writeln!(f, "    -> {}", character)?;
            }
        }

        if !self.opening_crawl.is_empty() {
            writeln!(f, " -> Opening Crawl:")?;
            writeln!(f, "{}", indent(&self.opening_crawl, "\t"))?;
        }

        writeln!(f, " -> Resource URL: {}", self.url)?;
        writeln!(f, " -> Record Created: {}", self.created)?;
        writeln!(f, " -> Last Edited: {}", self.edited)?;

        Ok(())
    }
}

