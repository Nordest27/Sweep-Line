use std::io::{BufRead, Write};
#[derive(Clone, PartialOrd)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn to_grid(&self, grid_size: f64) -> Point {
        return Point {
            x: (self.x / grid_size).round() * grid_size,
            y: (self.y / grid_size).round() * grid_size,
        };
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Segment {
    pub ini: Point,
    pub end: Point,
}
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

impl Segment {
    pub fn to_grid(&self, grid_size: f64) -> Segment {
        return Segment {
            ini: self.ini.to_grid(grid_size),
            end: self.end.to_grid(grid_size),
        };
    }

    pub fn interpolate_y(&self, x: f64) -> f64 {
        if self.ini.x == self.end.x {
            return self.ini.y;
        }
        return self.ini.y + (self.end.y - self.ini.y) * (x - self.ini.x) / (self.end.x - self.ini.x);
    }

    pub fn slope(&self) -> f64 {
        if self.end.x == self.ini.x {
            return f64::INFINITY;
        }
        return (self.end.y - self.ini.y) / (self.end.x - self.ini.x);
    }
}

impl Hash for Segment
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.ini.x.to_bits().hash(state),
        self.ini.y.to_bits().hash(state),
        self.end.x.to_bits().hash(state),
        self.end.y.to_bits().hash(state)).hash(state);
    }

}
impl Eq for Segment {}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        // Custom ordering logic, e.g., by comparing start points or any other criteria
        self.ini.y.partial_cmp(&other.ini.y).unwrap().then_with(|| {
            let self_slope = self.slope();
            let other_slope = other.slope();
            if self_slope.is_infinite() && other_slope.is_infinite() {
                return Ordering::Equal;
            }
            if self_slope.is_infinite() {
                return Ordering::Less;
            }
            if other_slope.is_infinite() {
                return Ordering::Greater;
            }
            return self_slope.partial_cmp(&other_slope).unwrap().then_with(|| {
                self.ini.x.partial_cmp(&other.ini.x).unwrap().then_with(|| {
                    self.end.y.partial_cmp(&other.end.y).unwrap().then_with(|| {
                        self.end.x.partial_cmp(&other.end.x).unwrap()
                    })
                })
            })
        })
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Collinear,
    Clockwise,
    CounterClockwise,
}

#[derive(Clone)]
pub struct SweepLineProblem {
    pub segments: Vec<Segment>,
    pub result: Vec<Segment>,
    pub time: f64,
    pub basic_operations: i32
}

impl SweepLineProblem {

    pub fn load(path: &str) -> SweepLineProblem {
        let content = std::fs::read_to_string(path).unwrap();
        let mut lines = content.lines();
        let n_segments = lines.next().unwrap().parse::<i32>().unwrap();
        let mut segments = Vec::new();
        for _ in 0..n_segments {
            let line = lines.next().unwrap();
            let mut values = line.split_whitespace();
            let x1 = values.next().unwrap().parse::<f64>().unwrap();
            let y1 = values.next().unwrap().parse::<f64>().unwrap();
            let x2 = values.next().unwrap().parse::<f64>().unwrap();
            let y2 = values.next().unwrap().parse::<f64>().unwrap();
            segments.push(Segment{ini: Point{x: x1, y: y1}, end: Point{x: x2, y: y2}});
        }
        return SweepLineProblem{segments, result: Vec::new(), time: 0.0, basic_operations: 0};
    }

    pub fn save(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        let mut content = String::new();
        content.push_str(&format!("{}\n", self.segments.len()));
        for segment in self.segments.iter() {
            content.push_str(
                &format!(
                    "{} {} {} {}\n",
                    segment.ini.x, segment.ini.y, segment.end.x, segment.end.y
                )
            );
        }
        file.write_all(content.as_bytes()).unwrap();
    }

}
pub fn distance(p1: &Point, p2: &Point) -> f64 {
    return ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt();
}

pub fn intersection_point(s1: &Segment, s2: &Segment) -> Segment {
    let a1 = s1.end.y - s1.ini.y;
    let b1 = s1.ini.x - s1.end.x;
    let c1 = a1 * s1.ini.x + b1 * s1.ini.y;

    let a2 = s2.end.y - s2.ini.y;
    let b2 = s2.ini.x - s2.end.x;
    let c2 = a2 * s2.ini.x + b2 * s2.ini.y;

    let determinant = a1 * b2 - a2 * b1;

    let i_point = Point {
        x: (b2 * c1 - b1 * c2) / determinant,
        y: (a1 * c2 - a2 * c1) / determinant,
    };

    return Segment {ini: i_point.clone(), end: i_point};
}

