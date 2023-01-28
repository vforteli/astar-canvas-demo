use std::{
    collections::{HashMap, HashSet},
    ops::Mul,
};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::hybridheap::HybridHeap;

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
}

pub struct PathResult {
    pub from_index: u32,
    pub to_index: u32,
    pub total_distance: f32,
    pub path_indexes: HashSet<u32>, // hohum.. maybe return coordinates instead, since that would better reflect the "public api"
    pub visited_indexes: HashMap<u32, VisitedPoint<f32, u32>>, // should be generic..
}

#[derive(Clone, Copy)]
pub struct VisitedPoint<S, K> {
    pub score: S,
    pub came_from_key: K,
}

pub fn coordinates_to_index(width: u32, x: u32, y: u32) -> u32 {
    y * width + x
}

pub fn index_to_coordinates(width: u32, index: u32) -> Point {
    Point {
        x: index % width,
        y: index / width,
    }
}

/// Calculates the weight from one cell to a neighbour. The weight is from the middle of the first cell to the middle of the second cell
/// Moving diagonally increases weight.
pub fn calculate_weight(from: &Point, to: &Point, weights: &Vec<f32>, width: u32) -> f32 {
    let mut to_weight = weights[coordinates_to_index(width, to.x, to.y) as usize];

    if to_weight < 0.0 {
        return to_weight;
    }

    let mut from_weight = weights[coordinates_to_index(width, from.x, from.y) as usize];

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
    let index = coordinates_to_index(width, point.x, point.y);

    let mut neighbours: Vec<u32> = Vec::with_capacity(8);

    // top row
    if point.y > 0 {
        let row_left = (point.y - 1) * width;
        let row_right = row_left + width - 1;
        let top = index - width;

        if top > row_left {
            neighbours.push(top - 1);
        }

        neighbours.push(top);

        if top < row_right {
            neighbours.push(top + 1);
        }
    }

    // middle row
    {
        let row_left = (point.y) * width;
        let row_right = row_left + width - 1;

        if index > row_left {
            neighbours.push(index - 1);
        }
        if index < row_right {
            neighbours.push(index + 1);
        }
    }

    // bottom row
    if point.y < (height - 1) {
        let row_left = (point.y + 1) * width;
        let row_right = row_left + width - 1;
        let bottom = index + width;

        if bottom > row_left {
            neighbours.push(bottom - 1);
        }

        neighbours.push(bottom);

        if bottom < row_right {
            neighbours.push(bottom + 1);
        }
    }

    neighbours
}

/// Find path \o/
pub fn find_path(
    from: Point,
    to: Point,
    width: u32,
    height: u32,
    multiplier: u32,
    min_weight: f32,
    weights: &Vec<f32>,
) -> Option<PathResult> {
    // openset contains seen nodes which havent yet been visited
    let mut openset: HybridHeap<u32, f32> = HybridHeap::new();

    // g scores contains the currently best scores for visited nodes and from where we ended up here
    let mut g_score: HashMap<u32, VisitedPoint<f32, u32>> = HashMap::new();

    let from_index = coordinates_to_index(width, from.x, from.y);
    let to_index = coordinates_to_index(width, to.x, to.y);

    g_score.insert(
        from_index,
        VisitedPoint {
            score: 0.0,
            came_from_key: from_index,
        },
    );
    openset.push(
        from_index,
        calculate_heuristical_distance(&from, &to, multiplier, min_weight),
    );

    while let Some(current_index) = openset.pop() {
        if current_index == to_index {
            return Some(PathResult {
                from_index,
                to_index,
                total_distance: g_score.get(&to_index).unwrap().score,
                path_indexes: reconstruct_path(&g_score, to_index),
                visited_indexes: g_score,
            });
        }

        let current_score = g_score[&current_index];
        let current_point = index_to_coordinates(width, current_index);

        for neighbour_index in get_neighbours(&current_point, width, height) {
            let neighbour_point = &index_to_coordinates(width, neighbour_index);
            let weight = calculate_weight(&current_point, &neighbour_point, &weights, width);

            // wall...
            if weight < 1.0 {
                continue;
            }

            let tentative_g_score = current_score.score + weight;

            // If this neighbour is already processed and the gscore through the current node is not lower, we can skip to the next
            // otherwise upsert the new score
            match g_score.get(&neighbour_index) {
                Some(p) if (*p).score <= tentative_g_score => continue,
                _ => g_score.insert(
                    neighbour_index,
                    VisitedPoint {
                        score: tentative_g_score,
                        came_from_key: current_index,
                    },
                ),
            };

            let tentative_f_score = tentative_g_score
                + calculate_heuristical_distance(&neighbour_point, &to, multiplier, min_weight);

            // If the neighbour node is seen for the first time, ie not open and not closed, put it in the openset
            // We can safely try to decrease the key, if the value is higher or doesnt exist, nothing will happen
            match openset.get_value(neighbour_index) {
                Some(v) if v > tentative_f_score => {
                    openset.change_value(neighbour_index, tentative_f_score)
                }
                _ => openset.push(neighbour_index, tentative_f_score),
            };
        }
    }
    None
}

