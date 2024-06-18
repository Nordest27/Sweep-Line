use crate::domain::{Point, Segment, Direction, SweepLineProblem};

pub fn create_random_example(n_segments: i32) -> SweepLineProblem {
    let mut segments = Vec::new();
    let mut result = Vec::new();
    let mut time = 0.0;
    let mut basic_operations = 0;

    for _ in 0..n_segments {
        while true {
            let ini = Point {
                x: (rand::random::<i32>() % 1000).abs() as f64,
                y: (rand::random::<i32>() % 1000).abs() as f64,
            };
            let end = Point {
                x: (rand::random::<i32>() % 1000).abs() as f64,
                y: (rand::random::<i32>() % 1000).abs() as f64,
            };
            let segment = Segment{ini, end};
            if segment.ini.x != segment.end.x && segment.ini.y != segment.end.y {
                segments.push(segment);
                break;
            }
        }
    }

    SweepLineProblem{segments, result, time, basic_operations}
}
