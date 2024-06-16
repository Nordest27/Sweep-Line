use std::cmp::Ordering;
use crate::domain::{
    Point, Segment, Direction, SweepLineProblem,
    Treap,
    segments_intersection
};


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

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
enum EventType {
    Start,
    End,
    Cross,
}

fn check_for_intersection(
    f_segment: &Segment,
    s_segment: &Segment,
) -> Vec<(Point, EventType, Segment, Segment)>{
    let mut events = Vec::new();
    if let Some(p) = segments_intersection(f_segment, s_segment) {
        if p.ini == p.end {
            events.push((p.ini, EventType::Cross, f_segment.clone(), s_segment.clone()));
        } else {
            events.push((p.ini, EventType::Cross, f_segment.clone(), s_segment.clone()));
            events.push((p.end, EventType::Cross, f_segment.clone(), s_segment.clone()));
        }
    }
    events
}

pub fn sweep_line_solver(sweep_line_problem: &mut SweepLineProblem) {
    sweep_line_problem.result.clear();
    let mut events: Vec<(Point, EventType, Segment, Segment)> = Vec::new();

    for segment in &sweep_line_problem.segments {
        events.push((segment.ini.clone(), EventType::Start, segment.clone(), segment.clone()));
        events.push((segment.end.clone(), EventType::End, segment.clone(), segment.clone()));
    }

    events.sort_by(|a, b| {
        if a.0.x == b.0.x {
            if a.0.y == b.0.y {
                a.1.partial_cmp(&b.1).unwrap()
            }
            else {
                a.0.y.partial_cmp(&b.0.y).unwrap()
            }
        } else {
            a.0.x.partial_cmp(&b.0.x).unwrap()
        }
    });

    let mut segments_tree = Treap::new();
    println!("--------------------------------");
    while !events.is_empty() {
        let (p, event_type, s1, s2) = events.remove(0);
        segments_tree.print_inorder();
        match event_type {
            EventType::Start => {
                let successor = segments_tree.successor(&s1);
                let predecessor = segments_tree.predecessor(&s1);
                if let Some(successor) = successor {
                    events.extend(check_for_intersection(&s1, successor));
                }
                if let Some(predecessor) = predecessor {
                    events.extend(check_for_intersection(predecessor, &s1));
                }
                segments_tree.insert(s1.clone());
            }
            EventType::End => {
                let predecessor = segments_tree.predecessor(&s1);
                let successor = segments_tree.successor(&s1);
                if let (Some(predecessor), Some(successor)) =
                    (predecessor, successor) {
                    events.extend(check_for_intersection(predecessor, successor));
                }
                segments_tree.remove(&s1);
            }
            EventType::Cross => {
                sweep_line_problem.result.push(Segment {
                    ini: p.clone(),
                    end: p.clone(),
                });
                println!("Crossing at point ({}, {})", p.x, p.y);
                segments_tree.remove(&s1);
                segments_tree.remove(&s2);
                segments_tree.insert(Segment {ini: p.clone(), end: s2.end.clone()});
                segments_tree.insert(Segment {ini: p.clone(), end: s1.end.clone()});
                let successor = segments_tree.successor(&s1);
                if let Some(successor) = successor {
                    events.extend(check_for_intersection(&s1, successor));
                }
                let predecessor = segments_tree.predecessor(&s2);
                if let Some(predecessor) = predecessor {
                    events.extend(check_for_intersection(predecessor, &s2));
                }
            }
        }
    }
}