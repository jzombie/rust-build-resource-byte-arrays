use bytes::Bytes;
use flate2::write::GzEncoder;
use flate2::Compression;
use reqwest;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum Resource {
    File(String), // Path to a file
    Url(String),  // URL as a string
    Data(Bytes),  // Arbitrary in-memory data
}

impl Resource {
    /// Fetches and optionally compresses the resource into a `Bytes` object.
    pub fn fetch(&self, compress: bool) -> io::Result<Bytes> {
        match self {
            Resource::File(path) => {
                let mut file = File::open(path)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;

                if compress {
                    Ok(Self::compress_data(buffer)?)
                } else {
                    Ok(Bytes::from(buffer))
                }
            }
            Resource::Url(url) => {
                let response = reqwest::blocking::get(url)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
                    .bytes()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

                if compress {
                    Ok(Self::compress_data(response.to_vec())?)
                } else {
                    Ok(Bytes::from(response))
                }
            }
            Resource::Data(data) => {
                if compress {
                    Ok(Self::compress_data(data.to_vec())?)
                } else {
                    Ok(data.clone())
                }
            }
        }
    }

    /// Compresses data using GzEncoding.
    fn compress_data(data: Vec<u8>) -> io::Result<Bytes> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&data)?;
        let compressed_data = encoder.finish()?;
        Ok(Bytes::from(compressed_data))
    }
}
