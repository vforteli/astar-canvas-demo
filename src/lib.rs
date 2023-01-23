mod astar;
mod hybridheap;
mod utils;

use std::{convert::TryInto, vec};

use bmp::Image;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, astar-rust-wasm!");
}

#[wasm_bindgen]
pub fn click(x: u32, y: u32) {
    alert(format!("clicked {}:{}!", x, y).as_str());
}

struct Point {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    image: Image,
    frame_data: Vec<u8>,
    start_pixel: Option<Point>,
    cell_weights: Vec<u8>,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Self {
        let mut bytes: &[u8] = include_bytes!("../assets/castle.bmp");
        let image = bmp::from_reader(&mut bytes).unwrap();
        let height = image.get_height();
        let width = image.get_width();

        Board {
            width,
            height,
            image,
            frame_data: vec![0; (width * height * 4).try_into().unwrap()],
            start_pixel: None,
            cell_weights: vec![0; (width * height).try_into().unwrap()],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&mut self) -> *const u8 {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y * self.width + x) * 4;
                let pixel = self.image.get_pixel(x, y);

                self.frame_data[i as usize] = pixel.r;
                self.frame_data[(i + 1) as usize] = pixel.g;
                self.frame_data[(i + 2) as usize] = pixel.b;
                self.frame_data[(i + 3) as usize] = 255;
            }
        }

        if let Some(pixel) = &self.start_pixel {
            let pixel_index: usize = ((pixel.y * self.width * 4) + (pixel.x * 4))
                .try_into()
                .unwrap();
            self.frame_data[pixel_index] = 255;
            self.frame_data[pixel_index + 1] = 0;
            self.frame_data[pixel_index + 2] = 0;
        }

        self.frame_data.as_ptr()
    }

    pub fn frame_data(&self) -> *const u8 {
        self.frame_data.as_ptr()
    }

    pub fn click_cell(&mut self, x: u32, y: u32) {
        self.start_pixel = Some(Point { x, y });
    }
}
