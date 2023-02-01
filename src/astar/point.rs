use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn to_1d_index(&self, width: u32) -> u32 {
        self.y * width + self.x
    }

    #[inline(always)]
    pub fn from_1d_index(width: u32, index: u32) -> Self {
        Point {
            x: index % width,
            y: index / width,
        }
    }
}
