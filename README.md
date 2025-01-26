# embed-bytes (Work in Progress)

`embed-bytes` is a Rust crate that simplifies embedding low-level binary arrays and assets in other Rust programs. It relies only on the bytes crate for efficient handling of these assets.

When used in a `build.rs` script, `embed-bytes` operates as a zero-cost abstraction, as all processing occurs at compile-time. The embedding directory is automatically created during the build process, where asset names are converted into `.bin` files, and a Rust source file is generated to include these assets using include_bytes!.

-- WORK IN PROGRESS --

## Related

- https://stackoverflow.com/questions/75676449/how-to-build-and-publish-a-crate-containing-generated-code
- https://github.com/rust-lang/cargo/issues/12456
- https://github.com/rust-lang/cargo/issues/9398
- https://github.com/pyrossh/rust-embed
