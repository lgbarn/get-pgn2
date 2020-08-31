use error_chain::error_chain;
use std::io::Read;
use serde_json;
use serde::{Deserialize};

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

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("https://api.chess.com/pub/player/lgbarn/games/archives")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let deserialized: Archive = serde_json::from_str(&body).unwrap();

    println!("deserialized = {:?}", deserialized);

    Ok(())
}


