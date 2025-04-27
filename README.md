# Color Parser

A simple and lightweight Rust library to parse hex color codes.

Supports:
- 3-digit shorthand (`#FA3`)
- 4-digit shorthand with alpha (`#FA3C`)
- Full hex (`#FFAABB`)
- Full hex with alpha (`#FFAABBCC`)

## Usage

```rust
use color_parser::parse_hex_color;

let color = parse_hex_color("#FFAABB").unwrap();
println!("{:?}", color); // Rgba { red: 255, green: 170, blue: 187, alpha: 255 }
```

## Features
- [x] Support for RGBA values
- [x] Support for shorthand syntax
- [x] CLI support
- [x] Support for HSV, HSL, CMYL parsing
