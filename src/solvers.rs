use std::cmp::Ordering;
use ggez::input::keyboard::KeyCode::E;
use std::collections::{HashMap, BinaryHeap};
use crate::domain::{
    Point, Segment, Direction, SweepLineProblem,
    Treap,
    segments_intersection
};
use crate::example_generator::create_random_example;


pub fn naive_intersection_solver(sweep_line_problem: &mut SweepLineProblem) {
    sweep_line_problem.result.clear();
    for i in 0..sweep_line_problem.segments.len() {
        for j in i+1..sweep_line_problem.segments.len() {
            let segment_i = &sweep_line_problem.segments[i];
            let segment_j = &sweep_line_problem.segments[j];
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

#[derive(Clone, Debug)]
#[derive(PartialEq, PartialOrd)]
enum EventType {
    Start,
    Cross,
    End,
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
struct Event {
    p1: Point,
    p2: Point,
    event_type: EventType,
    s1_i: usize,
    s2_i: usize
}

impl Eq for Event {}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.p1.x.partial_cmp(&self.p1.x).unwrap()
            .then_with(|| {
                if other.event_type != self.event_type {
                    match other.event_type {
                        EventType::Start | EventType::Cross => Ordering::Less,
                        EventType::End => Ordering::Greater,
                    }
                } else {
                    Ordering::Equal
                }
            })
            .then_with(|| other.p1.y.partial_cmp(&self.p1.y).unwrap())
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn check_for_intersection(
    segments: &Vec<Segment>,
    f_segment_i: usize,
    s_segment_i: usize,
    basic_operations: &mut i32
) -> Option<Event> {
    *basic_operations += 1;
    if let Some(intersection) = segments_intersection(
        &segments[f_segment_i], &segments[s_segment_i]
    ) {
        //println!("Adding event at point ({}, {})", p.ini.x, p.ini.y);
        return Some(Event {
            p1: intersection.ini,
            p2: intersection.end,
            event_type: EventType::Cross,
            s1_i: f_segment_i,
            s2_i: s_segment_i
        });
    }
    None
}

pub fn sweep_line_solver(sweep_line_problem: &mut SweepLineProblem) {
    sweep_line_problem.result.clear();
    let mut events: BinaryHeap<Event> = BinaryHeap::new();
    let mut segments_list = Vec::new();
    let mut segments_map = HashMap::new();
    let mut i: usize = 0;
    let size = sweep_line_problem.segments.len();
    for segment in &sweep_line_problem.segments{
        let mut aux_segment = segment.clone();
        if aux_segment.ini.x > aux_segment.end.x {
            aux_segment = Segment {
                ini: aux_segment.end,
                end: aux_segment.ini,
            };
        }
        //if aux_segment.ini.x == aux_segment.end.x { continue; }
        events.push(Event {
            p1: aux_segment.ini.clone(),
            p2: aux_segment.ini.clone(),
            event_type: EventType::Start,
            s1_i: i,
            s2_i: i
        });
        events.push(Event {
            p1: aux_segment.end.clone(),
            p2: aux_segment.end.clone(),
            event_type: EventType::End,
            s1_i: i,
            s2_i: i
        });
        segments_list.push(aux_segment.clone());
        segments_map.insert(aux_segment, i);
        i += 1;
    }
    let mut segments_tree = Treap::new();
    let mut iters = 0;
    while !events.is_empty() {
        iters += 1;
        if iters > size*size {break}
        /*println!("--------------------------------");
        println!("Events");
        let mut events_clone = events.clone();
        while let Some(Event {p, event_type, s1_i, s2_i}) = events_clone.pop() {
            println!("({}, {}) - {:?} - ({}, {})", p.x, p.y, event_type, s1_i, s2_i);
        }*/
        let Some(Event {p1, p2, event_type, s1_i, s2_i}) = events.pop()
            else { panic!("This cant happen!") };

        //segments_tree.print_inorder();
        match event_type {
            EventType::Start => {
                let s1 = &segments_list[s1_i];
                let successor = segments_tree.successor(
                    s1, &mut sweep_line_problem.basic_operations);
                let predecessor = segments_tree.predecessor(
                    s1, &mut sweep_line_problem.basic_operations);
                if let Some(successor) = successor {
                    if let Some(event) = check_for_intersection(
                        &segments_list, s1_i, segments_map[successor],
                        &mut sweep_line_problem.basic_operations
                    ) { events.push(event) }
                }
                if let Some(predecessor) = predecessor {
                    if let Some(event) = check_for_intersection(
                        &segments_list, segments_map[predecessor], s1_i,
                        &mut sweep_line_problem.basic_operations
                    ){ events.push(event) }
                }
                segments_tree.insert(s1.clone(), &mut sweep_line_problem.basic_operations);
            }
            EventType::End => {
                let s1= &segments_list[s1_i];
                if segments_tree.remove(&s1, &mut sweep_line_problem.basic_operations) {
                    let predecessor = segments_tree.predecessor(
                        &s1, &mut sweep_line_problem.basic_operations);
                    let successor = segments_tree.successor(
                        &s1, &mut sweep_line_problem.basic_operations);
                    if let (Some(predecessor), Some(successor)) =
                        (predecessor, successor) {
                        if let Some(event) = check_for_intersection(
                            &segments_list,
                            segments_map[predecessor],
                            segments_map[successor],
                            &mut sweep_line_problem.basic_operations
                        ) { events.push(event) }
                    }
                }
                else {
                    /*
                    println!("ERROR: s1 must be in the tree");
                    println!("S1: ({}, {}), ({}, {})",
                             s1.ini.x, s1.ini.y, s1.end.x, s1.end.y);
                    segments_tree.print_inorder();
                     */
                }
            }
            EventType::Cross => {
                sweep_line_problem.result.push(Segment {
                    ini: p1.clone(),
                    end: p2.clone(),
                });

                if segments_list[s1_i].ini == segments_list[s1_i].end ||
                    segments_list[s2_i].ini == segments_list[s2_i].end ||
                    p1 == segments_list[s1_i].end || p1 == segments_list[s2_i].end ||
                    p1 == segments_list[s1_i].ini || p1 == segments_list[s2_i].ini
                { continue; }

                //println!("Crossing at point ({}, {})", p.x, p.y);
                if !segments_tree.remove(
                    &segments_list[s1_i], &mut sweep_line_problem.basic_operations) {
                    /*println!("S1: ({}, {}), ({}, {})",
                             segments_list[s1_i].ini.x,
                             segments_list[s1_i].ini.y,
                             segments_list[s1_i].end.x,
                             segments_list[s1_i].end.y);
                    println!("ERROR: s1 must be in the tree");
                    segments_tree.print_inorder();*/
                    continue;
                }
                if !segments_tree.remove(
                    &segments_list[s2_i], &mut sweep_line_problem.basic_operations) {
                    /*println!("S2: ({}, {}), ({}, {})",
                             segments_list[s2_i].ini.x,
                             segments_list[s2_i].ini.y,
                             segments_list[s2_i].end.x,
                             segments_list[s2_i].end.y);
                    println!("ERROR: s2 must be in the tree");
                    segments_tree.print_inorder();*/
                    segments_tree.insert(
                        segments_list[s1_i].clone(), &mut sweep_line_problem.basic_operations
                    );
                    continue;
                }
                segments_map.remove(&segments_list[s1_i]);
                segments_map.remove(&segments_list[s2_i]);

                segments_list[s1_i] = Segment {
                    ini: p1.clone(),
                    end: segments_list[s1_i].end.clone(),
                };
                segments_list[s2_i] = Segment {
                    ini: p1.clone(),
                    end: segments_list[s2_i].end.clone(),
                };

                segments_map.insert(segments_list[s1_i].clone(), s1_i);
                segments_map.insert(segments_list[s2_i].clone(), s2_i);

                let successor = segments_tree.successor(
                    &segments_list[s1_i], &mut sweep_line_problem.basic_operations);
                let predecessor = segments_tree.predecessor(
                    &segments_list[s2_i], &mut sweep_line_problem.basic_operations);

                if let Some(successor) = successor {
                    let successor_i = segments_map[&successor];
                    if successor_i == s2_i{
                        /*println!("S1 successor: ({}, {})", successor.end.x, successor.end.y);
                        segments_tree.print_inorder();
                        println!("ERROR: s1 successor and s2 must not be the same");*/
                    }
                    else {
                        if let Some(event) = check_for_intersection(
                            &segments_list, s1_i, successor_i,
                            &mut sweep_line_problem.basic_operations
                        ) { events.push(event) }
                    }
                }
                if let Some(predecessor) = predecessor {
                    let predecessor_i = segments_map[&predecessor];
                    if predecessor_i == s1_i{
                        /*println!("S2 predecessor: ({}, {})", predecessor.end.x, predecessor.end.y);
                        segments_tree.print_inorder();
                        println!("ERROR: s2 predecessor and s1 must not be the same");*/
                    }
                    else {
                        if let Some(event) = check_for_intersection(
                            &segments_list, predecessor_i, s2_i,
                            &mut sweep_line_problem.basic_operations
                        ) { events.push(event) }
                    }
                }
                segments_tree.insert(
                    segments_list[s1_i].clone(), &mut sweep_line_problem.basic_operations);
                segments_tree.insert(
                    segments_list[s2_i].clone(), &mut sweep_line_problem.basic_operations);
            }
        }
    }
}



pub fn test_sweep_line_solver() {
    let mut naive_basic_operations = Vec::new();
    let mut sweep_line_basic_operations = Vec::new();
    for i in 0..10 {
        println!("{}", i);
        let mut sweep_line_problem = create_random_example(10000);
        let mut naive_sweep_line_problem = sweep_line_problem.clone();
        naive_intersection_solver(&mut naive_sweep_line_problem);
        sweep_line_solver(&mut sweep_line_problem);
        naive_basic_operations.push(naive_sweep_line_problem.basic_operations);
        sweep_line_basic_operations.push(sweep_line_problem.basic_operations);
        /*
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
        */
    }
    //Write as csv in the terminal
    println!("Naive basic operations, Sweep line basic operations");
    for (naive, sweep_line) in naive_basic_operations.iter().zip(sweep_line_basic_operations.iter()) {
        println!("{}, {}", naive, sweep_line);
    }
}