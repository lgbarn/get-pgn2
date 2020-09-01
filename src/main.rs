use error_chain::error_chain;
use serde::Deserialize;
use serde_json;
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[derive(Deserialize, Debug)]
struct Archive {
    archives: Vec<String>,
}

impl Archive {
    pub fn get_months(self) -> Vec<String> {
        self.archives
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("https://api.chess.com/pub/player/lgbarn1966/games/archives")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let deserialized: Archive = serde_json::from_str(&body).unwrap();

    let mut file = File::create("lgbarn1966.pgn")?;

    for month in deserialized.get_months().iter() {
        let month = format!("{}/pgn", month);

        println!("Downloading game from {} for lgbarn1966", month);

        let mut res = reqwest::blocking::get(&month)?;
        let mut data = String::new();
        res.read_to_string(&mut data)?;

        writeln!(&mut file, "{}", data)?;
    }

    Ok(())
}