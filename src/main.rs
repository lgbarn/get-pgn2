use error_chain::error_chain;
use std::io::Read;
use serde_json;
use serde::{Deserialize};
//use curl::easy::Easy;
//use curl::easy;
//use restson::RestClient;


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
    pub fn get_months(self)  -> Vec<String> {
        self.archives
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("https://api.chess.com/pub/player/lgbarn/games/archives")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let deserialized: Archive = serde_json::from_str(&body).unwrap();

    println!("{:?}", deserialized.get_months());

   
    let mut res = reqwest::blocking::get("https://api.chess.com/pub/player/lgbarn/games/2010/05/pgn")?;
    let mut data = String::new();
    res.read_to_string(&mut data)?;

    
    println!("{}", data);
    

    Ok(())
}


