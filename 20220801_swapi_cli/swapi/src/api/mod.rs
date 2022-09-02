mod films;
mod people;
mod planets;
mod species;
mod starships;
mod vehicles;

use films::Films;
use people::People;
use planets::Planets;
use species::Species;
use starships::Starships;
use vehicles::Vehicles;

use serde::Deserialize;


//--------------------------------------------------------------------------------------
// Setup for communicating with the API. Uses the `serde` crate to deserialize results
// from SWAPI into Rust-native objects that can then be printed (or manipulated in
// other ways if you want to extend the functionality of the tool).
//--------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Resource {
    Films(Films),
    People(People),
    Planets(Planets),
    Species(Species),
    Starships(Starships),
    Vehicles(Vehicles),
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Films(film) => writeln!(f, "{}", film),
            Self::People(p) => writeln!(f, "{}", p),
            Self::Species(s) => writeln!(f, "{}", s),
            Self::Starships(s) => writeln!(f, "{}", s),
            Self::Vehicles(v) => writeln!(f, "{}", v),
            Self::Planets(p) => writeln!(f, "{}", p),
        }
    }
}


/// Responses come in two flavors: single results and multiples. Multiples come with
/// metadata indicating how many records are returned with URLs for the next and
/// previous page of results, along with a list of `Resource`s as results. A single
/// result is just one `Resource`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum SwapiResponse {
    Single(Resource),
    Multiple {
        count: u64,
        next: Option<String>,
        previous: Option<String>, 
        results: Vec<Resource>,
    }
}

/// Formatting string for displaying a response
impl std::fmt::Display for SwapiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwapiResponse::Single(r) => writeln!(f, "{}", r)?,
            SwapiResponse::Multiple { count, next, previous, results } => {
                writeln!(f, "--- Displaying {} results ---", count)?;
                if next.is_some() || previous.is_some() {
                    writeln!(f, "You can search other pages with the '-p' or '--page' flag.")?;
                }
                writeln!(f, "\n--- Search Results ---\n")?;
                for result in results {
                    writeln!(f, "{}", result)?;
                }
            }
        }
        Ok(())
    }
}