use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};
use xml::reader::{EventReader, XmlEvent};

type TF = HashMap<String, usize>;
type Index = HashMap<PathBuf, TF>;

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let n = std::cmp::min(n, self.content.len());
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

fn read_entire_xml_file(file_path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
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

fn index_directory(dir_path: &str, tf_index: &mut Index) -> Result<(), Box<dyn std::error::Error>> {
    let dir = fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            index_directory(path.to_str().unwrap(), tf_index)?;
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext != "xhtml" && ext != "xml" && ext != "html" {
                continue;
            }
        } else {
            continue;
        }

        println!("Indexing {:?}...", path);
        let content_str = read_entire_xml_file(&path)?;
        let content = content_str.chars().collect::<Vec<_>>();

        let mut tf = TF::new();
        let mut lexer = Lexer::new(&content);

        while let Some(token_chars) = lexer.next_token() {
            let token: String = token_chars.iter().collect::<String>().to_lowercase();
            *tf.entry(token).or_insert(0) += 1;
        }

        tf_index.insert(path, tf);
    }

    Ok(())
}

fn save_index(index_path: &str, tf_index: &Index) -> Result<(), Box<dyn std::error::Error>> {
    let index_file = File::create(index_path)?;
    serde_json::to_writer(index_file, &tf_index)?;
    Ok(())
}

fn load_index(index_path: &str) -> Result<Index, Box<dyn std::error::Error>> {
    if !std::path::Path::new(index_path).exists() {
        return Ok(Index::new());
    }
    let index_file = File::open(index_path)?;
    let tf_index: Index = serde_json::from_reader(index_file)?;
    Ok(tf_index)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let index_path = "index.json";
    let mut tf_index = load_index(index_path)?;

    loop {
        print!("Search Engine > ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts[0] {
            "add" => {
                if parts.len() < 2 {
                    println!("Usage: add <folder>");
                    continue;
                }
                let folder = parts[1];
                if let Err(e) = index_directory(folder, &mut tf_index) {
                    eprintln!("Error indexing: {}", e);
                } else {
                    save_index(index_path, &tf_index)?;
                    println!("Indexed {} files.", tf_index.len());
                }
            }
            "search" => {
                if parts.len() < 2 {
                    println!("Usage: search <keyword>");
                    continue;
                }
                let keyword = parts[1].to_lowercase();
                let mut total_count = 0;
                let mut file_count = 0;

                for (_path, tf) in &tf_index {
                    if let Some(&count) = tf.get(&keyword) {
                        total_count += count;
                        file_count += 1;
                    }
                }

                println!("Found '{}' in {} files, total occurrences (content length): {}", keyword, file_count, total_count);
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}. Available: add, search, quit", parts[0]);
            }
        }
    }

    Ok(())
}
