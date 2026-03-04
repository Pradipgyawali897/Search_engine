use std::fs::File;
use crate::Index;

pub fn save_index(
    index_path: &str,
    tf_index: &Index,
) -> Result<(), Box<dyn std::error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::TF;

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join("indexer_storage_test");
        std::fs::create_dir_all(&dir).unwrap();
        let index_path = dir.join("test_index.json");
        let index_str = index_path.to_str().unwrap();

        let mut index = Index::new();
        let mut tf = TF::new();
        tf.insert("hello".to_string(), 3);
        tf.insert("world".to_string(), 1);
        index.insert(PathBuf::from("test.xml"), tf);

        save_index(index_str, &index).unwrap();
        let loaded = load_index(index_str).unwrap();

        assert_eq!(loaded.len(), 1);
        let loaded_tf = loaded.get(&PathBuf::from("test.xml")).unwrap();
        assert_eq!(loaded_tf.get("hello"), Some(&3));
        assert_eq!(loaded_tf.get("world"), Some(&1));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
