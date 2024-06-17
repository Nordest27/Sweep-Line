mod example_generator;
mod domain;
mod visualization;
mod solvers;

use rand;
use std::cmp::PartialEq;
use domain::{Point, Segment, Direction, SweepLineProblem, segments_intersection, test_treap};
use solvers::test_sweep_line_solver;
use visualization::visualization;

fn main() {
    //test_treap();
    test_sweep_line_solver();
    //visualization().unwrap();
}
