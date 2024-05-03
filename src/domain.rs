use std::io::Write;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn to_grid(&self, grid_size: f32) -> Point {
        return Point {
            x: (self.x / grid_size).round() * grid_size,
            y: (self.y / grid_size).round() * grid_size,
        };
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Segment {
    pub ini: Point,
    pub end: Point,
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
    pub time: f32,
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
            let x1 = values.next().unwrap().parse::<f32>().unwrap();
            let y1 = values.next().unwrap().parse::<f32>().unwrap();
            let x2 = values.next().unwrap().parse::<f32>().unwrap();
            let y2 = values.next().unwrap().parse::<f32>().unwrap();
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
pub fn distance(p1: &Point, p2: &Point) -> f32 {
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
    let x_min = f32::min(s.ini.x, s.end.x);
    let x_max = f32::max(s.ini.x, s.end.x);
    let y_min = f32::min(s.ini.y, s.end.y);
    let y_max = f32::max(s.ini.y, s.end.y);
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
