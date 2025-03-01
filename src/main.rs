#![feature(try_find)]
use std::fs::{self};

use clap::{crate_authors, crate_version, value_parser, Arg, ArgMatches, Command};
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
    let file_name = matches.remove_one::<String>("file").expect("please provide a file name");
    let file_contents = fs::read_to_string(&file_name)?;
    let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();
    let parse = parse(&file_contents_split, None, None);
    println!("parse: {:#?}", parse);
    Ok(())
}

fn parse(file_contents: &[&str], seek_to_column_in_line: Option<usize>, section_name: Option<String>) -> Section {
    let mut section_to_add: Section = Section::new(section_name.unwrap_or(String::from("global")), vec![], HashMap::new());
    let mut line_number = 0;
    for file_contents_line in file_contents {
        if let Some(i) = file_contents_line.chars().position(|x| x == '=') {
            let _ = section_to_add.key_value_pair_hashmap.insert(String::from(file_contents_line[..i].trim()), String::from(file_contents_line[i+1..].trim())); 
        }
        else if file_contents_line.trim_start().starts_with('[') && file_contents_line.trim_end().ends_with(']') {
            let file_contents_line_as_char_vec: Vec<char> = file_contents_line.chars().collect();
            if let Some(location_of_dot_separator) = file_contents_line_as_char_vec[seek_to_column_in_line.unwrap_or(1)..].iter().position(|x| *x == '.') {
                println!("new");
                println!("{:#?}", seek_to_column_in_line);
                println!("{}", location_of_dot_separator + 1);
                println!("{}", &file_contents_line);
                // println!("{}", &file_contents_line[seek_to_column_in_line.unwrap_or(1)..location_of_dot_separator + 1]);
                let section_name = String::from(&file_contents_line[seek_to_column_in_line.unwrap_or(1)..seek_to_column_in_line.unwrap_or(1) + location_of_dot_separator]);
                // let section_name_2 = String::from(&file_contents_line[seek_to_column_in_line.unwrap_or(1)..location_of_dot_separator + 1]);
                // if String::from(&file_contents_line[seek_to_column_in_line.unwrap_or(1)..location_of_dot_separator]) == section_to_add.section_name {
                    // section_to_add.key_value_pair_hashmap.insert(String::from(file_contents_line[..i].trim()), String::from(file_contents_line[i+1..].trim()));
                // }
                println!("{}", section_name);
                let lines_remaining = file_contents.get((line_number)..);
                section_to_add.sub_sections.push(Rc::new(parse(lines_remaining.unwrap(), Some(location_of_dot_separator + 1), Some(section_name))));
            }
            else {
                let section_name = String::from(&file_contents_line[1..file_contents_line.len() - 1]);
                let lines_remaining = file_contents.get((line_number + 1)..);
                if lines_remaining.is_some() {
                    section_to_add.sub_sections.push(Rc::new(parse(lines_remaining.unwrap(), None, Some(section_name))));
                }
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
    use tempfile::NamedTempFile;

    #[test]
    fn file_can_be_parsed_with_no_sections() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let mut tempfile: NamedTempFile = NamedTempFile::new()?;
        let file_contents = "
author=me
name=mypackage
version=0.1.0";
        // test can only work when the file contains the text in file_contents
        assert!(tempfile.write(file_contents.as_bytes()).is_ok_and(|x| x > 0));
        let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();
        // Act
        let res = parse(&file_contents_split, None, None);
        // Assert
        assert_eq!(res.section_name, String::from("global"));
        assert_eq!(res.key_value_pair_hashmap.len(), 3);
        let key_value_pair_author = res.key_value_pair_hashmap.get_key_value(&String::from("author"));
        let key_value_pair_name = res.key_value_pair_hashmap.get_key_value(&String::from("name"));
        let key_value_pair_version = res.key_value_pair_hashmap.get_key_value(&String::from("version"));
        assert!(key_value_pair_author.is_some_and(|x| *x.0 == String::from("author") && *x.1 == String::from("me")));
        assert!(key_value_pair_name.is_some_and(|x| *x.0 == String::from("name") && *x.1 == String::from("mypackage")));
        assert!(key_value_pair_version.is_some_and(|x| *x.0 == String::from("version") && *x.1 == String::from("0.1.0")));
        assert!(res.sub_sections.is_empty());

        // Teardown
        tempfile.close()?;
        Ok(())
    }

    #[test]
    fn file_can_be_parsed_with_sections() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let mut tempfile: NamedTempFile = NamedTempFile::new()?;
        let file_contents = "
[package]
author=me
name=mypackage
version=0.1.0";
        // test can only work when the file contains the text in file_contents
        assert!(tempfile.write(file_contents.as_bytes()).is_ok_and(|x| x > 0));
        let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();
        // Act
        let res = parse(&file_contents_split, None, None);
        // Assert
        assert_eq!(res.section_name, String::from("global"));
        assert!(res.key_value_pair_hashmap.is_empty());
        assert_eq!(res.sub_sections.len(), 1);

        let package_section = res.sub_sections.get(0).unwrap();

        assert_eq!(package_section.section_name, String::from("package"));
        assert_eq!(package_section.key_value_pair_hashmap.len(), 3);
        assert!(package_section.key_value_pair_hashmap.get_key_value(&String::from("author")).is_some_and(|x| *x.0 == String::from("author") && *x.1 == String::from("me")));
        assert!(package_section.key_value_pair_hashmap.get_key_value(&String::from("name")).is_some_and(|x| *x.0 == String::from("name") && *x.1 == String::from("mypackage")));
        assert!(package_section.key_value_pair_hashmap.get_key_value(&String::from("version")).is_some_and(|x| *x.0 == String::from("version") && *x.1 == String::from("0.1.0")));
        assert!(package_section.sub_sections.is_empty());
        
        // Teardown
        tempfile.close()?;
        Ok(())
    }

    #[test]
    fn supports_sub_sections_via_dot_operator() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let mut tempfile: NamedTempFile = NamedTempFile::new()?;
        let file_contents = "
[package]
author=me
name=mypackage
version=0.1.0
[package.more]
package_size=330KiB
licence=GPLv3
";
        // test can only work when the file contains the text in file_contents
        assert!(tempfile.write(file_contents.as_bytes()).is_ok_and(|x| x > 0));
        let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();
        // Act

        let res = parse(&file_contents_split, None, None);

        // Assert
        let package_section = res.sub_sections.get(0).unwrap();

        assert_eq!(package_section.section_name, String::from("package"));
        assert_eq!(package_section.key_value_pair_hashmap.len(), 3);
        assert_eq!(package_section.sub_sections.len(), 1);

        let more_sub_section = package_section.sub_sections.get(0).unwrap();
        assert_eq!(more_sub_section.section_name, String::from("more"));
        assert_eq!(more_sub_section.key_value_pair_hashmap.len(), 2);
        assert!(more_sub_section.sub_sections.is_empty());

        tempfile.close()?;
        Ok(())
    }

    #[test]
    fn supports_sub_sections_via_dot_operator_even_when_parent_section_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {

        let mut tempfile: NamedTempFile = NamedTempFile::new()?;
            // Arrange
        let file_contents = "
[package]
author=me
name=mypackage
version=0.1.0
[info.more]
favorite_animal=monkey
favorite_colour=blue
";
        // test can only work when the file contains the text in file_contents
        assert!(tempfile.write(file_contents.as_bytes()).is_ok_and(|x| x > 0));
        let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();
        // Act
        let res = parse(&file_contents_split, None, None);

        // Assert
        let package_section = res.sub_sections.get(0).unwrap();

        assert_eq!(package_section.section_name, String::from("package"));
        assert_eq!(package_section.key_value_pair_hashmap.len(), 3);
        assert_eq!(package_section.sub_sections.len(), 1);

        let info_sub_section = package_section.sub_sections.get(0).unwrap();
        assert_eq!(info_sub_section.section_name, String::from("info"));
        assert_eq!(info_sub_section.key_value_pair_hashmap.len(), 0);

        let more_sub_section = info_sub_section.sub_sections.get(0).unwrap();
        assert_eq!(more_sub_section.section_name, String::from("more"));
        assert_eq!(more_sub_section.key_value_pair_hashmap.len(), 2);
        assert!(more_sub_section.sub_sections.is_empty());

        tempfile.close()?;
        Ok(())

    }
}
