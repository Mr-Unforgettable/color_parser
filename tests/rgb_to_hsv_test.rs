use color_parser::{Rgba, parse_rgb_to_hsv};

#[cfg(test)]
mod tests {
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
    fn test_hsv_black() {
        let color = rgba(0, 0, 0);
        let hsv = parse_rgb_to_hsv(&color).unwrap();

        assert_eq!(hsv.hue, 0.0);
        assert_eq!(hsv.saturation, 0.0);
        assert_eq!(hsv.value, 0.0);
    }

    #[test]
    fn test_hsv_white() {
        let color = rgba(255, 255, 255);
        let hsv = parse_rgb_to_hsv(&color).unwrap();

        assert_eq!(hsv.hue, 0.0);
        assert_eq!(hsv.saturation, 0.0);
        assert_eq!(hsv.value, 100.0);
    }

    #[test]
    fn test_hsv_red() {
        let color = rgba(255, 0, 0);
        let hsv = parse_rgb_to_hsv(&color).unwrap();

        assert!((hsv.hue - 0.0).abs() < 0.01);
        assert_eq!(hsv.saturation, 100.0);
        assert_eq!(hsv.value, 100.0);
    }

    #[test]
    fn test_hsv_green() {
        let color = rgba(0, 255, 0);
        let hsv = parse_rgb_to_hsv(&color).unwrap();

        assert!((hsv.hue - 120.0).abs() < 0.01);
        assert_eq!(hsv.saturation, 100.0);
        assert_eq!(hsv.value, 100.0);
    }

    #[test]
    fn test_hsv_blue() {
        let color = rgba(0, 0, 255);
        let hsv = parse_rgb_to_hsv(&color).unwrap();

        assert!((hsv.hue - 240.0).abs() < 0.01);
        assert_eq!(hsv.saturation, 100.0);
        assert_eq!(hsv.value, 100.0);
    }

    #[test]
    fn test_hsv_gray() {
        let color = rgba(128, 128, 128);
        let hsv = parse_rgb_to_hsv(&color).unwrap();

        assert_eq!(hsv.hue, 0.0);
        assert_eq!(hsv.saturation, 0.0);
        assert!((hsv.value - 50.2).abs() < 0.2); // 128/255 ~= 50.2%
    }
}
