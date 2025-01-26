use embed_resources::{Resource, ResourceContainer};
use std::path::Path;

// Run with: cargo run -p embed-resources --example container
fn main() -> std::io::Result<()> {
    let mut container = ResourceContainer::new(Path::new("embed"));

    // Add resources from different sources
    container.add_resource("local_file", Resource::File("Cargo.toml".to_string()), true);
    container.add_resource(
        "remote_file",
        Resource::Url("https://zenosmosis.com".to_string()),
        true,
    );
    container.add_resource(
        "arbitrary_data",
        Resource::Data(bytes::Bytes::from("Hello, world!")),
        true,
    );

    // Embed all resources into the "embed" directory with compression
    container.embed_all()?;

    Ok(())
}
