use std::cmp::max;

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let foo = max(r, g);
    // max(, b)
}
