use std::cmp::Ordering;
use ggez::input::keyboard::KeyCode::E;
use crate::domain::{
    Point, Segment, Direction, SweepLineProblem,
    Treap, EPSILON,
    segments_intersection
};
use crate::example_generator::create_random_example;


pub fn naive_intersection_solver(sweep_line_problem: &mut SweepLineProblem) {
    sweep_line_problem.result.clear();
    for segment_i in sweep_line_problem.segments.iter() {
        for segment_j in sweep_line_problem.segments.iter() {
            if segment_j > segment_i {
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
                sweep_line_problem.basic_operations += 1;
            }
        }
    }
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
enum EventType {
    Start,
    End,
    Cross,
}

fn check_for_intersection(
    f_segment: &Segment,
    s_segment: &Segment,
) -> Vec<(Point, EventType, Segment, Segment)> {
    let mut events = Vec::new();
    if let Some(p) = segments_intersection(f_segment, s_segment) {
        //println!("Adding event at point ({}, {})", p.ini.x, p.ini.y);
        events.push((p.ini, EventType::Cross, f_segment.clone(), s_segment.clone()));
    }
    events
}

pub fn sweep_line_solver(sweep_line_problem: &mut SweepLineProblem) {
    sweep_line_problem.result.clear();
    let mut events: Vec<(Point, EventType, Segment, Segment)> = Vec::new();

    for segment in &sweep_line_problem.segments {
        let mut aux_segment = segment.clone();
        if aux_segment.ini.x > aux_segment.end.x {
            std::mem::swap(&mut aux_segment.ini, &mut aux_segment.end);
        }
        if aux_segment.ini.x == aux_segment.end.x { continue; }
        events.push((aux_segment.ini.clone(), EventType::Start, aux_segment.clone(), aux_segment.clone()));
        events.push((aux_segment.end.clone(), EventType::End, aux_segment.clone(), aux_segment.clone()));
    }
    let mut segments_tree = Treap::new();
    while !events.is_empty() {
        sweep_line_problem.basic_operations += 1;
        events.sort_by(|a, b| {
            a.0.x.partial_cmp(&b.0.x).unwrap()
                .then_with(|| {
                    if a.1 != b.1 {
                        match a.1 {
                            EventType::Start | EventType::Cross => Ordering::Less,
                            EventType::End => Ordering::Greater,
                        }
                    } else {
                        Ordering::Equal
                    }
                })
                .then_with(|| a.0.y.partial_cmp(&b.0.y).unwrap())
        });
        /*
        println!("--------------------------------");
        println!("Events");
        for event in &events {
            println!("({}, {}) - ({:?})", event.0.x, event.0.y, event.1);
        }*/
        let (p, event_type, s1, s2) = events.remove(0);
        //segments_tree.print_inorder();
        match event_type {
            EventType::Start => {
                let successor = segments_tree.successor(&s1, p.x);
                let predecessor = segments_tree.predecessor(&s1, p.x);
                if let Some(successor) = successor {
                    events.extend(check_for_intersection(&s1, successor));
                }
                if let Some(predecessor) = predecessor {
                    events.extend(check_for_intersection(predecessor, &s1));
                }
                segments_tree.insert(s1.clone(), s1.ini.x);
            }
            EventType::End => {
                if segments_tree.remove(&s1, p.x) {
                    let predecessor = segments_tree.predecessor(&s1, p.x);
                    let successor = segments_tree.successor(&s1, p.x);
                    if let (Some(predecessor), Some(successor)) =
                        (predecessor, successor) {
                        events.extend(check_for_intersection(predecessor, successor));
                    }
                }
            }
            EventType::Cross => {
                sweep_line_problem.result.push(Segment {
                    ini: p.clone(),
                    end: p.clone(),
                });
                //println!("Crossing at point ({}, {})", p.x, p.y);
                let mut rank = segments_tree.public_get_rank(&s1, p.x);
                rank = rank.min(segments_tree.public_get_rank(&s2, p.x));

                if !segments_tree.remove_ith(rank) { continue }
                if !segments_tree.remove_ith(rank) {
                    segments_tree.insert(s1.clone(), p.x);
                }
                let successor = segments_tree.successor(&s1, p.x);
                let predecessor = segments_tree.predecessor(&s2, p.x);
                segments_tree.print_inorder();
                if let Some(successor) = successor {
                    if *successor == s2 {
                        println!("S1 successor: ({}, {})", successor.end.x, successor.end.y);
                        segments_tree.print_inorder();
                        println!("ERROR: s1 successor and s2 must not be the same");
                    }
                    else {
                        events.extend(check_for_intersection(&s1, successor));
                    }
                }
                if let Some(predecessor) = predecessor {
                    if *predecessor == s1 {
                        println!("S2 predecessor: ({}, {})", predecessor.end.x, predecessor.end.y);
                        segments_tree.print_inorder();
                        println!("ERROR: s2 predecessor and s1 must not be the same");
                    }
                    else {
                        events.extend(check_for_intersection(predecessor, &s2));
                    }
                }
                segments_tree.insert_ith(s1.clone(), rank);
                segments_tree.insert_ith(s2.clone(), rank);
            }
        }
    }
}

pub fn test_sweep_line_solver() {
    let mut naive_basic_operations = Vec::new();
    let mut sweep_line_basic_operations = Vec::new();
    for _ in 0..1000 {
        let mut sweep_line_problem = create_random_example(50);
        let mut naive_sweep_line_problem = sweep_line_problem.clone();
        naive_intersection_solver(&mut naive_sweep_line_problem);
        sweep_line_solver(&mut sweep_line_problem);
        naive_basic_operations.push(naive_sweep_line_problem.basic_operations);
        sweep_line_basic_operations.push(sweep_line_problem.basic_operations);
        continue;
        naive_sweep_line_problem.result.sort_by(|a, b| {
            a.ini.x.partial_cmp(&b.ini.x).unwrap()
                .then_with(|| a.ini.y.partial_cmp(&b.ini.y).unwrap())
                .then_with(|| a.end.x.partial_cmp(&b.end.x).unwrap())
                .then_with(|| a.end.y.partial_cmp(&b.end.y).unwrap())
        });
        sweep_line_problem.result.sort_by(|a, b| {
            a.ini.x.partial_cmp(&b.ini.x).unwrap()
                .then_with(|| a.ini.y.partial_cmp(&b.ini.y).unwrap())
                .then_with(|| a.end.x.partial_cmp(&b.end.x).unwrap())
                .then_with(|| a.end.y.partial_cmp(&b.end.y).unwrap())
        });
        for (s1, s2) in naive_sweep_line_problem.result.iter().zip(sweep_line_problem.result.iter()) {
            if s1.to_grid(0.1) != s2.to_grid(0.1) {
                println!("Naive result:");
                for segment in naive_sweep_line_problem.result.iter() {
                    println!("({}, {}) - ({}, {})", segment.ini.x, segment.ini.y, segment.end.x, segment.end.y);
                }
                println!("Sweep line result:");
                for segment in sweep_line_problem.result.iter() {
                    println!("({}, {}) - ({}, {})", segment.ini.x, segment.ini.y, segment.end.x, segment.end.y);
                }
                panic!("Results are different");
            }
        }
    }
    //Write as csv in the terminal
    println!("Naive basic operations, Sweep line basic operations");
    for (naive, sweep_line) in naive_basic_operations.iter().zip(sweep_line_basic_operations.iter()) {
        println!("{}, {}", naive, sweep_line);
    }
}