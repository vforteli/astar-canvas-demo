pub mod astar;
pub mod hybridheap;
pub mod utils;

use std::vec;

use astar::{astar::FindPath, point::Point};
use utils::image_to_vec;
use wasm_bindgen::prelude::*;

use crate::utils::{image_to_weight_map, set_panic_hook};

const TERRAIN_MIN_WEIGHT: f32 = 1.0;
const TERRAIN_MAX_WEIGHT: f32 = 10.0;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Board {
    frame_data: Vec<u8>,
    image_data: Vec<u8>,
    height: u32,
    width: u32,
    start_pixel: Option<Point>,
    cell_weights: Vec<f32>,
    path_finder: Option<FindPath>,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Self {
        set_panic_hook();
        let mut bytes: &[u8] = include_bytes!("../assets/castle.bmp");
        let image = bmp::from_reader(&mut bytes).unwrap();
        let image_data = image_to_vec(&image);
        let cell_weights = image_to_weight_map(&image, TERRAIN_MIN_WEIGHT, TERRAIN_MAX_WEIGHT);

        Board {
            frame_data: vec![0; (image.get_width() * image.get_height() * 4) as usize],
            image_data,
            height: image.get_height(),
            width: image.get_width(),
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

    pub fn render(&mut self) {
        let width = self.width;

        self.frame_data.clone_from(&self.image_data);

        if let Some(p) = &self.path_finder {
            for i in p.visited_points().iter().map(|v| v.0 * 4) {
                self.frame_data[i as usize] =
                    self.frame_data[i as usize].checked_sub(40).unwrap_or(0);
                self.frame_data[(i + 1) as usize] = self.frame_data[(i + 1) as usize]
                    .checked_sub(40)
                    .unwrap_or(0);
                self.frame_data[(i + 2) as usize] = self.frame_data[(i + 2) as usize]
                    .checked_sub(40)
                    .unwrap_or(0);
                self.frame_data[(i + 3) as usize] = 255;
            }

            if let Some(path_indexes) = &p.path_indexes {
                for i in path_indexes.iter().map(|v| v * 4) {
                    self.frame_data[i as usize] = 100;
                    self.frame_data[(i + 1) as usize] = 100;
                    self.frame_data[(i + 2) as usize] = 100;
                    self.frame_data[(i + 3) as usize] = 255;
                }
            }

            self.frame_data[(p.to_index * 4) as usize] = 255;
            self.frame_data[((p.to_index * 4) + 1) as usize] = 0;
            self.frame_data[((p.to_index * 4) + 2) as usize] = 0;

            self.frame_data[(p.from_index * 4) as usize] = 0;
            self.frame_data[((p.from_index * 4) + 1) as usize] = 255;
            self.frame_data[((p.from_index * 4) + 2) as usize] = 0;
        }

        if let Some(pixel) = &self.start_pixel {
            let pixel_index = (pixel.to_1d_index(width) * 4) as usize;
            self.frame_data[pixel_index] = 0;
            self.frame_data[pixel_index + 1] = 255;
            self.frame_data[pixel_index + 2] = 0;
        }
    }

    pub fn frame_data(&self) -> *const u8 {
        self.frame_data.as_ptr()
    }

    pub fn set_from(&mut self, x: u32, y: u32) {
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
        ));
    }

    pub fn tick(&mut self, ticks: u32) -> Option<f32> {
        match self.path_finder.as_mut() {
            Some(p) => p.tick(ticks, &self.cell_weights),
            None => Some(-1.0),
        }
    }
}
