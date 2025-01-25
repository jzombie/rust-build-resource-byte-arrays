use build_resource_byte_arrays::write_byte_arrays;

fn main() {
    // Create a byte array
    let byte_array = vec![1, 2, 3];

    // Write the byte array to a Rust file
    write_byte_arrays("output.rs", vec![("ARRAY_NAME", byte_array.into())]).unwrap();
}
