# Rust Embed Bytes Utilities Workspace

A Rust workspace containing two related crates for embedding resources in Rust applications.

## Crates

1. **[`embed-bytes`](embed-bytes/)**  
  This crate is designed for use in the `build.rs` file of a Rust project, simplifying the embedding and management of byte arrays.  It automatically generates a directory of `.bin` files along with a corresponding Rust file, named to match the directory, which includes `include_bytes!` macro references for each .bin file.

    When used as a `build.rs` preprocessor, it introduces zero-cost abstractions into the resulting binary.

2. **[`embed-resources`](embed-resources/)**  
  A higher-level crate built on top of `embed-bytes`, extending functionality to include embedding files, URLs, and in-memory data with optional compression.

    Like `embed-bytes`, it operates as a `build.rs` preprocessor to embed resources as zero-cost abstractions in the final binary.

## Usage

Refer to the individual crate directories for more details and examples.  
