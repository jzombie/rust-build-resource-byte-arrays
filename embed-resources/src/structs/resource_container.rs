use crate::Resource;
use std::io;
use std::path::Path;

pub struct ResourceContainer {
    resources: Vec<(String, Resource)>, // (Name, Resource)
}

impl ResourceContainer {
    /// Creates a new empty container.
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }

    /// Adds a resource to the container.
    pub fn add_resource(&mut self, name: &str, resource: Resource) {
        self.resources.push((name.to_string(), resource));
    }

    /// Processes all resources and writes them to the embed directory.
    pub fn embed_all(&self, output_path: &Path, compress: bool) -> io::Result<()> {
        let mut byte_arrays = Vec::new();

        for (name, resource) in &self.resources {
            let bytes = resource.fetch(compress)?;
            byte_arrays.push((name.as_str(), bytes));
        }

        // Use `embed-bytes` to write the byte arrays
        embed_bytes::write_byte_arrays(output_path, byte_arrays)
    }
}
