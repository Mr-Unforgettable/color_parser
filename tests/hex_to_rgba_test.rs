use color_parser::{ColorParserError, Rgba, parse_hex_to_rgba};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_hex_with_alpha() {
        let color = parse_hex_to_rgba("#FFAA33CC").unwrap();
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
        let color = parse_hex_to_rgba("#FFAA33").unwrap();
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
        let color = parse_hex_to_rgba("#FA3").unwrap();
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
            parse_hex_to_rgba("FFFFF"),
            Err(ColorParserError::InvalidLength)
        ));
    }

    #[test]
    fn test_invalid_characters() {
        assert!(matches!(
            parse_hex_to_rgba("#GGHHII"),
            Err(ColorParserError::InvalidCharacter)
        ));
    }
}
