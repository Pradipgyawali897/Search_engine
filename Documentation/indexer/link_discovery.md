# Link Discovery & Filtering

The indexer includes a robust system for discovering and processing links found during the indexing process.

## Pipeline Overview

When a parser returns discovered links, the `discovery` module processes them in one place. The process follows these steps:

1. **Classification**: The URL is classified as either `Visitable` or `Junk`.
2. **Normalization**: URLs are canonicalized first so classification and deduplication run against a stable representation.
3. **Deduplication**: `Visitable` URLs are checked against an in-memory `HashSet` of hashes to prevent redundant processing.
4. **Storage**: The link is written to the appropriate persistence file.

## Classification Logic

The classification logic identifies "junk" links to keep the crawl frontier clean.

- **Junk Criteria**:
    - Disallowed extensions: `.jpg`, `.pdf`, `.zip`, `.css`, `.js`, etc.
    - Known tracking/social domains: Facebook, Google Analytics, etc.

## Memory-Efficient Deduplication

To minimize RAM usage while handling millions of URLs, the system:
- Generates a **64-bit hash** (using Rust's `DefaultHasher`) for each normalized URL.
- Stores only the `u64` hash in an internal `HashSet` protected by a `Mutex`.
- This avoids storing massive URL strings in memory across the entire indexing session.

## Storage Schema

Links are saved as `DiscoveredLink` JSON objects:

```json
{
  "url": "https://example.com/page",
  "category": "visitable",
  "timestamp": 1773668325
}
```

- **Files**:
    - `visitable_urls.txt`: Canonical visitable URLs ready for the next crawl or index pass.
    - `junk_urls.json`: Filtered links kept for analysis or audit.
