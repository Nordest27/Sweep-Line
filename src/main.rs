mod example_generator;
mod domain;
mod visualization;
mod solvers;

use rand;
use std::cmp::PartialEq;
use domain::{Point, Segment, Direction, SweepLineProblem, segments_intersection, test_treap};
use visualization::visualization;

fn main() {
    test_treap();
    /*
    let s1 = Segment{ini: Point{x: 1.0, y: 1.0}, end: Point{x: 10.0, y: 1.0}};
    let s2 = Segment{ini: Point{x: 2.0, y: 2.0}, end: Point{x: 40.0, y: 0.0}};

    match segments_intersection(&s1, &s2) {
        Some(p) => if p.ini == p.end {
            println!("Segments intersect at point ({}, {})", p.ini.x, p.ini.y);
        } else {
            println!("Segments intersect at segment with initial point ({}, {}) and end point ({}, {})", p.ini.x, p.ini.y, p.end.x, p.end.y);
        },
        None => println!("Segments do not intersect"),
    }
    visualization().unwrap();
    */

}
