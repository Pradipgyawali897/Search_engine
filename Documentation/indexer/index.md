# Indexer Subsystem

The indexer transforms raw HTML into a structured searchable index.

## Pipeline Flow

1. **Parser**: Produces a `ParsedDocument` with cleaned text plus discovered links.
2. **Link Discovery**: Canonicalizes, classifies, deduplicates, and persists links in one module.
3. **Tokenizer**: Scans text into raw tokens and normalizes them into index terms.
4. **TF Computation**: Builds the final term-frequency map from the normalized token stream.

## Storage Engine

The index is persisted as a JSON file (`index.json`) mapping document paths to their respective TF maps. Discovered links are stored separately in `visitable_urls.txt` and `junk_urls.json`.

## Functional Utilities

The key crate entry points are:

- `indexer::index_file(domain, parser)`: Full parse-discover-tokenize-index pipeline.
- `indexer::load_visited_urls()`: Preloads already-seen visitable URLs into the in-memory dedupe set.
- `indexer::discovery::process_link(url)`: Processes one discovered URL through normalization, classification, and persistence.

For more details on how links are processed, see [Link Discovery & Filtering](link_discovery.md).
