mod utils;

use std::{convert::TryInto, vec};

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

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

struct Point {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    image_data: Vec<u8>,
    start_pixel: Option<Point>,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Self {
        let mut bytes: &[u8] = include_bytes!("../assets/castle.bmp");

        let image = bmp::from_reader(&mut bytes).unwrap();

        let height = image.get_height();
        let width = image.get_width();

        let cells = (0..width * height)
            .map(|i| if i % 7 == 0 { Cell::Alive } else { Cell::Dead })
            .collect();

        let mut image_data: Vec<u8> = vec![0; (width * height * 4).try_into().unwrap()];

        for y in 0..height {
            for x in 0..width {
                let i = (y * width + x) * 4;
                let pixel = image.get_pixel(x, y);

                image_data[i as usize] = pixel.r;
                image_data[(i + 1) as usize] = pixel.g;
                image_data[(i + 2) as usize] = pixel.b;
                image_data[(i + 3) as usize] = 255;
            }
        }

        Board {
            width,
            height,
            cells,
            image_data,
            start_pixel: None,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn image_data(&self) -> *const u8 {
        self.image_data.as_ptr()
    }

    pub fn click_cell(&mut self, x: u32, y: u32) {
        self.start_pixel = Some(Point { x, y });
        // // alert(format!("clicked {}:{}!", x, y).as_str());
        let pixel_index: usize = ((y * self.width * 4) + (x * 4)).try_into().unwrap();
        self.image_data[pixel_index] = 255;
        self.image_data[pixel_index + 1] = 0;
        self.image_data[pixel_index + 2] = 0;
    }
}
