# Crawler Subsystem

The crawler is responsible for the discovery and retrieval of web assets.

## Core Modules

### 1. Frontier Manager
Manages the priority queue of URLs. It uses a `HashSet` to ensure unique traversal (visited URLs are never re-queued).

### 2. Robots Parser
Implements the Robots Exclusion Protocol. It fetches `/robots.txt` from every discovery target to ensure legal and ethical crawling.

### 3. URL Normalizer
Sanitizes discovered links to prevent infinite loops and duplicate indexing (e.g., removing fragments and handling trailing slashes).

## DNS Resolution
The crawler includes a custom DNS resolver kernel to map hostnames to IP addresses before initiating TCP handshakes.
