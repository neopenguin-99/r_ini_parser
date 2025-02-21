use std::fs::{self, read};

use clap::{arg, crate_authors, crate_version, value_parser, Arg, ArgAction, ArgMatches, Command, ValueEnum};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut matches: ArgMatches = Command::new("parser")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(Arg::new("file")
        .value_parser(value_parser!(String))
        .num_args(1))
        .get_matches();
    let file_name = match matches.remove_one::<String>("file") {
        Some(x) => x,
        None => panic!("please provide a file name")
    };
    let file_contents = fs::read_to_string(file_name)?;
    println!("file_contents: {}", file_contents);
    Ok(())
}
