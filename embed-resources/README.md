# embed-resources (Work in Progress)

`embed-resources` is a Rust crate built on top of [`embed-bytes`](https://crates.io/crates/embed-bytes) that extends its functionality to handle multiple resource types: local files, URLs, and in-memory data. It also supports optional Gzipped compression for reduced storage size.

Unlike `embed-bytes`, which directly embeds assets, `embed-resources` simplifies managing mixed resources by organizing them in a virtual "container." Resources are automatically converted into `.bin` files, and a Rust source file is generated to embed them using `include_bytes!`.

### Key Features:
- **Multi-Type Resource Support**: Embed local files, fetch remote data, or embed arbitrary in-memory data.
- **Optional Compression**: Compress resources with Gzipped encoding during the build process.
- **Unified Management**: Organize resources into a container with minimal setup while benefiting from zero-cost runtime embedding.

### Example Usage:
In `build.rs`:
```rust
use embed_resources::{Resource, ResourceContainer};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut container = ResourceContainer::new(Path::new("embed"));

    container.add_resource("local_file", Resource::File("assets/file.txt".to_string()), true);
    container.add_resource("remote_data", Resource::Url("https://example.com/data.bin".to_string()), true);
    container.add_resource("in_memory_data", Resource::Data(bytes::Bytes::from("Hello, world!")), true);

    container.embed_all()?;

    Ok(())
}
```

This creates `.bin` files and a Rust source file in the `embed/` directory, ready to use with `include_bytes!`.

## License

[MIT License](LICENSE) (c) 2025 Jeremy Harris.
