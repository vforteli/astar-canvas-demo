use std::{convert::TryInto, ops::Mul};

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
pub fn calculate_weight(from: Point, to: Point, weights: Vec<f32>, width: u32) -> f32 {
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
    from: Point,
    to: Point,
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
