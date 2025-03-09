#![feature(try_find)]
use std::fs::{self};
use std::cell::RefCell;
use std::ops::Deref;

use clap::{crate_authors, crate_version, value_parser, Arg, ArgMatches, Command};
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


fn parse_sections(file_contents: &[&str], this_section_name: Option<String>) -> Section {
    let mut section: Section = Section::new(this_section_name.unwrap_or(String::from("global")), RefCell::new(vec![]), HashMap::new());

    for file_contents_line in file_contents {
        let section_names = file_contents_line.strip_prefix('[').unwrap_or(&file_contents_line).strip_suffix(']').unwrap_or(&file_contents_line).split('.').filter(|x| *x != "").collect::<Vec<&str>>();
        if section_names.is_empty() {
            continue;
        }
        println!("section_name: {:#?}", section_names);
        for section_name in &section_names {
            if let Some(res) = section.sub_sections.get_mut().into_iter().filter(|x| section_names.iter().any(|y| x.section_name == *y)).last() { // add it to the section if that section already exists//include all of section_names in search
                section = res.clone();
            }
            else {
                println!("in here!");
                let new_section = Section::new(String::from(*section_name), RefCell::new(vec![]), HashMap::new());
                section.sub_sections.get_mut().push(new_section);
            }
        }
    }
    return section;
}

fn parse_sections_2(file_contents: &[&str], this_section_name: Option<String>) -> Section {
    let mut section: Section = Section::new(this_section_name.unwrap_or(String::from("global")), RefCell::new(vec![]), HashMap::new());
    for file_contents_line in file_contents {
        let section_names = file_contents_line.strip_prefix('[').unwrap_or(&file_contents_line).strip_suffix(']').unwrap_or(&file_contents_line).split('.').filter(|x| *x != "").collect::<Vec<&str>>();
        let iter = section.sub_sections.get_mut().into_iter();
        println!("hi");
        for section_name in section_names {

        }
        // if let Some(x) = iter.filter(|x| section_names.iter().any(|y| *y == x.section_name)).collect::<Vec<_>>().get(0) {
            // println!("found section_name: {} in section_names", x.section_name);
        // }
        // else {
            // section.sub_sections.get_mut().push(Section::new(String::from()));
        // }
    }
    section
}

fn traverse_sections_to_leaf<'a>(section: &'a mut Section, section_names: Vec<&'a str>) -> (&'a mut Section, Vec<&'a str>) {
    if let Some(section_name) = section_names.first() {
        if let Some(_) = section.sub_sections.get_mut().into_iter().filter(|x| x.section_name == *section_name).collect::<Vec<_>>().get(0) {
            // We have found an already existing section, so traverse
            return traverse_sections_to_leaf(section, section_names[1..].to_vec());
        }
        else {
            // We have found the leaf node
            return (section, vec![]);
        }
    }
    else {
        return (section, section_names);
    }

}

fn add_sections<'a>(mut section: &'a mut Section, section_names: Vec<&'a str>) -> &'a Section {
    for section_name in &section_names {
        let mut section_2 = Section::new(section_name.to_string(), RefCell::new(vec![]), HashMap::new());
        section.sub_sections.get_mut().push(section_2.clone());
        add_sections(&mut section_2, section_names[1..].to_vec());

    }
    section
}

fn add_sections_2<'a>(mut section: &Section, section_names: Vec<&'a str>) -> Section {
    for section_name in &section_names {
        let b = Section::new(section_name.to_string(), RefCell::new(vec![]), HashMap::new());
        section.sub_sections.get_mut().push(b);
        let mut c = section.sub_sections.get_mut().last().unwrap();
        add_sections_2(c, section_names[1..].to_vec());
    }
    section.clone()
}



