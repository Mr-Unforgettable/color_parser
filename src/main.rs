//! A simple CLI application for parsing and displaying color values in various formats.
//!
//! This application accepts a hex color code from the command line (e.g., `#FFAA00` or `ffaa00`),
//! converts it to RGBA, and then prints its equivalent in HSL, HSV, and CMYK formats.
//!
//! # Usage
//! ```bash
//! cargo run -- #ffaa00
//! cargo run -- ff8800
//! ```
//!
//! # Dependencies
//! - `color_parser` — your local crate/module for color conversions
//! - `owo-colors` — for terminal color preview output

use color_parser::{
    ColorParserError, parse_hex_to_rgba, parse_rgb_to_cmyk, parse_rgb_to_hsl, parse_rgb_to_hsv,
};
use owo_colors::OwoColorize;
use std::{env, process};

/// Entry point for the application.
///
/// If an error occurs during execution, the app prints an error message
/// and exits with code 1.
fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "❌ Error:".red(), e.to_string().red()); // Print error message
        process::exit(1); // Exit with error code
    }
}

/// Orchestrates the CLI tool logic:
/// 1. Parses the color argument.
/// 2. Converts it to RGBA.
/// 3. Converts and prints HSL, HSV, and CMYK representations.
/// 4. Displays a color swatch preview in the terminal.
fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let color_hex = get_color_argument()?;

    // Parse the hex color to RGBA
    let rgba_color = parse_hex_to_rgba(&color_hex)?;

    // Parse RGB to HSL
    let hsl_color = parse_rgb_to_hsl(&rgba_color)?;

    // Parse RGB to HSV
    let hsv_color = parse_rgb_to_hsv(&rgba_color)?;

    // Parse RGB to CMYK
    let cmyk_color = parse_rgb_to_cmyk(&rgba_color)?;

    // Create swatch using the actual RGB color
    let color_preview = "      ".on_truecolor(rgba_color.red, rgba_color.green, rgba_color.blue);

    println!("\n Hex Input: #{color}\n", color = color_hex.to_uppercase());
    println!("🎨  Color: {}", color_preview);
    println!(
        "\n🌈  RGBA: rgba({}, {}, {})",
        rgba_color.red, rgba_color.green, rgba_color.blue
    );
    println!("    → Red:   {}", rgba_color.red);
    println!("    → Green: {}", rgba_color.green);
    println!("    → Blue:  {}", rgba_color.blue);
    println!("    → Alpha: {}", rgba_color.alpha);

    println!(
        "\n🌈  HSL: hsl({}°, {}%, {}%)",
        hsl_color.hue.round(),
        hsl_color.saturation.round(),
        hsl_color.lightness.round()
    );
    println!("    → Hue:        {}°", hsl_color.hue.round());
    println!("    → Saturation: {}%", hsl_color.saturation.round());
    println!("    → Lightness:  {}%", hsl_color.lightness.round());

    println!(
        "\n🌈  HSV: hsv({}°, {}%, {}%)",
        hsv_color.hue.round(),
        hsv_color.saturation.round(),
        hsv_color.value.round()
    );
    println!("    → Hue:        {}°", hsv_color.hue.round());
    println!("    → Saturation: {}%", hsv_color.saturation.round());
    println!("    → value:      {}%", hsv_color.value.round());

    println!(
        "\n🌈  CMYK: CMYK({}%, {}%, {}%, {}%)",
        cmyk_color.cyan.round(),
        cmyk_color.magenta.round(),
        cmyk_color.yellow.round(),
        cmyk_color.black.round(),
    );
    println!("    → Cyan:    {}%", cmyk_color.cyan.round());
    println!("    → Magenta: {}%", cmyk_color.magenta.round());
    println!("    → Yellow:  {}%", cmyk_color.yellow.round());
    println!("    → Black:   {}%", cmyk_color.black.round());
    println!();
    Ok(())
}

/// Retrieves and validates the hex color argument from the command line.
///
/// # Returns
/// A `Result` containing the hex string if valid, or an error if missing.
///
/// # Errors
/// - Exits the program if no argument is passed.
/// - Returns `InvalidLength` if the argument is not a valid hex code length.
///
/// # Examples
/// ```bash
/// cargo run -- #ffaa00
/// cargo run -- ffcc00
/// ```
fn get_color_argument() -> Result<String, ColorParserError> {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Ensure that the user passed exactly one argument (besides the program name)
    if args.len() != 2 {
        eprintln!("Usage: {} <hex-color>", args[0]); // Print usage error
        eprintln!("Example: {} fff or {} #ffcc00", args[0], args[0]);
        std::process::exit(1); // Exit with error code 1
    }

    Ok(args[1].clone())
}
