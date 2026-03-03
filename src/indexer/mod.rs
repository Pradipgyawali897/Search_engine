pub mod storage;

use crate::lexer;
use crate::parser::Parser;
use std::fs;
use std::path::PathBuf;

pub fn index_file(
    path: &PathBuf,
    parser: &dyn Parser,
) -> Result<crate::TF, Box<dyn std::error::Error>> {
    let content_str = parser.parse(path)?;
    let content: Vec<char> = content_str.chars().collect();

    let mut tf = crate::TF::new();
    let mut lexer = lexer::Lexer::new(&content);

    while let Some(token_chars) = lexer.next_token() {
        let token: String = token_chars.iter().collect::<String>().to_lowercase();
        *tf.entry(token).or_insert(0) += 1;
    }

    Ok(tf)
}

pub fn index_directory(
    dir_path: &str,
    tf_index: &mut crate::Index,
    parser: &dyn Parser,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            index_directory(path.to_str().unwrap(), tf_index, parser)?;
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
        let tf = index_file(&path, parser)?;
        tf_index.insert(path, tf);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::xml::XmlParser;
    use std::io::Write;

    #[test]
    fn index_file_counts_tokens() {
        let dir = std::env::temp_dir().join("search_engine_indexer_test");
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.xml");

        let mut file = std::fs::File::create(&file_path).unwrap();
        write!(file, "<doc><p>hello hello world</p></doc>").unwrap();

        let parser = XmlParser;
        let tf = index_file(&file_path, &parser).unwrap();

        assert_eq!(tf.get("hello"), Some(&2));
        assert_eq!(tf.get("world"), Some(&1));

        std::fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn index_directory_skips_non_xml() {
        let dir = std::env::temp_dir().join("search_engine_indexer_dir_test");
        std::fs::create_dir_all(&dir).unwrap();

        let txt_path = dir.join("readme.txt");
        std::fs::File::create(&txt_path).unwrap();

        let xml_path = dir.join("doc.xml");
        let mut file = std::fs::File::create(&xml_path).unwrap();
        write!(file, "<doc><p>test</p></doc>").unwrap();

        let parser = XmlParser;
        let mut index = crate::Index::new();
        index_directory(dir.to_str().unwrap(), &mut index, &parser).unwrap();

        assert_eq!(index.len(), 1);
        assert!(index.contains_key(&xml_path));

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
