use indexer::tf_counter;
use indexer::html_parser::HtmlParser;
use std::fs;
use std::io::Write;

#[test]
fn test_integration_indexing() {
    let dir = std::env::temp_dir().join("integration_indexer_test_refined");
    fs::create_dir_all(&dir).unwrap();
    let file_path = dir.join("test.xml");

    let mut file = fs::File::create(&file_path).unwrap();
    write!(file, "<doc><p>integration test hello</p></doc>").unwrap();

    let parser = HtmlParser;
    let mut index = indexer::Index::new();
    tf_counter::index_directory(dir.to_str().unwrap(), &mut index, &parser).unwrap();

    assert!(index.contains_key(&file_path));
    let tf = index.get(&file_path).unwrap();
    assert_eq!(tf.get("integration"), Some(&1));

    let _ = fs::remove_dir_all(&dir);
}
