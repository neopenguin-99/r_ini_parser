use std::fs::{self, read};

use clap::{arg, crate_authors, crate_version, value_parser, Arg, ArgAction, ArgMatches, Command, ValueEnum};
use std::rc::Rc;
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
    let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();
    let parse = parse(&file_contents_split, String::from("main"));
    println!("parse: {:#?}", parse);
    Ok(())
}

fn parse(file_contents: &[&str], section_name: String) -> Section {
    let mut section_to_add: Section = Section::new(section_name.clone(), vec![], HashMap::new());
    let mut line_number = 0;
    for file_contents_line in file_contents {
        println!("file_contents: {:#?}", file_contents);
        if let Some(i) = file_contents_line.chars().position(|x| x == '=') {
            let _ = section_to_add.key_value_pair_hashmap.insert(String::from(file_contents_line[..i].trim()), String::from(file_contents_line[i+1..].trim())); 

        }
        else if file_contents_line.trim_start().starts_with('[') && file_contents_line.trim_end().ends_with(']') {
            println!("enter here");
            let section_name = String::from(&file_contents_line[..file_contents_line.len()]);
            let lines_remaining = file_contents.get((line_number + 1)..);
            if lines_remaining.is_some() {
                section_to_add.sub_sections.push(Rc::new(parse(lines_remaining.unwrap(), section_name)));
            }
            return section_to_add;
        }
        line_number += 1;
    }
    section_to_add
}

#[derive(Debug)]
struct Section {
    section_name: String,
    sub_sections: Vec<Rc<Section>>,
    key_value_pair_hashmap: HashMap<String, String>
}
impl Section {
    fn new(section_name: String, sub_sections: Vec<Rc<Section>>, key_value_pair_hashmap: HashMap<String, String>) -> Section {
        Section {
            section_name,
            sub_sections,
            key_value_pair_hashmap
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use tempfile::{Builder, NamedTempFile, TempDir};

    #[test]
    fn parse_contents_as_section_name_when_section_is_not_provided() -> Result<(), Box<dyn std::error::Error>> {
        let mut tempfile: NamedTempFile = NamedTempFile::new()?;
        let file_contents = "
author=\"me\"
name=\"mypackage\"
version=\"0.1.0\"";
        let _ = tempfile.write(file_contents.as_bytes());
        let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();
        let res = parse(&file_contents_split, String::from("main"));
        assert_eq!(res.key_value_pair_hashmap.keys().try_find(|f| **f.eq("author")), );
        println!("{:#?}", res);
        Ok(())
    }
}
