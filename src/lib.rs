use bytes::Bytes;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// Writes byte arrays to a Rust file as `pub static` constants and saves the
/// byte contents to individual `.bin` files.
///
/// # Arguments
/// * `output_rust_path` - The path where the generated Rust file will be written.
/// * `output_bin_dir` - The directory where the `.bin` files will be saved.
/// * `byte_arrays` - A list of tuples (name, content) where:
///     - `name` is the name of the `pub static` variable.
///     - `content` is a `Bytes` object containing the byte data.
///
/// # Example
/// ```
/// use build_resource_byte_arrays::write_byte_arrays;
/// use bytes::Bytes;
///
/// write_byte_arrays(
///     "output.rs",
///     "bin_output_dir",
///     vec![("ARRAY1", Bytes::from(vec![1, 2, 3]))],
/// ).unwrap();
/// ```
pub fn write_byte_arrays<P: AsRef<Path>, Q: AsRef<Path>>(
    output_rust_path: P,
    output_bin_dir: Q,
    byte_arrays: Vec<(&str, Bytes)>,
    feature_flag: Option<String>,
) -> io::Result<()> {
    let expanded_feature_flag = match feature_flag {
        Some(flag) => flag,
        None => "build_resource_byte_arrays".to_string(),
    };

    let rust_file_path = output_rust_path.as_ref();
    let bin_dir_path = output_bin_dir.as_ref();

    // Ensure the output binary directory exists
    fs::create_dir_all(bin_dir_path)?;

    // Create or truncate the Rust file
    let mut rust_file = File::create(rust_file_path)?;

    // Write a header to the Rust file
    writeln!(
        rust_file,
        "// Automatically generated file. Do not edit.\n// Generated by build-resource-byte-arrays crate.\n"
    )?;

    for (name, content) in byte_arrays {
        // Write the `.bin` file
        let bin_file_path = bin_dir_path.join(format!("{name}.bin"));
        let mut bin_file = File::create(&bin_file_path)?;
        bin_file.write_all(&content)?;

        // Write logic for handling `build_mode` in the Rust file
        writeln!(rust_file, "#[cfg(feature = \"{}\")]", expanded_feature_flag)?;
        writeln!(
            rust_file,
            "pub static {name}: &[u8] = include_bytes!(\"{}\");",
            bin_file_path.display()
        )?;

        writeln!(rust_file, "")?;

        writeln!(
            rust_file,
            "#[cfg(not(feature = \"{}\"))]",
            expanded_feature_flag
        )?;
        writeln!(rust_file, "pub static {name}: &[u8] = &[];")?;

        writeln!(rust_file, "")?;

        writeln!(
            rust_file,
            "#[cfg(not(feature = \"{}\"))]",
            expanded_feature_flag
        )?;
        writeln!(rust_file, "#[ctor::ctor]",)?;
        writeln!(rust_file, "eprintln!(\"Warning: `ARRAY1` is empty because the `build_resource_byte_arrays` feature is not enabled.\");")?;
    }

    Ok(())
}
