//! A utility library for parsing and converting colors between different formats.
//!
//! Supports conversion between:
//! - Hexadecimal (`#RRGGBB`, `#RRGGBBAA`, `#RGB`, `#RGBA`) and `Rgba`
//! - `Rgba` to `Hsl`, `Hsv`, and `Cmyk`
//!
//! # Example
//! ```rust
//! use color_parser::*;
//!
//! let rgba = parse_hex_to_rgba("#ff8800").unwrap();
//! let hsl = parse_rgb_to_hsl(&rgba).unwrap();
//! let hsv = parse_rgb_to_hsv(&rgba).unwrap();
//! let cmyk = parse_rgb_to_cmyk(&rgba).unwrap();
//! ```

/// Represents a color in the RGBA color space.
#[derive(Debug, PartialEq, Eq)]
pub struct Rgba {
    /// Red channel (0–255)
    pub red: u8,
    /// Green channel (0–255)
    pub green: u8,
    /// Blue channel (0–255)
    pub blue: u8,
    /// Alpha channel (0–255), where 255 is fully opaque
    pub alpha: u8,
}

/// Represents a color in the HSL color space.
pub struct Hsl {
    /// Hue in degrees [0–360)
    pub hue: f64,
    /// Saturation as percentage [0–100]
    pub saturation: f64,
    /// Lightness as percentage [0–100]
    pub lightness: f64,
}

/// Represents a color in the HSV color space.
pub struct Hsv {
    /// Hue in degrees [0–360)
    pub hue: f64,
    /// Saturation as percentage [0–100]
    pub saturation: f64,
    /// Value (brightness) as percentage [0–100]
    pub value: f64,
}

/// Represents a color in the CMYK color space.
pub struct Cmyk {
    /// Cyan channel as percentage [0–100]
    pub cyan: f64,
    /// Magenta channel as percentage [0–100]
    pub magenta: f64,
    /// Yellow channel as percentage [0–100]
    pub yellow: f64,
    /// Black (Key) channel as percentage [0–100]
    pub black: f64,
}

/// Errors that can occur during color parsing or conversion.
#[derive(Debug)]
pub enum ColorParserError {
    /// Invalid hex string length (must be 3, 4, 6, or 8 characters)
    InvalidLength,
    /// Invalid character in hex string
    InvalidCharacter,
    /// RGB values must be in the 0–255 range
    InvalidRgbValue,
}

impl std::fmt::Display for ColorParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColorParserError::InvalidLength => write!(f, "Hex color must be 6 character long"),
            ColorParserError::InvalidCharacter => write!(f, "Invalid character in hex color"),
            ColorParserError::InvalidRgbValue => write!(f, "RGB value must be between 0 and 255"),
        }
    }
}

impl std::error::Error for ColorParserError {}

/// Parses a hexadecimal color string into an `Rgba` struct.
///
/// Accepts the following formats:
/// - `#RRGGBB`
/// - `#RRGGBBAA`
/// - `#RGB`
/// - `#RGBA`
///
/// # Errors
/// Returns `ColorParserError` if the format or characters are invalid.
///
/// # Examples
/// ```rust
/// use color_parser::{parse_hex_to_rgba};
///
/// let color = parse_hex_to_rgba("#ff8800").unwrap();
/// assert_eq!(color.red, 255);
/// ```
pub fn parse_hex_to_rgba(hex: &str) -> Result<Rgba, ColorParserError> {
    let hex = hex.trim_start_matches('#');

    // Handle different hex color formats and ensure valid length
    let expanded = match hex.len() {
        8 => hex.to_string(),      // Full RGBA
        6 => format!("{}FF", hex), // default alpha = 255
        4 => {
            // Expands #RGBA => #RRGGBBAA
            let mut s = String::with_capacity(8);
            for ch in hex.chars() {
                s.push(ch);
                s.push(ch);
            }
            s
        }
        3 => {
            // Expands #RGB => #RRGGBB + FF
            let mut s = String::with_capacity(8);
            for ch in hex.chars() {
                s.push(ch);
                s.push(ch);
            }
            s.push_str("FF"); // Default alpha
            s
        }
        _ => return Err(ColorParserError::InvalidLength),
    };

    let red =
        u8::from_str_radix(&expanded[0..2], 16).map_err(|_| ColorParserError::InvalidCharacter)?;
    let green =
        u8::from_str_radix(&expanded[2..4], 16).map_err(|_| ColorParserError::InvalidCharacter)?;
    let blue =
        u8::from_str_radix(&expanded[4..6], 16).map_err(|_| ColorParserError::InvalidCharacter)?;
    let alpha =
        u8::from_str_radix(&expanded[6..8], 16).map_err(|_| ColorParserError::InvalidCharacter)?;

    // Check that RGBA values are within valid range
    if !(0..=255).contains(&red)
        || !(0..=255).contains(&green)
        || !(0..=255).contains(&blue)
        || !(0..=255).contains(&alpha)
    {
        return Err(ColorParserError::InvalidRgbValue);
    }

    Ok(Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

/// Converts an `Rgba` color to the HSL color space.
///
/// # Errors
/// Returns `InvalidRgbValue` if RGB values are outside the 0–255 range.
/// This should never occur with valid `u8` values.
///
/// # Examples
/// ```rust
/// use color_parser::{Rgba, parse_rgb_to_hsl};
///
/// let rgba = Rgba { red: 255, green: 0, blue: 0, alpha: 255 };
/// let hsl = parse_rgb_to_hsl(&rgba).unwrap();
/// assert_eq!(hsl.hue.round(), 0.0);
/// ```
pub fn parse_rgb_to_hsl(color: &Rgba) -> Result<Hsl, ColorParserError> {
    // Check that RGBA values are within valid range
    if !(0..=255).contains(&color.red)
        || !(0..=255).contains(&color.green)
        || !(0..=255).contains(&color.blue)
    {
        return Err(ColorParserError::InvalidRgbValue);
    }

    // Convert r, g, b [0, 255] range to [0, 1]
    let r = color.red as f64 / 255.0;
    let g = color.green as f64 / 255.0;
    let b = color.blue as f64 / 255.0;

    // Find min and max among the r, g, b values
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Calculate lightness
    let lightness = (max + min) / 2.0;

    // Calculate saturation
    let saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };

    // Calculate hue
    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        (g - b) / delta + (if g < b { 6.0 } else { 0.0 })
    } else if max == g {
        (b - r) / delta + 2.0 // 120° on the color wheel
    } else {
        (r - g) / delta + 4.0 // 240° on the color wheel
    };

    Ok(Hsl {
        hue: (hue * 60.0) % 360.0, // Normalize the hue to [0°, 350°]
        saturation: saturation * 100.0,
        lightness: lightness * 100.0,
    })
}

