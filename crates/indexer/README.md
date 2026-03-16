# Indexer Crate

The `indexer` crate is responsible for processing documents, tokenizing content, and discovering new links.

## Key Features

### 1. Tokenization
- Extracts words and sequences from text.
- Supports XML and HTML parsing.

### 2. Link Filtering & Discovery
- Extracts URLs from `<a>` tags in HTML documents.
- **Classification**: Differentiates between "visitable" (potential crawl targets) and "junk" (media, social media, tracking) links.
- **Normalization**: Ensures consistent URL formats (removing fragments, etc.).
- **Deduplication**: Uses a memory-efficient `u64` hash-based `HashSet` to prevent processing the same visitable URL multiple times in a session.
- **Schema-based Storage**: Saves discovered links as structured JSON objects.

## Usage

Links are processed via the `tokenizer::utils::save_url` function, which handles classification, normalization, deduplication, and storage.

```rust
use indexer::tokenizer::link_filter::classify_link;
use indexer::tokenizer::utils::save_url;

let url = "https://example.com";
let category = classify_link(url);
save_url(url, category);
```

## Storage Format

Discovered links are saved to `visitable_urls.json` or `junk_urls.json`:

```json
{
  "url": "https://example.com/page",
  "category": "visitable",
  "timestamp": 1773668325
}
```
