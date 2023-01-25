use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    ops::Mul,
};

use crate::hybridheap::HybridHeap;

#[derive(Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

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
pub fn calculate_weight(from: Point, to: Point, weights: &Vec<f32>, width: u32) -> f32 {
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
    let x = (from.x - to.x) as f32 * min_weight;
    let y = (from.y - to.y) as f32 * min_weight;

    ((x * x + y * y) * multiplier as f32).sqrt()
}

/// Get the indexes of neighbouring cells, oob indexes are naturally not returned
pub fn get_neighbours(point: Point, width: u32, height: u32) -> Vec<u32> {
    let index: i64 = coordinates_to_index(width, point.x, point.y).into();

    let mut neighbours: Vec<u32> = Vec::new();

    // top row
    if point.y > 0 {
        let row_left = (point.y - 1) * width;
        let row_right = row_left + width - 1;
        let top = index - width as i64;

        if top > row_left.into() {
            neighbours.push((top - 1).try_into().unwrap());
        }

        neighbours.push(top.try_into().unwrap());

        if top < row_right.into() {
            neighbours.push((top + 1).try_into().unwrap());
        }
    }

    // middle row
    {
        let row_left = (point.y) * width;
        let row_right = row_left + width - 1;

        if index > row_left.into() {
            neighbours.push((index - 1).try_into().unwrap());
        }
        if index < row_right.into() {
            neighbours.push((index + 1).try_into().unwrap());
        }
    }

    // bottom row
    if point.y < (height - 1) {
        let row_left = (point.y + 1) * width;
        let row_right = row_left + width - 1;
        let bottom = index + width as i64;

        if bottom > row_left.into() {
            neighbours.push((bottom - 1).try_into().unwrap());
        }

        neighbours.push(bottom.try_into().unwrap());

        if bottom < row_right.into() {
            neighbours.push((bottom + 1).try_into().unwrap());
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
    weights: Vec<f32>,
) {
    let mut openset: HybridHeap<u32, f32> = HybridHeap::new();
    let mut closedset: HashSet<u32> = HashSet::new();
    let mut g_score: HashMap<u32, f32> = HashMap::new();
    let mut came_from: HashMap<u32, u32> = HashMap::new();

    let from_index = coordinates_to_index(width, from.x, from.y);
    let to_index = coordinates_to_index(width, to.x, to.y);

    g_score.insert(from_index, 0.0);
    openset.push(
        &from_index,
        calculate_heuristical_distance(&from, &to, multiplier, min_weight),
    );

    while let Some(current) = openset.pop().cloned() {
        closedset.insert(current);

        if current == to_index {
            todo!("reconstruct path!")
        }

        let current_score = g_score.get(&current).unwrap().clone();

        for neighbour in get_neighbours(index_to_coordinates(width, current), width, height) {
            let weight = calculate_weight(
                index_to_coordinates(width, current),
                index_to_coordinates(width, neighbour),
                &weights,
                width,
            );

            if weight < 1.0 {
                continue;
            }

            let tentative_g_score = current_score + weight;
            let g_score_neighbour = g_score.get(&neighbour);

            // If this neighbour is already processed and the gscore through the current node is not lower, we can skip to the next
            //if (closedset.containsKey(neighbour) && g_scoreneighbour <= tentative_gscore)
            if let Some(score) = g_score_neighbour {
                // doing this on the same line as above is unstable :/
                if *score <= tentative_g_score {
                    continue;
                }
            }

            // If this is the first time at the neighbour, or the gscore through the current node is better, update stuff
            if g_score_neighbour.is_none() || tentative_g_score < *g_score_neighbour.unwrap() {
                g_score.insert(neighbour, tentative_g_score);
                came_from.insert(neighbour, current);
            }

            let tentative_f_score = tentative_g_score
                + calculate_heuristical_distance(
                    &index_to_coordinates(width, neighbour),
                    &to,
                    multiplier,
                    min_weight,
                );

            // If the neighbour node is seen for the first time, ie not open and not closed, put it in the openset
            if !openset.contains_key(&neighbour) && !closedset.contains(&neighbour) {
                openset.push(neighbour, tentative_f_score);
            } else {
                // We can safely try to decrease the key, if the value is higher or doesnt exist, nothing will happen
                openset.change_value(&neighbour, tentative_f_score);
            }
        }
    }
}

/*
/**
     * Find a path from start to end coordinates
     * @param start
     * @param end
     * @param heuristicMultiplier
     * @return PathInfo containing the path and path length if available, and the nodes visited
     */
    public PathInfo findPath(Coordinates start, Coordinates end, int heuristicMultiplier)
    {
        int initialsize = 1000;
        HybridHeap<Float, Coordinates> openset = new HybridHeap<>();
        AbstractMap<Coordinates, Integer> closedset = new MapHache<>(initialsize);
        AbstractMap<Coordinates, Coordinates> camefrom = new MapHache<>(initialsize);
        AbstractMap<Coordinates, Float> g_score = new MapHache<>(initialsize);

        // Distance from start is obviously 0... good place to start
        g_score.put(start, 0f);
        openset.insert(getHDistance(start, end, heuristicMultiplier, terrainMinWeight), start);

        while (!openset.isEmpty())
        {
            Coordinates current = openset.deleteMin();  // Get an open node with the lowest f_score, ie the one which looks best at the time
            closedset.put(current, 0);

            if (current.equals(end))    // Yaayy! A path was found, and if A* works it should be the shortest one :p
                return new PathInfo(reconstructPath(current, camefrom), closedset, g_score.get(end));

            for (Coordinates neighbour : getNeighbours(current))
            {
                float weight = calculateWeight(current, neighbour);

                if (weight == -1)    // Wall...
                    continue;

                float tentative_gscore = g_score.get(current) + weight;
                Float g_scoreneighbour = g_score.get(neighbour);

                // If this neighbour is already processed and the gscore through the current node is not lower, we can skip to the next
                if (closedset.containsKey(neighbour) && g_scoreneighbour <= tentative_gscore)
                    continue;

                // If this is the first time at the neighbour, or the gscore through the current node is better, update stuff
                if (g_scoreneighbour == null || tentative_gscore < g_scoreneighbour)
                {
                    g_score.put(neighbour, tentative_gscore);
                    camefrom.put(neighbour, current);
                }

                // If the neighbour node is seen for the first time, ie not open and not closed, put it in the openset
                float tentative_fscore = tentative_gscore + getHDistance(neighbour, end, heuristicMultiplier, terrainMinWeight);
                if (!openset.containsKey(neighbour) && !closedset.containsKey(neighbour))
                    openset.insert(tentative_fscore, neighbour);

                // We can safely try to decrease the key, if the value is higher or doesnt exist, nothing will happen
                openset.decreaseKey(tentative_fscore, neighbour);
            }
        }

        // If we get here, no path was found... return stuff anyway to show pretty graphs
        return new PathInfo(null, closedset, null);
    } */

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_neighbours_top_left() {
        assert_eq!(
            get_neighbours(Point { x: 0, y: 0 }, 10, 10),
            vec![1, 10, 11]
        );
    }

    #[test]
    fn test_get_neighbours_top_right() {
        assert_eq!(
            get_neighbours(Point { x: 9, y: 0 }, 10, 10),
            vec![8, 18, 19]
        );
    }

    #[test]
    fn test_get_neighbours_bottom_left() {
        assert_eq!(
            get_neighbours(Point { x: 0, y: 9 }, 10, 10),
            vec![80, 81, 91]
        );
    }

    #[test]
    fn test_get_neighbours_bottom_right() {
        assert_eq!(
            get_neighbours(Point { x: 9, y: 9 }, 10, 10),
            vec![88, 89, 98]
        );
    }
}

/*
0  1  2  3  4  5  6  7  8  9
10 11 12 13 14 15 16 17 18 19
...
80 81 82 83 84 85 86 87 88 89
90 91 92 93 94 95 96 97 98 99
*/
