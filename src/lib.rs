pub mod astar;
pub mod hybridheap;
pub mod utils;

use std::{convert::TryInto, vec};

use astar::{astar::FindPath, point::Point};
use bmp::Image;
use wasm_bindgen::prelude::*;

use crate::utils::{normalize, rgb_to_hsv};

const TERRAIN_MIN_WEIGHT: f32 = 1.0;
const TERRAIN_MAX_WEIGHT: f32 = 10.0;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    image: Image,
    frame_data: Vec<u8>,
    start_pixel: Option<Point>,
    cell_weights: Vec<f32>,
    path_finder: Option<FindPath>,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let mut bytes: &[u8] = include_bytes!("../assets/castle.bmp");
        let image = bmp::from_reader(&mut bytes).unwrap();
        let height = image.get_height();
        let width = image.get_width();

        let mut cell_weights = vec![0.0; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);

                let hsv = rgb_to_hsv(pixel.r, pixel.g, pixel.b);
                let inverted_brighntess = (hsv.brightness - 1.0).abs();

                let normalized_brighntess = if hsv.brightness < 0.05 {
                    -1.0
                } else {
                    normalize(
                        0.0,
                        1.0,
                        TERRAIN_MIN_WEIGHT,
                        TERRAIN_MAX_WEIGHT,
                        inverted_brighntess,
                    )
                };

                cell_weights[Point::new(x, y).to_1d_index(width) as usize] = normalized_brighntess;
            }
        }

        Board {
            width,
            height,
            image,
            frame_data: vec![0; (width * height * 4).try_into().unwrap()],
            start_pixel: None,
            cell_weights,
            path_finder: None,
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

                if self.path_finder.is_some()
                    && self.path_finder.as_ref().unwrap().path_indexes.is_some()
                    && self
                        .path_finder
                        .as_ref()
                        .unwrap()
                        .path_indexes
                        .as_ref()
                        .unwrap()
                        .contains(&Point::new(x, y).to_1d_index(self.width))
                {
                    self.frame_data[i as usize] = 100;
                    self.frame_data[(i + 1) as usize] = 100;
                    self.frame_data[(i + 2) as usize] = 100;
                    self.frame_data[(i + 3) as usize] = 255;
                } else if self.path_finder.is_some()
                    && self
                        .path_finder
                        .as_ref()
                        .unwrap()
                        .visited_points()
                        .contains_key(&Point::new(x, y).to_1d_index(self.width))
                {
                    self.frame_data[i as usize] = pixel.r.checked_sub(40).unwrap_or(pixel.r);
                    self.frame_data[(i + 1) as usize] = pixel.g.checked_sub(40).unwrap_or(pixel.g);
                    self.frame_data[(i + 2) as usize] = pixel.b.checked_sub(40).unwrap_or(pixel.b);
                    self.frame_data[(i + 3) as usize] = 255;
                } else {
                    self.frame_data[i as usize] = pixel.r;
                    self.frame_data[(i + 1) as usize] = pixel.g;
                    self.frame_data[(i + 2) as usize] = pixel.b;
                    self.frame_data[(i + 3) as usize] = 255;
                }
            }
        }

        if let Some(pixel) = &self.start_pixel {
            let pixel_index: usize = ((pixel.y * self.width * 4) + (pixel.x * 4))
                .try_into()
                .unwrap();
            self.frame_data[pixel_index] = 0;
            self.frame_data[pixel_index + 1] = 255;
            self.frame_data[pixel_index + 2] = 0;
        }

        if let Some(p) = &self.path_finder {
            self.frame_data[(p.to_index * 4) as usize] = 255;
            self.frame_data[((p.to_index * 4) + 1) as usize] = 0;
            self.frame_data[((p.to_index * 4) + 2) as usize] = 0;

            self.frame_data[(p.from_index * 4) as usize] = 0;
            self.frame_data[((p.from_index * 4) + 1) as usize] = 255;
            self.frame_data[((p.from_index * 4) + 2) as usize] = 0;
        }

        self.frame_data.as_ptr()
    }

    pub fn frame_data(&self) -> *const u8 {
        self.frame_data.as_ptr()
    }

    pub fn click_cell(&mut self, x: u32, y: u32) {
        self.path_finder = None;
        self.start_pixel = Some(Point { x, y });
    }

    /// Get cell info... currently just the weight
    pub fn get_cell_info(&mut self, x: u32, y: u32) -> Option<f32> {
        let index = Point::new(x, y).to_1d_index(self.width);
        self.cell_weights.get(index as usize).copied()
    }

    pub fn start_path_find(&mut self, from: Point, to: Point, multiplier: u32) {
        self.path_finder = Some(FindPath::new(
            from,
            to,
            self.width,
            self.height,
            multiplier,
            TERRAIN_MIN_WEIGHT,
        ))
    }

    pub fn tick(&mut self, ticks: u32) -> Option<f32> {
        match self.path_finder.as_mut() {
            Some(p) => p.tick(ticks, &self.cell_weights),
            None => Some(-1.0),
        }
    }
}
