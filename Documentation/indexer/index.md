# Indexer Subsystem

The indexer transforms raw HTML into a structured searchable index.

## Pipeline Flow

1. **HTML Parser**: Strips tags and extracts meaningful text content.
2. **Tokenizer**: Splits text into atomic tokens based on alphanumeric boundaries.
3. **TF Computation**: Calculates Term Frequency (TF) for each token within the document.

## Storage Engine

The index is persisted as a JSON file (`index.json`) mapping document paths to their respective TF maps.
