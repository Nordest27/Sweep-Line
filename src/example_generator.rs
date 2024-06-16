use crate::domain::{Point, Segment, Direction, SweepLineProblem};

pub fn create_random_example(n_segments: i32) -> SweepLineProblem {
    let mut segments = Vec::new();
    let mut result = Vec::new();
    let mut time = 0.0;
    let mut basic_operations = 0;

    for _ in 0..n_segments {
        let ini = Point{x: rand::random::<f64>(), y: rand::random::<f64>()};
        let end = Point{x: rand::random::<f64>(), y: rand::random::<f64>()};
        segments.push(Segment{ini, end}.to_grid(0.01));
    }

    SweepLineProblem{segments, result, time, basic_operations}
}
