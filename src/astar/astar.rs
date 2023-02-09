use std::collections::{HashMap, HashSet};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{astar::point::Point, hybridheap::HybridHeap};

use super::astar_utils::{
    calculate_heuristical_distance, calculate_weight, get_neighbours, reconstruct_path,
};

pub struct PathResult {
    pub from_index: u32,
    pub to_index: u32,
    pub total_distance: f32,
    pub path_indexes: HashSet<u32>, // hohum.. maybe return coordinates instead, since that would better reflect the "public api"
    pub visited_indexes: HashMap<u32, VisitedPoint<f32, u32>>, // should be generic..
}

#[wasm_bindgen]
pub struct PathStatistics {
    pub total_distance: f32,
    pub nodes_visited_count: u32,
    pub path_nodes_count: u32,
}

#[derive(Clone, Copy)]
pub struct VisitedPoint<S, K> {
    pub score: S,
    pub came_from_key: K,
}

pub struct FindPath {
    to: Point,
    pub to_index: u32,
    pub from_index: u32,
    width: u32,
    height: u32,
    multiplier: u32,
    min_weight: f32,
    openset: HybridHeap<u32, f32>, // openset contains seen nodes which havent yet been visited
    g_score: HashMap<u32, VisitedPoint<f32, u32>>, // g scores contains the currently best scores for visited nodes and from where we ended up here
    pub path_indexes: Option<HashSet<u32>>, // hohum.. maybe return coordinates instead, since that would better reflect the "public api"
}

impl FindPath {
    pub fn new(
        from: Point,
        to: Point,
        width: u32,
        height: u32,
        multiplier: u32,
        min_weight: f32,
    ) -> Self {
        let mut openset: HybridHeap<u32, f32> = HybridHeap::with_capacity(1000);
        let mut g_score: HashMap<u32, VisitedPoint<f32, u32>> = HashMap::with_capacity(1000);

        let from_index = from.to_1d_index(width);
        let to_index = to.to_1d_index(width);

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

        FindPath {
            to,
            to_index,
            from_index,
            width,
            height,
            multiplier,
            min_weight,
            openset,
            g_score,
            path_indexes: None,
        }
    }

    pub fn reset(&mut self) {
        self.g_score.clear();
        self.openset.clear();
        self.path_indexes = None;
    }

    pub fn visited_points(&self) -> &HashMap<u32, VisitedPoint<f32, u32>> {
        &self.g_score
    }

    pub fn openset_points(&self) -> &HybridHeap<u32, f32> {
        &self.openset
    }

    /// Tick ... specify number of max nodes to process
    /// Returns None if the path was not found with specified tick count
    pub fn tick(&mut self, ticks: u32, weights: &Vec<f32>) -> Option<f32> {
        let mut remaining_ticks = ticks; // todo wtf, no underflow panic but wrapping? so, apparently wasm is fine with js passing in 0 here and then decreasing it without panicking
        while let Some(current_index) = self.openset.pop() {
            if current_index == self.to_index {
                self.path_indexes = Some(reconstruct_path(&self.g_score, self.to_index));
                return Some(self.g_score.get(&self.to_index).unwrap().score);
            }

            tick(
                &mut self.g_score,
                &mut self.openset,
                &self.to,
                current_index,
                self.width,
                self.height,
                self.multiplier,
                self.min_weight,
                weights,
            );

            remaining_ticks = remaining_ticks - 1;

            if remaining_ticks <= 0 {
                return None;
            }
        }

        None
    }
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
    let mut openset: HybridHeap<u32, f32> = HybridHeap::with_capacity(1000);

    // g scores contains the currently best scores for visited nodes and from where we ended up here
    let mut g_score: HashMap<u32, VisitedPoint<f32, u32>> = HashMap::with_capacity(1000);

    let from_index = from.to_1d_index(width);
    let to_index = to.to_1d_index(width);

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

        tick(
            &mut g_score,
            &mut openset,
            &to,
            current_index,
            width,
            height,
            multiplier,
            min_weight,
            weights,
        )
    }
    None
}

fn tick(
    g_score: &mut HashMap<u32, VisitedPoint<f32, u32>>,
    openset: &mut HybridHeap<u32, f32>,
    to: &Point,
    current_index: u32,
    width: u32,
    height: u32,
    multiplier: u32,
    min_weight: f32,
    weights: &Vec<f32>,
) {
    let current_score = g_score[&current_index];
    let current_point = Point::from_1d_index(width, current_index);

    for neighbour_index in get_neighbours(&current_point, width, height) {
        let neighbour_point = Point::from_1d_index(width, neighbour_index);
        let weight = calculate_weight(&current_point, &neighbour_point, &weights, width);

        // wall...
        if weight <= 0.0 {
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
