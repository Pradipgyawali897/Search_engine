use std::fs::File;
use crate::Index;

pub fn save_index(index_path: &str, tf_index: &Index) -> Result<(), Box<dyn std::error::Error>> {
    let index_file = File::create(index_path)?;
    serde_json::to_writer(index_file, &tf_index)?;
    Ok(())
}

pub fn load_index(index_path: &str) -> Result<Index, Box<dyn std::error::Error>> {
    if !std::path::Path::new(index_path).exists() {
        return Ok(Index::new());
    }
    let index_file = File::open(index_path)?;
    let tf_index: Index = serde_json::from_reader(index_file)?;
    Ok(tf_index)
}