fn parse(file_contents: &[&str], seek_to_column_in_line: Option<usize>, section_name: Option<String>) -> Section {
    let mut section_to_add: Section = Section::new(section_name.unwrap_or(String::from("global")), RefCell::new(vec![]), HashMap::new());
    let mut line_number = 0;
    for file_contents_line in file_contents {
        let line_end: usize;
        if let Some(i) = file_contents_line.chars().position(|x| x == '#') {
            line_end = i;
        }
        else {
            line_end = file_contents_line.len();
        }
        if let Some(i) = file_contents_line.chars().position(|x| x == '=') {
            let _ = section_to_add.key_value_pair_hashmap.insert(String::from(file_contents_line[..i].trim()), String::from(file_contents_line[i+1..line_end].trim())); 
        }
        else if file_contents_line.trim_start().starts_with('[') && file_contents_line.trim_end().ends_with(']') {
            let file_contents_line_as_char_vec: Vec<char> = file_contents_line.chars().collect();
            if let Some(location_of_dot_separator) = file_contents_line_as_char_vec[seek_to_column_in_line.unwrap_or(0)..].iter().position(|x| *x == '.') {

            }
            else {

            }



            let file_contents_line_as_char_vec: Vec<char> = file_contents_line.chars().collect();
            if let Some(location_of_dot_separator) = file_contents_line_as_char_vec[seek_to_column_in_line.unwrap_or(1)..].iter().position(|x| *x == '.') {
                println!("new");
                println!("{:#?}", seek_to_column_in_line);
                println!("{}", location_of_dot_separator + 1);
                println!("{}", &file_contents_line);
                let section_name = String::from(&file_contents_line[seek_to_column_in_line.unwrap_or(1)..seek_to_column_in_line.unwrap_or(1) + location_of_dot_separator]);
                println!("{}", section_name);
                let lines_remaining = file_contents.get((line_number)..);
                section_to_add.sub_sections.get_mut().push(parse(lines_remaining.unwrap(), Some(location_of_dot_separator + 1), Some(section_name)));
            }
            else {
                let section_name = String::from(&file_contents_line[1..file_contents_line.len() - 1]);
                let lines_remaining = file_contents.get((line_number + 1)..);
                if lines_remaining.is_some() {
                    section_to_add.sub_sections.get_mut().push(parse(lines_remaining.unwrap(), None, Some(section_name)));
                }
            }
            return section_to_add;
        }
        line_number += 1;
    }
    section_to_add
}

#[derive(Debug, Clone, PartialEq)]
struct Section {
    section_name: String,
    sub_sections: RefCell<Vec<Section>>,
    key_value_pair_hashmap: HashMap<String, String>
}
impl Section {
    fn new(section_name: String, sub_sections: RefCell<Vec<Section>>, key_value_pair_hashmap: HashMap<String, String>) -> Section {
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
        let mut res = parse(&file_contents_split, None, None);
        // Assert
        assert_eq!(res.section_name, String::from("global"));
        assert_eq!(res.key_value_pair_hashmap.len(), 3);
        let key_value_pair_author = res.key_value_pair_hashmap.get_key_value(&String::from("author"));
        let key_value_pair_name = res.key_value_pair_hashmap.get_key_value(&String::from("name"));
        let key_value_pair_version = res.key_value_pair_hashmap.get_key_value(&String::from("version"));
        assert!(key_value_pair_author.is_some_and(|x| *x.0 == String::from("author") && *x.1 == String::from("me")));
        assert!(key_value_pair_name.is_some_and(|x| *x.0 == String::from("name") && *x.1 == String::from("mypackage")));
        assert!(key_value_pair_version.is_some_and(|x| *x.0 == String::from("version") && *x.1 == String::from("0.1.0")));
        assert!(res.sub_sections.get_mut().is_empty());

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
        let mut res = parse(&file_contents_split, None, None);
        // Assert
        assert_eq!(res.section_name, String::from("global"));
        assert!(res.key_value_pair_hashmap.is_empty());
        assert_eq!(res.sub_sections.get_mut().len(), 1);

        let package_section = res.sub_sections.get_mut().get(0).unwrap();

        assert_eq!(package_section.section_name, String::from("package"));
        assert_eq!(package_section.key_value_pair_hashmap.len(), 3);
        assert!(package_section.key_value_pair_hashmap.get_key_value(&String::from("author")).is_some_and(|x| *x.0 == String::from("author") && *x.1 == String::from("me")));
        assert!(package_section.key_value_pair_hashmap.get_key_value(&String::from("name")).is_some_and(|x| *x.0 == String::from("name") && *x.1 == String::from("mypackage")));
        assert!(package_section.key_value_pair_hashmap.get_key_value(&String::from("version")).is_some_and(|x| *x.0 == String::from("version") && *x.1 == String::from("0.1.0")));
        // assert!(package_section.sub_sections.get_mut().is_empty());
        
        // Teardown
        tempfile.close()?;
        Ok(())
    }

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

        let mut res = parse(&file_contents_split, None, None);

        // Assert
        let package_section: &Section = res.sub_sections.get_mut().get(0).unwrap();

        assert_eq!(package_section.section_name, String::from("package"));
        assert_eq!(package_section.key_value_pair_hashmap.len(), 3);
        // assert_eq!(package_section.sub_sections.get_mut().len(), 1);

