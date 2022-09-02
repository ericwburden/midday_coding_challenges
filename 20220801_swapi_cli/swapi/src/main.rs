mod cli;
mod api;

use cli::Cli;
use api::SwapiResponse;

use clap::Parser;
use eyre::Result;

const BASE_URL: &'static str = "https://swapi.dev/api";


fn main() -> Result<()> {
    // Parse arguments into an URL for the request
    let args = Cli::parse();
    let url = format!("{}/{}", BASE_URL, args.get_cmd());

    // Make the request and deserialize the body
    let res = reqwest::blocking::get(url)?; 
    let body = res.json::<SwapiResponse>()?;

    // Print the result (single or multiple) to the console
    println!("{}", body);

    // Indicate successful return
    Ok(())
}
