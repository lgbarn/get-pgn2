use clap::{App, Arg};
use error_chain::error_chain;
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::string::String;

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
        .arg(
            Arg::with_name("l")
                .short("l")
                .long("lichess")
                .takes_value(false),
        )
        .get_matches();

    let currplayer = String::from(matches.value_of("player").unwrap());

    let url: String = if matches.is_present("l"){
        format!("https://lichess.org/api/games/user/{}", currplayer)
    } else {
        format!("https://api.chess.com/pub/player/{}/games/archives", currplayer)
    };
    //println!("URL: {}", url);
    
    let mut res = reqwest::blocking::get(&url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let deserialized: Archive = serde_json::from_str(&body).unwrap();

    let filename = currplayer.to_string() + ".pgn";

    let mut file = File::create(filename)?;

    for line in deserialized.get_months().iter() {
        let monthly_games_url = format!("{}/pgn", line);

        let mut res = reqwest::blocking::get(&monthly_games_url)?;
        let mut data = String::new();
        res.read_to_string(&mut data)?;

        println!(
            "Downloading games from {} for {}",
            monthly_games_url, currplayer
        );

        writeln!(&mut file, "{}", data)?;
    }

    Ok(())
    
}
