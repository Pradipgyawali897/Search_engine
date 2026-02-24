#[warn(unused)]
use std::collections::HashMap;
use std::fs::File;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[..n];
        self.content = &self.content[n..];
        token
    }

    fn take_while<F>(&mut self, mut predicate: F) -> &'a [char]
    where
        F: FnMut(char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();

        if self.content.is_empty() {
            return None;
        }

        let first = self.content[0];
        if first.is_alphabetic() {
            Some(self.take_while(|c| c.is_alphanumeric()))
        } else if first.is_numeric() {
            Some(self.take_while(|c| c.is_numeric()))
        } else {
            Some(self.chop(1))
        }
    }

    fn trim_left(&mut self) {
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }
}

fn read_entire_xml_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();

    for event in er {
        match event? {
            XmlEvent::Characters(text) => content.push_str(&text),
            _ => {}
        }
    }

    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "docs.gl/gl4/glVertexAttribDivisor.xhtml";
    let content_str = read_entire_xml_file(file_path)?;
    let content = content_str.chars().collect::<Vec<_>>();

    let mut lexer = Lexer::new(&content);

    while let Some(token_chars) = lexer.next_token() {
        let token: String = token_chars.iter().collect();
        println!("{}", token);
    }

    Ok(())
}
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
