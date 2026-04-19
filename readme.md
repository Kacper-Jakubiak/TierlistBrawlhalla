# Brawlhalla Tierlist Creator

A Rust application that generates a Brawlhalla legend tierlist as an image.

## Overview

App reads data from JSON files and creates a visual tierlist image ranking Brawlhalla legends by tier (S, A, B, C). If image creation fails, it falls back to displaying the tierlist in the console.

## Prerequisites

- Rust 1.70+
- Cargo

## Running the Project

1. Clone the repository
2. Navigate to the project directory
3. Run the application:

```bash
cargo run
```

The generated tierlist image will be saved to the output directory (or printed to console if image generation fails).

## Dependencies

- **serde_json**: JSON parsing and handling
- **image**: Image creation and processing
- **reqwest**: HTTP requests
- **scraper**: HTML scraping
- **log & env_logger**: Logging

## Configuration

Set the `RUST_LOG` environment variable to control logging verbosity:

```bash
RUST_LOG=info cargo run
```
