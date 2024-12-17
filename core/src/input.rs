use crate::prelude::*;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};
use std::{env, fs, path::PathBuf};

pub fn get_input(year: u32, day: u32) -> Result<String> {
    let cargo_dir = find_cargo_dir().ok_or(error!("Could not find Cargo.toml directory"))?;
    let input_path = cargo_dir.join("input.txt");

    if input_path.exists() {
        return fs::read_to_string(&input_path)
            .map_err(|e| error!("Failed to read existing input.txt: {}", e));
    }

    let token =
        env::var("AOC_TOKEN").map_err(|_| error!("AOC_TOKEN environment variable not set"))?;
    let user_agent = env::var("AOC_USER_AGENT")
        .map_err(|_| error!("AOC_USER_AGENT environment variable not set"))?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = Client::new();
    let response = client
        .get(&url)
        .header(COOKIE, format!("session={}", token))
        .header(USER_AGENT, user_agent)
        .send()
        .map_err(|e| error!("Failed to download input: {}", e))?;

    if !response.status().is_success() {
        return Err(error!(
            "Failed to download input: HTTP {}",
            response.status()
        ));
    }

    let input = response
        .text()
        .map_err(|e| error!("Failed to read response: {}", e))?;

    fs::write(&input_path, &input).map_err(|e| error!("Failed to write input.txt: {}", e))?;

    Ok(input)
}

fn find_cargo_dir() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;

    loop {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Some(current_dir);
        }

        if !current_dir.pop() {
            return None;
        }
    }
}
