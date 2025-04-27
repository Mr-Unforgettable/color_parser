use color_parser::{Rgba, parse_rgb_to_cmyk};

#[cfg(test)]
mod test {
    use super::*;

    fn rgba(r: u8, g: u8, b: u8) -> Rgba {
        Rgba {
            red: r,
            green: g,
            blue: b,
            alpha: 255,
        }
    }

    #[test]
    fn test_cmyk_black() {
        let color = rgba(0, 0, 0);
        let cmyk = parse_rgb_to_cmyk(&color).unwrap();
        assert_eq!(cmyk.cyan, 0.0);
        assert_eq!(cmyk.magenta, 0.0);
        assert_eq!(cmyk.yellow, 0.0);
        assert_eq!(cmyk.black, 100.0);
    }

    #[test]
    fn test_cmyk_white() {
        let color = rgba(255, 255, 255);
        let cmyk = parse_rgb_to_cmyk(&color).unwrap();
        assert_eq!(cmyk.cyan, 0.0);
        assert_eq!(cmyk.magenta, 0.0);
        assert_eq!(cmyk.yellow, 0.0);
        assert_eq!(cmyk.black, 0.0);
    }

    #[test]
    fn test_cmyk_red() {
        let color = rgba(255, 0, 0);
        let cmyk = parse_rgb_to_cmyk(&color).unwrap();
        assert_eq!(cmyk.cyan.round(), 0.0);
        assert_eq!(cmyk.magenta.round(), 100.0);
        assert_eq!(cmyk.yellow.round(), 100.0);
        assert_eq!(cmyk.black.round(), 0.0);
    }

    #[test]
    fn test_cmyk_green() {
        let color = rgba(0, 255, 0);
        let cmyk = parse_rgb_to_cmyk(&color).unwrap();
        assert_eq!(cmyk.cyan.round(), 100.0);
        assert_eq!(cmyk.magenta.round(), 0.0);
        assert_eq!(cmyk.yellow.round(), 100.0);
        assert_eq!(cmyk.black.round(), 0.0);
    }

    #[test]
    fn test_cmyk_blue() {
        let color = rgba(0, 0, 255);
        let cmyk = parse_rgb_to_cmyk(&color).unwrap();
        assert_eq!(cmyk.cyan.round(), 100.0);
        assert_eq!(cmyk.magenta.round(), 100.0);
        assert_eq!(cmyk.yellow.round(), 0.0);
        assert_eq!(cmyk.black.round(), 0.0);
    }
}
