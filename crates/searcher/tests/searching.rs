use searcher;
use indexer::{Index, TF};
use std::path::PathBuf;

#[test]
fn test_integration_searching() {
    let mut index = Index::new();
    let mut tf = TF::new();
    tf.insert("rust".to_string(), 5);
    index.insert(PathBuf::from("manual.xml"), tf);

    let (file_count, total_count) = searcher::find_occurrences("rust", &index);
    assert_eq!(file_count, 1);
    assert_eq!(total_count, 5);
}
