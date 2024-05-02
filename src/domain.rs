pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Segment {
    pub ini: Point,
    pub end: Point,
}

pub enum Direction {
    Collinear,
    Clockwise,
    CounterClockwise,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Collinear, Direction::Collinear) => true,
            (Direction::Clockwise, Direction::Clockwise) => true,
            (Direction::CounterClockwise, Direction::CounterClockwise) => true,
            _ => false,
        }
    }
}

pub struct SweepLineProblem {
    pub segments: Vec<Segment>,
    pub result: Vec<Point>,
    pub time: f32,
    pub basic_operations: i32
}
