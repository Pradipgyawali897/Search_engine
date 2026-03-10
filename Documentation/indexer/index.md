# Indexer Subsystem

The indexer transforms raw HTML into a structured searchable index.

## Pipeline Flow

1. **HTML Parser**: Strips tags and extracts meaningful text content.
2. **Tokenizer**: Splits text into atomic tokens based on alphanumeric boundaries. It also automatically detects URLs (starting with `http://`, `https://`, or `www.`) and saves them to `discovered_urls.txt` for further processing.
3. **TF Computation**: Calculates Term Frequency (TF) for each token within the document.

## Storage Engine

The index is persisted as a JSON file (`index.json`) mapping document paths to their respective TF maps.

## Functional Utilities

The `tokenizer` module also provides functional equivalents for common tasks:

- `tokenizer::tokenize(content)`: Returns a `Vec<String>` of all tokens in a string.
- `tokenizer::extract_urls(content)`: Specifically identifies and returns a `Vec<String>` of all URLs found in the content, while also updating `discovered_urls.txt`.