/// Converts an `Rgba` color to the HSV color space.
///
/// # Errors
/// Returns `InvalidRgbValue` if RGB values are outside the 0–255 range.
///
/// # Examples
/// ```rust
/// use color_parser::{Rgba, parse_rgb_to_hsv};
///
/// let rgba = Rgba { red: 0, green: 255, blue: 0, alpha: 255 };
/// let hsv = parse_rgb_to_hsv(&rgba).unwrap();
/// assert_eq!(hsv.hue.round(), 120.0);
/// ```
pub fn parse_rgb_to_hsv(color: &Rgba) -> Result<Hsv, ColorParserError> {
    // Check that RGBA values are within valid range
    if !(0..=255).contains(&color.red)
        || !(0..=255).contains(&color.green)
        || !(0..=255).contains(&color.blue)
    {
        return Err(ColorParserError::InvalidRgbValue);
    }

    // Convert r, g, b [0, 255] range to [0, 1]
    let r = color.red as f64 / 255.0;
    let g = color.green as f64 / 255.0;
    let b = color.blue as f64 / 255.0;

    // Find min and max among the r, g, b values
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Calculate the value
    let value = max;

    // Calculate the saturation
    let saturation = if delta == 0.0 { 0.0 } else { delta / value };

    // Calculate the hue
    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        (g - b) / delta + (if g < b { 6.0 } else { 0.0 })
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };

    Ok(Hsv {
        hue: (hue * 60.0) % 360.0,
        saturation: saturation * 100.0,
        value: value * 100.0,
    })
}

/// Converts an `Rgba` color to the CMYK color space.
///
/// # Errors
/// Returns `InvalidRgbValue` if RGB values are outside the 0–255 range.
///
/// # Examples
/// ```rust
/// use color_parser::{Rgba, parse_rgb_to_cmyk};
///
/// let rgba = Rgba { red: 0, green: 0, blue: 0, alpha: 255 };
/// let cmyk = parse_rgb_to_cmyk(&rgba).unwrap();
/// assert_eq!(cmyk.black, 100.0);
/// ```
pub fn parse_rgb_to_cmyk(color: &Rgba) -> Result<Cmyk, ColorParserError> {
    // Check that RGBA values are within valid range
    if !(0..=255).contains(&color.red)
        || !(0..=255).contains(&color.green)
        || !(0..=255).contains(&color.blue)
    {
        return Err(ColorParserError::InvalidRgbValue);
    }

    // Convert r, g, b [0, 255] range to [0, 1]
    let r = color.red as f64 / 255.0;
    let g = color.green as f64 / 255.0;
    let b = color.blue as f64 / 255.0;

    // Calculate the black key color
    let k = 1.0 - r.max(g).max(b);

    // If RGB is key (black), set CMY to 0
    if k == 1.0 {
        return Ok(Cmyk {
            cyan: 0.0 * 100.0,
            magenta: 0.0 * 100.0,
            yellow: 0.0 * 100.0,
            black: 100.0,
        });
    }
    // Calculate cyan, magenta and yellow color
    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);

    Ok(Cmyk {
        cyan: c * 100.0,
        magenta: m * 100.0,
        yellow: y * 100.0,
        black: k * 100.0,
    })
}
