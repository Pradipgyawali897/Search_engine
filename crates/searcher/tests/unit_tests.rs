use searcher::find_occurrences;
use indexer::{Index, TF};

#[test]
fn test_search_basic() {
    let mut index = Index::new();
    let mut tf = TF::new();
    tf.insert("rust".to_string(), 5);
    index.insert("test.xml".into(), tf);

    let (file_count, total_count) = find_occurrences("rust", &index);
    assert_eq!(file_count, 1);
    assert_eq!(total_count, 5);
}
