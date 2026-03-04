use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::{Index, TF};
use std::fs;
use std::path::PathBuf;

pub fn index_file(
    path: &PathBuf,
    parser: &dyn Parser,
) -> Result<TF, Box<dyn std::error::Error>> {
    let content_str = parser.parse(path)?;
    let content: Vec<char> = content_str.chars().collect();

    let mut tf = TF::new();
    let mut lexer = Lexer::new(&content);

    while let Some(token_chars) = lexer.next_token() {
        let token: String = token_chars.iter().collect::<String>().to_lowercase();
        *tf.entry(token).or_insert(0) += 1;
    }

    Ok(tf)
}

pub fn index_directory(
    dir_path: &str,
    tf_index: &mut Index,
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
            let ext = ext.to_string_lossy().to_lowercase();
            if ext != "xhtml" && ext != "xml" && ext != "html" {
                continue;
            }
        } else {
            continue;
        }

        let tf = index_file(&path, parser)?;
        tf_index.insert(path, tf);
    }

    Ok(())
}