pub fn collinear_point_on_segment(p: &Point, s: &Segment) -> bool {
    let x_min = f64::min(s.ini.x, s.end.x);
    let x_max = f64::max(s.ini.x, s.end.x);
    let y_min = f64::min(s.ini.y, s.end.y);
    let y_max = f64::max(s.ini.y, s.end.y);
    return x_min <= p.x && p.x <= x_max && y_min <= p.y && p.y <= y_max;
}

pub fn orientation(p: &Point, s: &Segment) -> Direction {
    let val = (s.end.y - s.ini.y) * (p.x - s.end.x) - (s.end.x - s.ini.x) * (p.y - s.end.y);
    if val == 0.0 {
        return Direction::Collinear
    }
    return if val > 0.0 {Direction::Clockwise} else {Direction::CounterClockwise};
}


pub fn segments_intersection(s1: &Segment, s2: &Segment) -> Option<Segment> {
    let o1 = orientation(&s2.ini, s1);
    let o2 = orientation(&s2.end, s1);
    let o3 = orientation(&s1.ini, s2);
    let o4 = orientation(&s1.end, s2);

    if o1  != o2 && o3 != o4 { return Some(intersection_point(s1, s2)) }

    // Return the intersection segment if the segments are collinear
    let s2_ini_intersects_s1 = o1 == Direction::Collinear && collinear_point_on_segment(&s2.ini, s1);
    let s2_end_intersects_s1 = o2 == Direction::Collinear && collinear_point_on_segment(&s2.end, s1);
    let s1_ini_intersects_s2 = o3 == Direction::Collinear && collinear_point_on_segment(&s1.ini, s2);
    let s1_end_intersects_s2 = o4 == Direction::Collinear && collinear_point_on_segment(&s1.end, s2);

    if s2_ini_intersects_s1 && s2_end_intersects_s1 { return Some((*s2).clone()); }
    if s1_ini_intersects_s2 && s1_end_intersects_s2 { return Some((*s1).clone()); }
    if s2_ini_intersects_s1 && s1_ini_intersects_s2 { return Some(Segment{ini: s2.ini.clone(), end: s1.ini.clone()}); }
    if s2_ini_intersects_s1 && s1_end_intersects_s2 { return Some(Segment{ini: s2.ini.clone(), end: s1.end.clone()}); }
    if s2_end_intersects_s1 && s1_ini_intersects_s2 { return Some(Segment{ini: s2.end.clone(), end: s1.ini.clone()}); }
    if s2_end_intersects_s1 && s1_end_intersects_s2 { return Some(Segment{ini: s2.end.clone(), end: s1.end.clone()}); }

    return None;
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Node{
    pub key: Segment,
    pub priority: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Clone)]
pub struct Treap {
    root: Option<Box<Node>>,
}

impl Treap {
    pub fn new() -> Treap {
        Treap { root: None }
    }

    fn split(
        &self, node: Option<Box<Node>>, key: &Segment, basic_operations: &mut i32
    ) -> (Option<Box<Node>>, Option<Box<Node>>) {
        *basic_operations += 1;
        match node {
            None => (None, None),
            Some(mut node) => {
                if *key > node.key {
                    let (left, right) =
                        self.split(node.right, key, basic_operations);
                    node.right = left;
                    (Some(node), right)
                } else if *key < node.key {
                    let (left, right) =
                        self.split(node.left, key, basic_operations);
                    node.left = right;
                    (left, Some(node))
                } else {
                    (node.left, node.right)
                }
            }
        }
    }

    pub fn insert(&mut self, key: Segment, basic_operations: &mut i32) {
        let new_node = Node {
            key: key,
            priority: rand::random::<i32>(),
            left: None,
            right: None,
        };
        let (left, right) =
            self.split(self.root.clone(), &new_node.key, basic_operations);
        self.root = self.merge(
            self.merge(
                left, Some(Box::new(new_node)), basic_operations
            ),
            right, basic_operations
        );
    }

