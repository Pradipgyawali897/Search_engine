use core::panic;
use std::collections::HashMap;
use std::fs::{self, ReadDir, read_dir};
#[warn(unused)]
use std::path::Path;
use std::{fs::File, io};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content, cursor: 0 }
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }
        todo!("not implemented");
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }
}

fn index(content: &str) -> HashMap<String, usize> {
    todo!("Hashmap to make ");
}

fn read_entire_xml_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();

    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event? {
            content.push_str(&text);
        }
    }

    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let all_docs=HashMap::<Path,HashMap<String,usize>>>::new();

    let content = read_entire_xml_file("docs.gl/gl4/glVertexAttribDivisor.xhtml")?
        .chars()
        .collect::<Vec<_>>();

    let lexer = Lexer::new(&content);
    println!("{:?}", lexer);
    /*
        let dir_path = "docs.gl/gl4";

        let dir = fs::read_dir(dir_path).unwrap_or_else(|err| {
            panic!("Error reading directory {}: {}", dir_path, err);
        });

        for entry in dir {
            let entry = entry?;
            let path_buff = entry.path();
            let file_path = path_buff.to_str().unwrap();
            let content = read_entire_xml_file(&file_path).unwrap_or_else(|err| {
                panic!("Failed to read XML file {}: {}", file_path, err);
            });

            println!("{file_path:?} => size: {}", content.len());
        }
    */
    Ok(())
}
