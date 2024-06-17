use std::cmp::Ordering;
use ggez::input::keyboard::KeyCode::E;
use std::collections::HashMap;
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
    segments: &Vec<Segment>,
    f_segment_i: usize,
    s_segment_i: usize,
) -> Vec<(Point, EventType, usize, usize)> {
    let mut events = Vec::new();
    if let Some(p) = segments_intersection(
        &segments[f_segment_i], &segments[s_segment_i]
    ) {
        //println!("Adding event at point ({}, {})", p.ini.x, p.ini.y);
        events.push((p.ini, EventType::Cross, f_segment_i, s_segment_i));
    }
    events
}

pub fn sweep_line_solver(sweep_line_problem: &mut SweepLineProblem) {
    sweep_line_problem.result.clear();
    let mut events: Vec<(Point, EventType, usize, usize)> = Vec::new();
    let mut segments_list = Vec::new();
    let mut segments_map = HashMap::new();
    let mut i: usize = 0;
    for segment in &sweep_line_problem.segments{
        let mut aux_segment = segment.clone();
        if aux_segment.ini.x > aux_segment.end.x {
            aux_segment = Segment {
                ini: aux_segment.end,
                end: aux_segment.ini,
            };
        }
        if aux_segment.ini.x == aux_segment.end.x { continue; }
        events.push((aux_segment.ini.clone(), EventType::Start, i, i));
        events.push((aux_segment.end.clone(), EventType::End, i, i));
        segments_list.push(aux_segment.clone());
        segments_map.insert(aux_segment, i);
        i += 1;
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

        //println!("--------------------------------");
        //println!("Events");
        //for event in &events {
        //    println!("({}, {}) - ({:?})", event.0.x, event.0.y, event.1);
        //}
        let (p, event_type, s1_i, s2_i) = events.remove(0);
        segments_tree.print_inorder();
        match event_type {
            EventType::Start => {
                let s1 = &segments_list[s1_i];
                let successor = segments_tree.successor(s1);
                let predecessor = segments_tree.predecessor(s1);
                if let Some(successor) = successor {
                    events.extend(check_for_intersection(
                        &segments_list, s1_i, segments_map[successor])
                    );
                }
                if let Some(predecessor) = predecessor {
                    events.extend(check_for_intersection(
                        &segments_list, segments_map[predecessor], s1_i)
                    );
                }
                segments_tree.insert(s1.clone());
            }
            EventType::End => {
                let s1= &segments_list[s1_i];
                if segments_tree.remove(&s1) {
                    let predecessor = segments_tree.predecessor(&s1);
                    let successor = segments_tree.successor(&s1);
                    if let (Some(predecessor), Some(successor)) =
                        (predecessor, successor) {
                        events.extend(check_for_intersection(
                            &segments_list,
                            segments_map[predecessor],
                            segments_map[successor])
                        );
                    }
                }
                else {
                    println!("ERROR: s1 must be in the tree");
                    println!("S1: ({}, {}), ({}, {})",
                             s1.ini.x, s1.ini.y, s1.end.x, s1.end.y);
                    segments_tree.print_inorder();
                }
            }
            EventType::Cross => {
                sweep_line_problem.result.push(Segment {
                    ini: p.clone(),
                    end: p.clone(),
                });
                //println!("Crossing at point ({}, {})", p.x, p.y);
                if !segments_tree.remove(&segments_list[s1_i]) {
                    println!("S1: ({}, {}), ({}, {})",
                             segments_list[s1_i].ini.x,
                             segments_list[s1_i].ini.y,
                             segments_list[s1_i].end.x,
                             segments_list[s1_i].end.y);
                    println!("ERROR: s1 must be in the tree");
                    segments_tree.print_inorder();
                    continue;
                }
                if !segments_tree.remove(&segments_list[s2_i]) {
                    println!("S2: ({}, {}), ({}, {})",
                             segments_list[s2_i].ini.x,
                             segments_list[s2_i].ini.y,
                             segments_list[s2_i].end.x,
                             segments_list[s2_i].end.y);
                    println!("ERROR: s2 must be in the tree");
                    segments_tree.print_inorder();
                    segments_tree.insert(segments_list[s1_i].clone());
                    continue;
                }
                segments_map.remove(&segments_list[s1_i]);
                segments_map.remove(&segments_list[s2_i]);

                segments_list[s1_i] = Segment {
                    ini: p.clone(),
                    end: segments_list[s1_i].end.clone(),
                };
                segments_list[s2_i] = Segment {
                    ini: p.clone(),
                    end: segments_list[s2_i].end.clone(),
                };

                segments_map.insert(segments_list[s1_i].clone(), s1_i);
                segments_map.insert(segments_list[s2_i].clone(), s2_i);
                segments_tree.insert(segments_list[s1_i].clone());
                segments_tree.insert(segments_list[s2_i].clone());

                let successor = segments_tree.successor(&segments_list[s1_i]);
                let predecessor = segments_tree.predecessor(&segments_list[s2_i]);

                if let Some(successor) = successor {
                    let successor_i = segments_map[&successor];
                    if successor_i == s2_i{
                        println!("S1 successor: ({}, {})", successor.end.x, successor.end.y);
                        segments_tree.print_inorder();
                        println!("ERROR: s1 successor and s2 must not be the same");
                    }
                    else {
                        events.extend(check_for_intersection(
                            &segments_list, s1_i, successor_i
                        ));
                    }
                }
                if let Some(predecessor) = predecessor {
                    let predecessor_i = segments_map[&predecessor];
                    if predecessor_i == s1_i{
                        println!("S2 predecessor: ({}, {})", predecessor.end.x, predecessor.end.y);
                        segments_tree.print_inorder();
                        println!("ERROR: s2 predecessor and s1 must not be the same");
                    }
                    else {
                        events.extend(check_for_intersection(
                            &segments_list, predecessor_i, s2_i
                        ));
                    }
                }
            }
        }
    }
}

pub fn test_sweep_line_solver() {
    let mut naive_basic_operations = Vec::new();
    let mut sweep_line_basic_operations = Vec::new();
    for _ in 0..1000 {
        let mut sweep_line_problem = create_random_example(3);
        let mut naive_sweep_line_problem = sweep_line_problem.clone();
        naive_intersection_solver(&mut naive_sweep_line_problem);
        sweep_line_solver(&mut sweep_line_problem);
        naive_basic_operations.push(naive_sweep_line_problem.basic_operations);
        sweep_line_basic_operations.push(sweep_line_problem.basic_operations);
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