use indexer::Index;

pub mod server;

pub fn find_occurrences(keyword: &str, index: &Index) -> (usize, usize) {
    let keyword = keyword.to_lowercase();
    let mut total_count = 0;
    let mut file_count = 0;

    for (_path, tf) in index {
        if let Some(&count) = tf.get(&keyword) {
            total_count += count;
            file_count += 1;
        }
    }

    (file_count, total_count)
}
