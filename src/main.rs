mod library;
use clap::{arg, Command};
use library::browse;
use pledge::pledge_promises;
use unveil::unveil;
extern crate serde_json;

fn cli() -> Command {
    Command::new("pobsd")
        .about("A tool to interact and manipulate the PlayOnBSD Database")
        .arg(arg!(<DATABASE> "The Database"))
        .arg_required_else_help(true)
}

fn main() -> Result<(), std::io::Error> {
    pledge_promises![Stdio Unveil Rpath Tty]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    let matches = cli().get_matches();
    let db = matches.get_one::<String>("DATABASE").expect("required");
    unveil(db, "r")
        .or_else(unveil::Error::ignore_platform)
        .unwrap();
    unveil("", "")
        .or_else(unveil::Error::ignore_platform)
        .unwrap();
    browse(db)?;
    Ok(())
}
