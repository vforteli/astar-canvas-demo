pub mod astar;
pub mod hybridheap;
pub mod utils;

use std::{convert::TryInto, vec};

use astar::{PathResult, Point};
use bmp::Image;
use wasm_bindgen::prelude::*;

use crate::{
    astar::find_path,
    utils::{normalize, rgb_to_hsv},
};

const TERRAIN_MIN_WEIGHT: u32 = 1;
const TERRAIN_MAX_WEIGHT: u32 = 10;
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
    path_info: Option<PathResult>,
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
            cell_weights: vec![0.0; (width * height).try_into().unwrap()],
            path_info: None,
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

                if self.path_info.is_some()
                    && self
                        .path_info
                        .as_ref()
                        .unwrap()
                        .path_indexes
                        .contains(&Point::new(x, y).to_1d_index(self.width))
                {
                    self.frame_data[i as usize] = 100;
                    self.frame_data[(i + 1) as usize] = 100;
                    self.frame_data[(i + 2) as usize] = 100;
                    self.frame_data[(i + 3) as usize] = 255;
                } else if self.path_info.is_some()
                    && self
                        .path_info
                        .as_ref()
                        .unwrap()
                        .visited_indexes
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

                let hsv = rgb_to_hsv(pixel.r, pixel.g, pixel.b);
                let inverted_brighntess = (hsv.brightness - 1.0).abs();
                let normalized_brighntess = if hsv.brightness < 0.05 {
                    -1.0
                } else {
                    normalize(
                        0.0,
                        1.0,
                        TERRAIN_MIN_WEIGHT as f32,
                        TERRAIN_MAX_WEIGHT as f32,
                        inverted_brighntess,
                    )
                };

                self.cell_weights[Point::new(x, y).to_1d_index(self.width) as usize] =
                    normalized_brighntess;
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

        if let Some(p) = &self.path_info {
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
        self.path_info = None;
        self.start_pixel = Some(Point { x, y });
    }

    /// Get cell info... currently just the weight
    pub fn get_cell_info(&mut self, x: u32, y: u32) -> Option<f32> {
        let index = Point::new(x, y).to_1d_index(self.width);
        self.cell_weights.get(index as usize).copied()
    }

    /// Calculates path. Call render again after this to get the output...
    /// Returns total weight of path
    pub fn calculate_path(&mut self, from: Point, to: Point, multiplier: u32) -> Option<f32> {
        self.path_info = find_path(
            from,
            to,
            self.width,
            self.height,
            multiplier,
            TERRAIN_MIN_WEIGHT as f32,
            &self.cell_weights,
        );

        self.path_info.as_ref().and_then(|v| Some(v.total_distance))
    }
}
