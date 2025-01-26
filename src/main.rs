use build_resource_byte_arrays::write_byte_arrays;
include!("../output.rs");

fn main() {
    let byte_arrays = vec![("ARRAY1", vec![1, 2, 3].into())];

    write_byte_arrays("output.rs", "/tmp/123", byte_arrays, None);

    println!("{:?}", ARRAY1);
}
