# Indexer Subsystem

The indexer transforms raw HTML into a structured searchable index.

## Pipeline Flow

1. **HTML Parser**: Strips tags and extracts meaningful text content. It extracts URLs from `<a>` tags and passes them to the link discovery system.
2. **Tokenizer**: Splits text into atomic tokens based on alphanumeric boundaries.
3. **Link Discovery**: Automatically classifies discovered URLs as "visitable" or "junk", normalizes them, and saves them to categorized JSON files.
4. **TF Computation**: Calculates Term Frequency (TF) for each token within the document.

## Storage Engine

The index is persisted as a JSON file (`index.json`) mapping document paths to their respective TF maps. Discovered links are stored separately in `visitable_urls.json` and `junk_urls.json`.

## Functional Utilities

The `tokenizer` module provides high-level utilities for link processing:

- `tokenizer::utils::save_url(url, category)`: Processes a URL through normalization and deduplication before saving.
- `tokenizer::link_filter::classify_link(url)`: Categorizes a URL as `Visitable` or `Junk`.

For more details on how links are processed, see [Link Discovery & Filtering](link_discovery.md).
