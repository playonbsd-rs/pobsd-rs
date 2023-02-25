use crate::parser::{Parser, ParserResult, Game};
use std::path::Path;
use std::io::Write;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub(crate) mod db;
pub(crate) mod parser;
pub mod commands;

pub use crate::commands::browse::browse;

pub fn check(db: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let parser = Parser::default();
    match parser.load_from_file(&db)? {
        ParserResult::WithError(games, lines) => {
            let message: Vec<String> = lines.into_iter().map(|x| x.to_string()).collect();
            println!("> {} games parsed.", games.len());
            println!("> Error occured at lines {}.", message.join(", "));
        }
        ParserResult::WithoutError(games) => {
            println!("> {} games parsed without error.", games.len());
        }
    }
    Ok(())
}

pub fn export(db: impl AsRef<Path>, js: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let parser = Parser::default();
    match parser.load_from_file(&db)? {
        ParserResult::WithError(_, lines) => {
            let message: Vec<String> = lines.into_iter().map(|x| x.to_string()).collect();
            eprintln!("> Error occured at lines {}.", message.join(", "));
            eprintln!("> Export aborted.");
        }
        ParserResult::WithoutError(games) => {
            println!("{}", serde_json::to_string_pretty(&games).unwrap());
            let mut js = std::fs::File::create(js)?;
            js.write(&serde_json::to_vec_pretty(&games).unwrap())?;
        }
    }
    Ok(())
}

pub fn add_game(file: impl AsRef<Path>) {
    let mut game = Game::new();
}


