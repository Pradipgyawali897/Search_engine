# Searcher Subsystem

The searcher is a lightweight query engine designed for fast retrieval of document occurrences.

## Query Handling

The searcher scans the `index.json` to identify documents where the searched term appears. It currently supports exact keyword matching.

## Performance

By leveraging Rust's `HashMap` implementation, lookups are optimized for O(1) average time complexity per token per document.
