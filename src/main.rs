use clap::{App, Arg};
use error_chain::error_chain;
use serde::Deserialize;
use std::fs::OpenOptions;
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

    let currplayer = || String::from(matches.value_of("player").unwrap());
    let check_site = || matches.is_present("l");

    if check_site() {
        download_games(
            format!("https://lichess.org/api/games/user/{}", currplayer()),
            currplayer(),
        )?
    } else {
        let chess_url = format!(
            "https://api.chess.com/pub/player/{}/games/archives",
            &currplayer()
        );

        let mut body = String::new();
        reqwest::blocking::get(&chess_url)?.read_to_string(&mut body)?;
        let deserialized: Archive = serde_json::from_str(&body).unwrap();

        deserialized.get_months().iter().for_each(|line| {
            let monthly_games_url = format!("{}/pgn", line);

            let download = download_games(monthly_games_url.to_string(), currplayer());
            match download {
                Ok(()) => println!("Successfully downloaded games"),
                Err(err) => println!("Could not download games: {:?}", err),
            }
        });
    };

    Ok(())
}

fn download_games(url: String, currplayer: String) -> Result<()> {
    println!("Downloading games from {} for {}", url, currplayer);
    let mut data = String::new();
    let mut option = OpenOptions::new();
    option.write(true).append(true).create(true);

    reqwest::blocking::get(&url)?.read_to_string(&mut data)?;
    writeln!((option.open(currplayer.to_string() + "pgn")?), "{}", &data)?;

    Ok(())
}
