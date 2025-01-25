use build_resource_byte_arrays::clear_byte_arrays;
use std::env;
use std::process;

fn main() {
    // Collect the CLI arguments
    let args: Vec<String> = env::args().collect();

    // Check if the `--clear` argument and a file path are provided
    if args.len() != 3 || args[1] != "--clear" {
        eprintln!("Usage: {} --clear <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[2];

    // Attempt to clear the byte arrays in the specified file
    if let Err(err) = clear_byte_arrays(file_path) {
        eprintln!("Failed to clear byte arrays in '{}': {}", file_path, err);
        process::exit(1);
    }

    println!("Successfully cleared byte arrays in '{}'", file_path);
}
