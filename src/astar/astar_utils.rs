use std::{
    collections::{HashMap, HashSet},
    ops::Mul,
};

use super::{astar::VisitedPoint, point::Point};

/// Calculates the weight from one cell to a neighbour. The weight is from the middle of the first cell to the middle of the second cell
/// Moving diagonally increases weight.
#[inline(always)]
pub fn calculate_weight(from: &Point, to: &Point, weights: &Vec<f32>, width: u32) -> f32 {
    let mut to_weight = weights[to.to_1d_index(width) as usize];

    if to_weight < 0.0 {
        return to_weight;
    }

    let mut from_weight = weights[from.to_1d_index(width) as usize];

    if from.x != to.x && from.y != to.y {
        from_weight = from_weight.powi(2).mul(2.0).sqrt();
        to_weight = to_weight.powi(2).mul(2.0).sqrt();
    }

    to_weight / 2.0 + from_weight / 2.0
}

/// Heuristic function... since we have an euclidean space, this will just be the euclidean distance with the minimum terrain weight
/// For A* it is important to never overestimate the distance and therefore minimum weight is assumed
pub fn calculate_heuristical_distance(
    from: &Point,
    to: &Point,
    multiplier: u32,
    min_weight: f32,
) -> f32 {
    let x = (from.x as i32 - to.x as i32).pow(2) as f32 * min_weight;
    let y = (from.y as i32 - to.y as i32).pow(2) as f32 * min_weight;

    ((x + y) * multiplier as f32).sqrt()
}

/// Get the indexes of neighbouring cells, oob indexes are naturally not returned
pub fn get_neighbours(point: &Point, width: u32, height: u32) -> Vec<u32> {
    let index = point.to_1d_index(width);

    let mut neighbours: Vec<u32> = Vec::with_capacity(8);

    // top row
    if point.y > 0 {
        let top = index - width;

        if point.x > 0 {
            neighbours.push(top - 1);
        }

        neighbours.push(top);

        if point.x < width - 1 {
            neighbours.push(top + 1);
        }
    }

    // middle row
    if point.x > 0 {
        neighbours.push(index - 1);
    }
    if point.x < width - 1 {
        neighbours.push(index + 1);
    }

    // bottom row
    if point.y < height - 1 {
        let bottom = index + width;

        if point.x > 0 {
            neighbours.push(bottom - 1);
        }

        neighbours.push(bottom);

        if point.x < width - 1 {
            neighbours.push(bottom + 1);
        }
    }

    neighbours
}

pub fn reconstruct_path(
    visited: &HashMap<u32, VisitedPoint<f32, u32>>,
    to_key: u32,
) -> HashSet<u32> {
    let mut path = HashSet::new();
    let mut key = to_key;

    while let Some(index) = visited.get(&key) {
        if key == index.came_from_key {
            break;
        }

        path.insert(index.came_from_key);
        key = index.came_from_key;
    }

    path
}
