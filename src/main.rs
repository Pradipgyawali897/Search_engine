use serde_json;
#[warn(unused)]
use std::collections::HashMap;
use std::{
    fs::{self, File},
    path::PathBuf,
};
use xml::reader::{EventReader, XmlEvent};
type TF = HashMap<String, usize>;
type Index = HashMap<PathBuf, TF>;

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}
const PATH_LEN: usize = 10;

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
    let dir_path = "docs.gl/gl4";
    let dir = fs::read_dir(dir_path)?;
    let mut tf_index = Index::new();

    for file in dir {
        let file_path_buff = file?.path();
        let &file_path = &file_path_buff.to_str().unwrap();
        let content_str = read_entire_xml_file(file_path)?;
        let content = content_str.chars().collect::<Vec<_>>();

        let mut tf = TF::new();
        let mut lexer = Lexer::new(&content);

        while let Some(token_chars) = lexer.next_token() {
            let token: String = token_chars.iter().collect();
            if let Some(count) = tf.get_mut(&token) {
                *count += 1;
            } else {
                tf.insert(token, 1);
            }
        }

        let mut stats: Vec<_> = tf.into_iter().collect();
        stats.sort_by_key(|(_, v)| std::cmp::Reverse(*v));

        tf = stats.into_iter().collect();

        tf_index.insert(file_path_buff, tf);
    }

    let index_path = "index.json";
    let index_file = File::create(index_path)?;
    println!("Writing in the file path {index_path}");
    serde_json::to_writer(index_file, &tf_index).expect("Serde works fine");
    /*

    for (p, tf) in tf_index {
        println!("{:?} file has", p);

        let mut n = 0;
        for (t, f) in tf {
            println!("{t} => {f}");
            n += 1;
            if n == PATH_LEN {
                break;
            }
        }
    }
    */
    Ok(())
}