        // todo add this back in
        // let more_sub_section = package_section.sub_sections.get_mut().get(0).unwrap();
        // assert_eq!(more_sub_section.section_name, String::from("more"));
        // assert_eq!(more_sub_section.key_value_pair_hashmap.len(), 2);
        // assert!(more_sub_section.sub_sections.get_mut().is_empty());

        tempfile.close()?;
        Ok(())
    }

    fn supports_sub_sections_via_dot_operator_even_when_parent_section_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {

        // Arrange
        let mut tempfile: NamedTempFile = NamedTempFile::new()?;
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
        let mut res = parse(&file_contents_split, None, None);

        // Assert
        let package_section: &mut Section = res.sub_sections.get_mut().get_mut(0).unwrap();

        assert_eq!(package_section.section_name, String::from("package"));
        assert_eq!(package_section.key_value_pair_hashmap.len(), 3);
        assert_eq!(package_section.sub_sections.get_mut().len(), 1);

        let info_sub_section = package_section.sub_sections.get_mut().get_mut(0).unwrap();
        assert_eq!(info_sub_section.section_name, String::from("info"));
        assert_eq!(info_sub_section.key_value_pair_hashmap.len(), 0);

        let more_sub_section = info_sub_section.sub_sections.get_mut().get_mut(0).unwrap();
        assert_eq!(more_sub_section.section_name, String::from("more"));
        assert_eq!(more_sub_section.key_value_pair_hashmap.len(), 2);
        assert!(more_sub_section.sub_sections.get_mut().is_empty());

        tempfile.close()?;
        Ok(())

    }
    #[test]
    fn comments_are_ignored() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let mut tempfile: NamedTempFile = NamedTempFile::new()?;
        let file_contents = "
[package]
author=me
name=mypackage#this is my package
version=0.1.0
";
        assert!(tempfile.write(file_contents.as_bytes()).is_ok_and(|x| x > 0));
        let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();
        // Act
        let mut res = parse(&file_contents_split, None, None);

        // Assert
        let package_section = res.sub_sections.get_mut().get_mut(0).unwrap();
        assert_eq!(package_section.section_name, String::from("package"));
        assert!(package_section.key_value_pair_hashmap.get_key_value(&String::from("name")).is_some_and(|x| *x.0 == String::from("name") && *x.1 == String::from("mypackage")));
        assert!(package_section.key_value_pair_hashmap.is_empty());
        Ok(())
    }

    #[test]
    fn parse_sections() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let mut tempfile: NamedTempFile = NamedTempFile::new()?;
        let file_contents = "
[package]
[package.b]
[package.a]
";
        assert!(tempfile.write(file_contents.as_bytes()).is_ok_and(|x| x > 0));
        let file_contents_split = file_contents.split('\n').collect::<Vec<_>>();

        // Act
        let mut res = super::parse_sections_2(&file_contents_split, None);
        
        // Assert
        println!("state of res: {:#?}", res);
        assert_eq!(res.sub_sections.get_mut().len(), 1);
        let package_section: &mut Section = res.sub_sections.get_mut().get_mut(0).unwrap();
        assert_eq!(package_section.section_name, String::from("package"));
        assert_eq!(package_section.sub_sections.get_mut().len(), 2);
        let a = package_section.sub_sections.get_mut().get_mut(0).unwrap();
        assert_eq!(a.section_name, String::from("a"));
        let b = package_section.sub_sections.get_mut().get_mut(1).unwrap();
        assert_eq!(b.section_name, String::from("b"));

        // Teardown
        Ok(())
    }

    #[test]
    fn traverse_sections_to_leaf() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let section_names = vec!["package", "a", "b"];
        let mut section: Section = Section::new(String::from("global"), RefCell::new(vec![]), HashMap::new());
        
        // Act
        let (section, _) = super::traverse_sections_to_leaf(&mut section, section_names);

        println!("{:#?}", *section);
        // Assert
        Ok(())
    }

    #[test]
    fn traverse_sections_to_leaf_2() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let section_names = vec!["package", "a", "b"];
        let mut section: Section = Section::new(String::from("package"), RefCell::new(vec![Section::new(String::from("a"), RefCell::new(vec![]), HashMap::new())]), HashMap::new());
        
        // Act
        let (section, _) = super::traverse_sections_to_leaf(&mut section, section_names);

        println!("{:#?}", *section);
        panic!("a");
        // Assert
        Ok(())
    }

    #[test]
    fn add_sections() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let mut section = Section::new(String::from("package"), RefCell::new(vec![]), HashMap::new());
        

        // Act
        let res = super::add_sections(&mut section, vec!["a", "b"]);

        println!("output: {:#?}", *res);
        panic!("b");

        Ok(())
    }
}
