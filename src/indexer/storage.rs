use std::fs::File;

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
        return Ok(crate::Index::new());
    }
    let index_file = File::open(index_path)?;
    let tf_index: crate::Index = serde_json::from_reader(index_file)?;
    Ok(tf_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join("search_engine_storage_test");
        std::fs::create_dir_all(&dir).unwrap();
        let index_path = dir.join("test_index.json");
        let index_str = index_path.to_str().unwrap();

        let mut index = crate::Index::new();
        let mut tf = crate::TF::new();
        tf.insert("hello".to_string(), 3);
        tf.insert("world".to_string(), 1);
        index.insert(std::path::PathBuf::from("test.xml"), tf);

        save_index(index_str, &index).unwrap();
        let loaded = load_index(index_str).unwrap();

        assert_eq!(loaded.len(), 1);
        let loaded_tf = loaded.get(&std::path::PathBuf::from("test.xml")).unwrap();
        assert_eq!(loaded_tf.get("hello"), Some(&3));
        assert_eq!(loaded_tf.get("world"), Some(&1));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_missing_file_returns_empty() {
        let result = load_index("/nonexistent/path/index.json").unwrap();
        assert!(result.is_empty());
    }
}
