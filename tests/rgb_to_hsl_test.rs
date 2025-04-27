use color_parser::{Rgba, parse_rgb_to_hsl};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hsl_red() {
        let color = Rgba {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        }; // Pure red
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 0); // hue: 0°
        assert_eq!(hsl.saturation as u32, 100); // saturation: 100%
        assert_eq!(hsl.lightness as u32, 50); // lightness: 50%
    }

    #[test]
    fn test_hsl_green() {
        let color = Rgba {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 255,
        }; // Pure green
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 120); // hue: 120°
        assert_eq!(hsl.saturation as u32, 100); // saturation: 100%
        assert_eq!(hsl.lightness as u32, 50); // lightness: 50%
    }

    #[test]
    fn test_hsl_blue() {
        let color = Rgba {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 255,
        }; // Pure blue
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 240); // hue: 240°
        assert_eq!(hsl.saturation as u32, 100); // saturation: 100%
        assert_eq!(hsl.lightness as u32, 50); // lightness: 50%
    }

    #[test]
    fn test_hsl_yellow() {
        let color = Rgba {
            red: 255,
            green: 255,
            blue: 0,
            alpha: 255,
        }; // Yellow (red + green)
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 60); // hue: 60°
        assert_eq!(hsl.saturation as u32, 100); // saturation: 100%
        assert_eq!(hsl.lightness as u32, 50); // lightness: 50%
    }

    #[test]
    fn test_hsl_cyan() {
        let color = Rgba {
            red: 0,
            green: 255,
            blue: 255,
            alpha: 255,
        }; // Cyan (green + blue)
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 180); // hue: 180°
        assert_eq!(hsl.saturation as u32, 100); // saturation: 100%
        assert_eq!(hsl.lightness as u32, 50); // lightness: 50%
    }

    #[test]
    fn test_hsl_magenta() {
        let color = Rgba {
            red: 255,
            green: 0,
            blue: 255,
            alpha: 255,
        }; // Magenta (red + blue)
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 300); // hue: 300°
        assert_eq!(hsl.saturation as u32, 100); // saturation: 100%
        assert_eq!(hsl.lightness as u32, 50); // lightness: 50%
    }

    #[test]
    fn test_hsl_gray() {
        let color = Rgba {
            red: 128,
            green: 128,
            blue: 128,
            alpha: 255,
        }; // Gray (Equal RGB values)
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 0); // hue: 60°
        assert_eq!(hsl.saturation as u32, 0); // saturation: 0%
        assert_eq!(hsl.lightness as u32, 50); // lightness: 50%
    }

    #[test]
    fn test_hsl_dark_red() {
        let color = Rgba {
            red: 139,
            green: 0,
            blue: 0,
            alpha: 255,
        }; // Dark Red
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 0); // hue: 0°
        assert_eq!(hsl.saturation as u32, 100); // saturation: 100%
        assert_eq!(hsl.lightness as u32, 27); // lightness: ~ 27%
    }

    #[test]
    fn test_hsl_light_blue() {
        let color = Rgba {
            red: 173,
            green: 216,
            blue: 230,
            alpha: 255,
        }; // Light Blue
        let hsl = parse_rgb_to_hsl(&color).unwrap();

        assert_eq!(hsl.hue as u32, 194); // hue: ~ 195°
        assert_eq!(hsl.saturation as u32, 53); // saturation: 53%
        assert_eq!(hsl.lightness as u32, 79); // lightness: 79%
    }
}
