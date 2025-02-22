use std::fs::{self, read};

use clap::{arg, crate_authors, crate_version, value_parser, Arg, ArgAction, ArgMatches, Command, ValueEnum};
use std::collections::HashMap;


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
    let file_contents = fs::read_to_string(&file_name)?;
    let file_contents_split = file_name.split('\n').collect::<Vec<_>>();
    println!("file_contents: {:#?}", file_contents);
    Ok(())
}

fn parse(file_contents: &Vec<&str>) -> Section {
    let section_to_add: Section;
    for file_contents_line in file_contents {
        if file_contents_line.contains('=') {

        }
        else if file_contents_line.starts_with('[') && file_contents_line.ends_with(']') {
            let section_name = &file_contents_line[0..file_contents_line.len() - 1];
            section_to_add.sub_sections.push(Section::new(parse(file_contents)))
        }
    }
    Section::new(String::from("main"), Vec::new(), HashMap::new())
}

struct Section {
    section_name: String,
    sub_sections: Vec<Box<Section>>,
    key_value_pair_hashmap: HashMap<String, String>
}
impl Section {
    fn new(section_name: String, sub_section: Vec<Box<Section>>, key_value_pair_hashmap: HashMap<String, String>) -> Section {
        Section {
            section_name,
            sub_section,
            key_value_pair_hashmap
        }
    }
}
