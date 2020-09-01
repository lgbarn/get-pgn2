use clap::{App, Arg};
use error_chain::error_chain;
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::{Read, Write};

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
    let matches = App::new("Get games from Chess.com")
        .version("0.1.0")
        .author("Luther Barnum")
        .about("Retrieves games from Chess.com in PGN format")
        .arg(
            Arg::with_name("player")
                .short("p")
                .long("player")
                .takes_value(true),
        )
        .get_matches();

    let  currplayer = matches.value_of("player").unwrap();

    let url = "https://api.chess.com/pub/player/".to_string() + currplayer + "/games/archives";

    println!("{}", url);

    let mut res =
        reqwest::blocking::get(&url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let deserialized: Archive = serde_json::from_str(&body).unwrap();

    let filename = currplayer.to_string() + ".pgn";

    let mut file = File::create(filename)?;

    for month in deserialized.get_months().iter() {
        let month = format!("{}/pgn", month);

        let mut res = reqwest::blocking::get(&month)?;
        let mut data = String::new();
        res.read_to_string(&mut data)?;

        println!("Downloading games from {} for lgbarn1966", month);

        writeln!(&mut file, "{}", data)?;
    }

    Ok(())
}
