use embed_resources::{Resource, ResourceContainer};
use std::path::Path;

// Run with: cargo run -p embed-resources --example container
fn main() -> std::io::Result<()> {
    let mut container = ResourceContainer::new();

    // Add resources from different sources
    container.add_resource("local_file", Resource::File("Cargo.toml".to_string()));
    container.add_resource(
        "remote_file",
        Resource::Url("https://zenosmosis.com".to_string()),
    );
    container.add_resource(
        "arbitrary_data",
        Resource::Data(bytes::Bytes::from("Hello, world!")),
    );

    // Embed all resources into the "embed" directory with compression
    container.embed_all(Path::new("embed"), true)?;

    Ok(())
}