    pub fn merge(
        &self, left: Option<Box<Node>>, right: Option<Box<Node>>, basic_operations: &mut i32
    ) -> Option<Box<Node>> {
        *basic_operations += 1;
        match (left, right) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(mut left), Some(mut right)) => {
                if left.priority > right.priority {
                    left.right = self.merge(left.right, Some(right), basic_operations);
                    Some(left)
                } else {
                    right.left = self.merge(Some(left), right.left, basic_operations);
                    Some(right)
                }
            }
        }
    }

    fn find_node(
        &self, node: Option<Box<Node>>, key: &Segment, basic_operations: &mut i32
    ) -> Option<Box<Node>> {
        *basic_operations += 1;
        match node {
            None => None,
            Some(node) => {
                if *key < node.key {
                    self.find_node(node.left, key, basic_operations)
                } else if *key > node.key {
                    self.find_node(node.right, key, basic_operations)
                } else {
                    Some(node)
                }
            }
        }
    }

    pub fn find(&self, key: &Segment, basic_operations: &mut i32) -> bool {
        self.find_node(self.root.clone(), key, basic_operations).is_some()
    }
    pub fn remove(&mut self, key: &Segment, basic_operations: &mut i32) -> bool {
        if !self.find(key, basic_operations) {
            return false;
        }
        let (left, right) =
            self.split(self.root.clone(), key, basic_operations);
        self.root = self.merge(left, right, basic_operations);
        return true;
    }


    pub fn successor(
        &self, key: &Segment, basic_operations: &mut i32
    ) -> Option<&Segment> {
        let mut current = &self.root;
        let mut successor = None;
        while let Some(node) = current {
            *basic_operations += 1;
            if *key < node.key {
                successor = Some(&node.key);
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        successor
    }

    pub fn predecessor(
        &self, key: &Segment, basic_operations: &mut i32
    ) -> Option<&Segment> {
        let mut current = &self.root;
        let mut predecessor = None;
        while let Some(node) = current {
            *basic_operations += 1;
            if *key > node.key {
                predecessor = Some(&node.key);
                current = &node.right;
            } else {
                current = &node.left;
            }
        }
        predecessor
    }

    fn inorder(&self, node: &Option<Box<Node>>) {
        match node {
            None => (),
            Some(node) => {
                self.inorder(&node.left);
                println!("(ini: ({}, {}), end: ({}, {}))",
                       node.key.ini.x, node.key.ini.y, node.key.end.x, node.key.end.y);
                self.inorder(&node.right);
            }
        }
    }
    pub fn print_inorder(&self) {
        println!("Inorder traversal:");
        self.inorder(&self.root);
        println!();
    }
}

pub fn test_treap() {
    let n = 50;
    let mut treap = Treap::new();
    for _ in 0..10000 {
        let ref mut basic_operations = 0;
        let x = rand::random::<f64>();
        let mut segments = Vec::new();
        for i in 0..n {
            let segment = Segment {
                ini: Point {
                    x: rand::random::<f64>(),
                    y: rand::random::<f64>(),
                },
                end: Point {
                    x: rand::random::<f64>(),
                    y: rand::random::<f64>(),
                }
            };
            treap.insert(segment.clone(), basic_operations);
            segments.push(segment.clone());
        }
        segments.sort_by(|a, b| a.cmp(b));
        // println!("---------------------------------------");
        println!("Segments inserted in the treap:");
        treap.print_inorder();
        //println!();
        println!("Segments inserted in the list:");
        for i in 0..n {
            println!("({}, {}), ({}, {}), ",
                   segments[i].ini.x, segments[i].ini.y, segments[i].end.x, segments[i].end.y);
        }
        /*
        println!();
        println!("Y values");
        for i in 0..n {
            print!("{}, ", segments[i].interpolate_y(0.0));
        }
        println!();*/
        //sleep(Duration::from_nanos(10000));
        for i in 0..n {
            //println!("Successor and predecessor of ({}, {}), ({}, {}): ",
            //       segments[i].ini.x, segments[i].ini.y, segments[i].end.x, segments[i].end.y);
            assert_eq!(treap.successor(&segments[i], basic_operations), if i == n-1 { None } else { Some(&segments[i + 1]) });
            assert_eq!(treap.predecessor(&segments[i], basic_operations), if i == 0 { None } else { Some(&segments[i - 1]) });
        }
        for i in 0..n {
            //println!("Removing ({}, {}), ({}, {})", segments[i].ini.x, segments[i].ini.y, segments[i].end.x, segments[i].end.y);
            assert_eq!(treap.remove(&segments[i], basic_operations), true);
            assert_eq!(treap.find(&segments[i], basic_operations), false);
            //treap.print_inorder();
        }
        assert_eq!(treap.root, None);
    }
}




