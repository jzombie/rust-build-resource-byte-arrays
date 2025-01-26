use crate::Resource;
use std::io;
use std::path::Path;

pub struct ResourceContainer<'a> {
    output_path: &'a Path,
    resources: Vec<(String, Resource, bool)>, // (Name, Resource, Compress)
}

impl<'a> ResourceContainer<'a> {
    /// Creates a new empty container.
    pub fn new(output_path: &'a Path) -> Self {
        Self {
            output_path,
            resources: Vec::new(),
        }
    }

    /// Adds a resource to the container.
    pub fn add_resource(&mut self, name: &str, resource: Resource, compress: bool) {
        self.resources.push((name.to_string(), resource, compress));
    }

    /// Processes all resources and writes them to the embed directory.
    pub fn embed_all(&self) -> io::Result<()> {
        let mut byte_arrays = Vec::new();

        for (name, resource, compress) in &self.resources {
            let bytes = resource.fetch(*compress)?;
            byte_arrays.push((name.as_str(), bytes));
        }

        // Use `embed-bytes` to write the byte arrays
        embed_bytes::write_byte_arrays(self.output_path, byte_arrays)
    }
}
