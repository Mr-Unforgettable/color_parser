use color_parser::parse_hex_color;
use std::env;

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Ensure that the user passed exactly one argument (besides the program name)
    if args.len() != 2 {
        eprintln!("Usage: {} <hex-color>", args[0]); // Print usage error
        std::process::exit(1); // Exit with error code 1
    }

    // Parse the color using the parser function
    match parse_hex_color(&args[1]) {
        Ok(color) => {
            println!(
                "Parsed color: R: {}, G: {}, B: {}, A: {}",
                color.red, color.green, color.blue, color.alpha
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e); // Print the error if parsing failed
            std::process::exit(1); // Exit with error code 1
        }
    }
}
