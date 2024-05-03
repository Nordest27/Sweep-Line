use crate::domain::{Point, Segment, Direction, SweepLineProblem, segments_intersection};


pub fn naive_intersection_solver(sweep_line_problem: &mut SweepLineProblem) {
    sweep_line_problem.result.clear();
    for segment_i in sweep_line_problem.segments.iter() {
        for segment_j in sweep_line_problem.segments.iter() {
            if segment_i != segment_j {
                match segments_intersection(segment_i, segment_j) {
                    Some(p) => {
                        if p.ini == p.end {
                            sweep_line_problem.result.push(Segment {
                                ini: p.ini.clone(),
                                end: p.ini.clone(),
                            });
                        } else {
                            sweep_line_problem.result.push(Segment {
                                ini: p.ini.clone(),
                                end: p.end.clone(),
                            });
                        }
                    }
                    None => (),
                }
            }
        }
    }
}