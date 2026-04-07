# Indexer Crate

The `indexer` crate now follows a clearer four-stage pipeline:

1. `parser`: fetches a source and returns a structured `ParsedDocument`.
2. `discovery`: normalizes, classifies, deduplicates, and persists discovered links.
3. `tokenizer`: performs pure text scanning and normalization.
4. `indexing`: builds term-frequency maps from parsed document text.

## Main Components

- `document::ParsedDocument`: shared parser output containing `text` and discovered `links`.
- `parser::{HtmlParser, XmlParser}`: source-specific parsers that populate `ParsedDocument`.
- `discovery::process_link`: centralized link handling for crawl frontier updates.
- `indexing::index_file`: end-to-end indexing entry point for a single resource.

## Storage Format

- `index.json`: persisted term-frequency index.
- `visitable_urls.txt`: canonical visitable URLs for later crawl/index passes.
- `junk_urls.json`: structured records for filtered links.

The exact paths for those files are centralized in
`crates/indexer/src/config.rs` and can be overridden with environment
variables at runtime.

Example junk record:

```json
{
  "url": "https://example.com/asset.js",
  "category": "junk",
  "timestamp": 1773668325
}
```
