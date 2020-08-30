use std::io::{stdout, Write};

use curl::easy::Easy;


fn main() {
    let mut easy = Easy::new();
easy.url("https://api.chess.com/pub/player/lgbarn/games/archives").unwrap();
easy.write_function(|data| {
    stdout().write_all(data).unwrap();
    Ok(data.len())
}).unwrap();
easy.perform().unwrap();

}
