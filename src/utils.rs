pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[derive(Clone, Copy, Debug)]
pub struct HSV {
    pub hue: f32,
    pub saturation: f32,
    pub brightness: f32,
}

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> HSV {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    HSV {
        hue: 0.0,
        saturation: 0.0,
        brightness: r.max(g).max(b),
    }
}

pub fn normalize(
    input_min: f32,
    input_max: f32,
    output_min: f32,
    output_max: f32,
    value: f32,
) -> f32 {
    output_min + (value - input_min) * (output_max - output_min) / (input_max - input_min)
}
