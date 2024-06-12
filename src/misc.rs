use nannou::{
    color::{rgb, Rgb},
    geom::Rect,
    Draw,
};

use crate::lsystem::LsystemConfig;

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

pub fn debug_info(draw: &Draw, win: Rect, lsystem: &LsystemConfig) {
    let pad = 6.0;
    draw.text(&format!("Lsystem config\n\n{}", lsystem))
        .h(win.pad(pad).h())
        .w(win.pad(pad).w())
        .line_spacing(pad)
        .font_size(14)
        .align_text_top()
        .left_justify();
}
