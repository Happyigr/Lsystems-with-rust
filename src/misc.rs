use nannou::color::{rgb, Rgb};

pub fn hex_to_rgb(hex: &str) -> Rgb {
    if hex.len() != 7 {
        panic!(
            "The hex value is not correct, use \"#RRGGBB\". Was used: {}",
            hex
        );
    }

    let r = u8::from_str_radix(&hex[1..3], 16).expect("Wrong hex value");
    let g = u8::from_str_radix(&hex[3..5], 16).expect("Wrong hex value");
    let b = u8::from_str_radix(&hex[5..], 16).expect("Wrong hex value");

    rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
