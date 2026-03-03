use std::fs;

use crate::crawler::seed::manager::SeedManager;

pub fn consume_seeds_from_file() -> Vec<String> {
    let content = fs::read_to_string("seed.txt").unwrap_or_else(|err| {
        eprintln!("error reading the seeds {err}");
        String::new()
    });

    let frountier: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    frountier
}

pub fn create_seed() -> SeedManager {
    let string_file_seed = consume_seeds_from_file();
    let file_seed = string_file_seed.iter().map(|s| s.as_str()).collect();
    let seed = SeedManager::new(file_seed);
    return seed;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn consume_seeds_from_temp_file() {
        let dir = std::env::temp_dir().join("search_engine_loader_test");
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("seed.txt");

        let mut file = std::fs::File::create(&file_path).unwrap();
        write!(file, "http://google.com\nhttp://example.com\n").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let seeds: Vec<String> = content.lines().map(|l| l.to_string()).collect();

        assert_eq!(seeds.len(), 2);
        assert_eq!(seeds[0], "http://google.com");
        assert_eq!(seeds[1], "http://example.com");

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
