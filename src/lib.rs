#[derive(Debug, PartialEq, Eq)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

// Errors that can occur during hex color parsing
#[derive(Debug)]
pub enum ColorParserError {
    InvalidLength,
    InvalidCharacter,
}

impl std::fmt::Display for ColorParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColorParserError::InvalidLength => write!(f, "Hex color must be 6 character long"),
            ColorParserError::InvalidCharacter => write!(f, "Invalid character in hex color"),
        }
    }
}

impl std::error::Error for ColorParserError {}

pub fn parse_hex_color(hex: &str) -> Result<Rgba, ColorParserError> {
    let hex = hex.trim_start_matches('#');

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

    Ok(Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_hex_with_alpha() {
        let color = parse_hex_color("#FFAA33CC").unwrap();
        assert_eq!(
            color,
            Rgba {
                red: 255,
                green: 170,
                blue: 51,
                alpha: 204
            }
        );
    }

    #[test]
    fn test_full_hex_without_alpha() {
        let color = parse_hex_color("#FFAA33").unwrap();
        assert_eq!(
            color,
            Rgba {
                red: 255,
                green: 170,
                blue: 51,
                alpha: 255
            }
        );
    }

    #[test]
    fn test_shorthand_rgb() {
        let color = parse_hex_color("#FA3").unwrap();
        assert_eq!(
            color,
            Rgba {
                red: 255,
                green: 170,
                blue: 51,
                alpha: 255
            }
        );
    }

    #[test]
    fn test_invalid_length() {
        assert!(matches!(
            parse_hex_color("FFFFF"),
            Err(ColorParserError::InvalidLength)
        ));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(matches!(
            parse_hex_color("#GGHHII"),
            Err(ColorParserError::InvalidCharacter)
        ));
    }
}
