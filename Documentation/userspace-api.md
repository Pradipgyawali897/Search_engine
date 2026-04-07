# Pernox User Space API

The Pernox kernel exposes its functionality through the following internal APIs (for library use) and CLI commands.

## CLI Usage

The `app` crate provides the primary interface:

```bash
cargo run -p app
```

## Library API (Rust)

Other crates can consume the following core functionalities:

- `spyder::get_robot_content(url)`: Fetches robots.txt rules.
- `indexer::index_file(url, parser)`: Runs the indexer pipeline for a single resource.
- `searcher::find_occurrences(index, query)`: Search for a term in the index.
