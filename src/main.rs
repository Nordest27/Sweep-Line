mod example_generator;
mod domain;
mod visualization;

use rand;
use std::cmp::PartialEq;
use std::{f32, f64};
use domain::{Point, Segment, Direction, SweepLineProblem};
use visualization::visualization;

fn point_on_segment(p: &Point, s: &Segment) -> bool {
    let x_min = f32::min(s.ini.x, s.end.x);
    let x_max = f32::max(s.ini.x, s.end.x);
    let y_min = f32::min(s.ini.y, s.end.y);
    let y_max = f32::max(s.ini.y, s.end.y);
    return x_min <= p.x && p.x <= x_max && y_min <= p.y && p.y <= y_max;
}

fn orientation(p: &Point, s: &Segment) -> Direction {
    let val = (s.end.y - s.ini.y) * (p.x - s.end.x) - (s.end.x - s.ini.x) * (p.y - s.end.y);
    if val == 0.0 {
        return Direction::Collinear
    }
    return if val > 0.0 {Direction::Clockwise} else {Direction::CounterClockwise};
}

fn segments_intersect(s1: &Segment, s2: &Segment) -> bool {
    let o1 = orientation(&s2.ini, s1);
    let o2 = orientation(&s2.end, s1);
    let o3 = orientation(&s1.ini, s2);
    let o4 = orientation(&s1.end, s2);

    if o1  != o2 && o3 != o4 { return true }

    if o1 == Direction::Collinear && point_on_segment(&s2.ini, s1) { return true }
    if o2 == Direction::Collinear && point_on_segment(&s2.end, s1) { return true }
    if o3 == Direction::Collinear && point_on_segment(&s1.ini, s2) { return true }
    if o4 == Direction::Collinear && point_on_segment(&s1.end, s2) { return true }

    return false;
}

fn main() {
    let s1 = Segment{ini: Point{x: 1.0, y: 1.0}, end: Point{x: 10.0, y: 1.0}};
    let s2 = Segment{ini: Point{x: 2.0, y: 2.0}, end: Point{x: 40.0, y: 0.0}};

    if segments_intersect(&s1, &s2) {
        println!("Segments intersect");
    } else {
        println!("Segments do not intersect");
    }
    visualization().unwrap();
}
