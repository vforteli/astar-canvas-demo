use std::convert::TryInto;

use astar_rust_wasm::astar::{coordinates_to_index, find_path, normalize, rgb_to_hsv, Point};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut bytes: &[u8] = include_bytes!("../assets/castle.bmp");
    let image = bmp::from_reader(&mut bytes).unwrap();
    let height = image.get_height();
    let width = image.get_width();

    let mut cell_weights = vec![0.0; (width * height).try_into().unwrap()];

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let hsv = rgb_to_hsv(pixel.r, pixel.g, pixel.b);
            let inverted_brighntess = (hsv.brightness - 1.0).abs();
            let normalized_brighntess = normalize(0.0, 1.0, 1.0, 10.0, inverted_brighntess);

            cell_weights[coordinates_to_index(width, x, y) as usize] = normalized_brighntess;
        }
    }

    find_path(
        Point { x: 0, y: 37 },
        Point { x: 99, y: 12 },
        width,
        height,
        1,
        1.0,
        &cell_weights,
    );

    c.bench_function("fib 20", |b| {
        b.iter(|| {
            find_path(
                Point { x: 0, y: 37 },
                Point { x: 99, y: 12 },
                width,
                height,
                1,
                1.0,
                &cell_weights,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