fn reconstruct_path(visited: &HashMap<u32, VisitedPoint<f32, u32>>, to_key: u32) -> HashSet<u32> {
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

#[cfg(test)]
mod tests {

    use super::*;

    /*
    0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    ...
    80 81 82 83 84 85 86 87 88 89
    90 91 92 93 94 95 96 97 98 99
    */

    #[test]
    fn test_get_neighbours_top_left() {
        assert_eq!(
            get_neighbours(&Point { x: 0, y: 0 }, 10, 10),
            vec![1, 10, 11]
        );
    }

    #[test]
    fn test_get_neighbours_top_right() {
        assert_eq!(
            get_neighbours(&Point { x: 9, y: 0 }, 10, 10),
            vec![8, 18, 19]
        );
    }

    #[test]
    fn test_get_neighbours_bottom_left() {
        assert_eq!(
            get_neighbours(&Point { x: 0, y: 9 }, 10, 10),
            vec![80, 81, 91]
        );
    }

    #[test]
    fn test_get_neighbours_bottom_right() {
        assert_eq!(
            get_neighbours(&Point { x: 9, y: 9 }, 10, 10),
            vec![88, 89, 98]
        );
    }

    #[test]
    fn test_find_path_straight() {
        let weights: Vec<f32> = vec![1.0; 100];
        let height = 10;
        let width = 10;
        let multiplier = 1;
        let min_weight = 1.0;

        {
            let result = find_path(
                Point { x: 0, y: 0 },
                Point { x: 9, y: 0 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(9.0, result.total_distance);
            assert_eq!(
                HashSet::from([7, 4, 2, 0, 5, 8, 6, 3, 1]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 0, y: 0 },
                Point { x: 0, y: 9 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(9.0, result.total_distance);
            assert_eq!(
                HashSet::from([30, 10, 80, 60, 0, 70, 50, 20, 40]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 9, y: 0 },
                Point { x: 0, y: 0 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(9.0, result.total_distance);
            assert_eq!(
                HashSet::from([2, 5, 7, 4, 8, 1, 3, 6, 9]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 0, y: 9 },
                Point { x: 0, y: 0 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(9.0, result.total_distance);
            assert_eq!(
                HashSet::from([60, 70, 10, 40, 90, 80, 20, 30, 50]),
                result.path_indexes
            );
        }
    }

    #[test]
    fn test_find_path_diagonal() {
        let weights: Vec<f32> = vec![1.0; 100];
        let height = 10;
        let width = 10;
        let multiplier = 1;
        let min_weight = 1.0;

        {
            let result = find_path(
                Point { x: 0, y: 0 },
                Point { x: 9, y: 9 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(12.727921, result.total_distance);
            assert_eq!(
                HashSet::from([55, 11, 44, 0, 77, 66, 88, 33, 22]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 9, y: 0 },
                Point { x: 0, y: 9 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(12.727921, result.total_distance);
            assert_eq!(
                HashSet::from([45, 72, 63, 27, 36, 54, 18, 9, 81]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 9, y: 9 },
                Point { x: 0, y: 0 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(12.727921, result.total_distance);
            assert_eq!(
                HashSet::from([22, 55, 99, 66, 11, 44, 88, 33, 77]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 0, y: 9 },
                Point { x: 9, y: 0 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(12.727921, result.total_distance);
            assert_eq!(
                HashSet::from([36, 81, 18, 63, 90, 27, 45, 54, 72]),
                result.path_indexes
            );
        }
    }

    #[test]
    fn test_find_path_weights() {
        let mut weights: Vec<f32> = vec![1.0; 10];
        weights.extend(vec![2.0; 10]);
        weights.extend(vec![1.0; 10]);
        weights.extend(vec![2.0; 10]);
        weights.extend(vec![1.0; 10]);
        weights.extend(vec![2.0; 10]);
        weights.extend(vec![1.0; 10]);
        weights.extend(vec![2.0; 10]);
        weights.extend(vec![1.0; 10]);
        weights.extend(vec![2.0; 10]);

        let height = 10;
        let width = 10;
        let multiplier = 1;
        let min_weight = 1.0;

        {
            let result = find_path(
                Point { x: 0, y: 0 },
                Point { x: 9, y: 0 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(9.0, result.total_distance);
            assert_eq!(
                HashSet::from([2, 0, 6, 1, 8, 7, 5, 4, 3]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 0, y: 0 },
                Point { x: 0, y: 9 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(13.5, result.total_distance);
            assert_eq!(
                HashSet::from([60, 20, 70, 80, 10, 0, 40, 50, 30]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 9, y: 0 },
                Point { x: 9, y: 9 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(13.5, result.total_distance);
            assert_eq!(
                HashSet::from([79, 59, 89, 19, 39, 69, 9, 49, 29]),
                result.path_indexes
            );
        }

        {
            let result = find_path(
                Point { x: 9, y: 9 },
                Point { x: 0, y: 9 },
                width,
                height,
                multiplier,
                min_weight,
                &weights,
            )
            .unwrap();

            assert_eq!(11.242641, result.total_distance);
            assert_eq!(
                HashSet::from([84, 82, 81, 83, 86, 85, 99, 88, 87]),
                result.path_indexes
            );
        }
    }
}
