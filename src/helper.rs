use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};
use xml::reader::{EventReader, XmlEvent};


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

pub fn index_directory(
    dir_path: &str,
    tf_index: &mut crate::Index,
) -> Result<(), Box<dyn std::error::Error>> {
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

        let mut tf = crate::TF::new();
        let mut lexer = lexer::Lexer::new(&content);

        while let Some(token_chars) = lexer.next_token() {
            let token: String = token_chars.iter().collect::<String>().to_lowercase();
            *tf.entry(token).or_insert(0) += 1;
        }

        tf_index.insert(path, tf);
    }

    Ok(())
}

pub fn save_index(
    index_path: &str,
    tf_index: &crate::Index,
) -> Result<(), Box<dyn std::error::Error>> {
    let index_file = File::create(index_path)?;
    serde_json::to_writer(index_file, &tf_index)?;
    Ok(())
}

pub fn load_index(index_path: &str) -> Result<crate::Index, Box<dyn std::error::Error>> {
    if !std::path::Path::new(index_path).exists() {
        return Ok(Index::new());
    }
    let index_file = File::open(index_path)?;
    let tf_index: Index = serde_json::from_reader(index_file)?;
    Ok(tf_index)
}
