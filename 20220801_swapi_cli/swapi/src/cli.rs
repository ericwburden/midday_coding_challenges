use clap::{Parser, Subcommand};

//--------------------------------------------------------------------------------------
// Setup for the CLI (via `clap`). Defines what arguments are accepted and how. It's set
// up in such a way as to make it easy to extend the CLI later with additional resource
// types like species, starships, planets, etc.
//--------------------------------------------------------------------------------------

/// A CLI for fetching Star Wars data
#[derive(Debug, Parser)]
#[clap(name = "swapi")]
#[clap(about = "A CLI for fetching Star Wars data", long_about = None)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    cmd: ResourceCmd,
}

impl Cli {
    pub(crate) fn get_cmd(&self) -> &ResourceCmd {
        &self.cmd
    }
}

/// Subcommand indicating which type of resource to fetch
#[derive(Debug, Subcommand)]
pub(crate) enum ResourceCmd {
    /// Searches for people
    #[clap(arg_required_else_help = true)]
    People {
        #[clap(subcommand)]
        search_by: SearchBy,
    },

    /// Searches for a species
    #[clap(arg_required_else_help = true)]
    Species {
        #[clap(subcommand)]
        search_by: SearchBy,
    },

    /// Searches for starships
    #[clap(arg_required_else_help = true)]
    Starships {
        #[clap(subcommand)]
        search_by: SearchBy,
    },

    /// Searches for vehicles
    #[clap(arg_required_else_help = true)]
    Vehicles {
        #[clap(subcommand)]
        search_by: SearchBy,
    },

    /// Searches for planets
    #[clap(arg_required_else_help = true)]
    Planets {
        #[clap(subcommand)]
        search_by: SearchBy,
    },

    /// Searches for films
    #[clap(arg_required_else_help = true)]
    Films {
        #[clap(subcommand)]
        search_by: SearchBy,
    },
}

/// Used to format the URL to fetch from SWAPI
impl std::fmt::Display for ResourceCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::People { search_by } => write!(f, "people/{}", search_by),
            Self::Species { search_by } => write!(f, "species/{}", search_by),
            Self::Starships { search_by } => write!(f, "starships/{}", search_by),
            Self::Vehicles { search_by } => write!(f, "vehicles/{}", search_by),
            Self::Planets { search_by } => write!(f, "planets/{}", search_by),
            Self::Films { search_by } => write!(f, "films/{}", search_by),
        }
    }
}

/// This enum helps make each `Entity` more generic. Just use this sub-command
/// on any entity that can be searched by ID or by a search string.
#[derive(Debug, Subcommand)]
pub(crate) enum SearchBy {
    /// Fetch a single entity by ID
    #[clap(arg_required_else_help = true)]
    ID {
        #[clap(value_parser)]
        get: u8
    },

    /// Search for entities that match a string
    #[clap(arg_required_else_help = true)]
    Search {
        #[clap(value_parser)]
        search: String,

        #[clap(long, short, value_parser)]
        page: Option<u8>,
    },
}

/// Also used to format the URL to fetch from SWAPI. Uses either the id or
/// search parameters (search string and page)
impl std::fmt::Display for SearchBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ID { get} => write!(f, "{}", get),
            Self::Search { search, page} => match page {
                None => write!(f, "?search={}", search),
                Some(n) => write!(f, "?search={}&page={}", search, n)
            }
        }
    }
}