#![allow(dead_code)]
// /src/utils/lines_and_curves.rs
use super::*;

pub trait Intersection {
    fn find_significant_xs(&self) -> Vec<f64>;
    fn y(&self, next: &Self, x: f64) -> Vec<(f64, bool)>;
    fn is_inside<
        T: Intersection
    >(intersections: &Vec<T>, x: f64, y: f64) -> bool {
        let mut highest_less_y = None;
        let mut result = false;
        for i in 0..intersections.len() {
            for (y_iter, is_inside) in Intersection::y(
                &intersections[i],
                &intersections[(i+1) % intersections.len()],
                x,
            ) {
                if y_iter < y {
                    let (new_y, new_can_cut) = if let Some(high_y) = highest_less_y {
                        if high_y < y_iter ||
                            (high_y <= y_iter && !is_inside) {
                            (y_iter, is_inside)
                        } else {
                            (high_y, result)
                        }
                    } else {
                        (y_iter, is_inside)
                    };

                    highest_less_y = Some(new_y);
                    result = new_can_cut;
                }
            }
        }
        return result;
    }

    fn times_cross_line(&self, line: &LineSegment) -> usize;
    fn times_cross_line_vec(&self, lines: &Vec<LineSegment>) -> usize {
        let mut count = 0;
        for line in lines {
            count += self.times_cross_line(&line);
        }
        return count;
    }
    fn times_cross_border_rect(&self, rect : &Rectangle) -> usize {
        self.times_cross_line(
            &LineSegment::from_ray(
                Point::from(rect.start_point.x, rect.start_point.y),
                Point::from(rect.end_point.x, rect.start_point.y)
            )
        ) +
        self.times_cross_line(
            &LineSegment::from_ray(
                Point::from(rect.end_point.x, rect.start_point.y),
                Point::from(rect.end_point.x, rect.end_point.y)
            )
        ) +
        self.times_cross_line(
            &LineSegment::from_ray(
                Point::from(rect.end_point.x, rect.end_point.y),
                Point::from(rect.start_point.x, rect.end_point.y)
            )
        ) +
        self.times_cross_line(
            &LineSegment::from_ray(
                Point::from(rect.start_point.x, rect.end_point.y),
                Point::from(rect.start_point.x, rect.start_point.y)
            )
        )
    }
    fn intersects_line(&self, line : &LineSegment) -> bool {
        self.times_cross_line(line) > 0
    }
    fn intersects_rectangle(&self, rect : &Rectangle) -> bool;
    fn bounding_box(&self) -> Rectangle;
    fn closest_distance_to_point(&self, point: &Point) -> f64;

    // Used a bit_radius on a cnc router
    // make a new path with intersections farther away from not cut zone by radius
    fn add_radius(
        items: &Vec<Self>,
        radius: f64,
        cut_inside: bool,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized;

    fn remove_touching_shapes(
        shapes: &Vec<(Vec<Self>, bool)>,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized;
    // fn lines_below_point(shape: &Vec<Box<Self>>, point: Point) -> usize {
    //     let mut count = 0;

    //     for i in 0..shape.len() {
    //         let line = &shape[i];
    //         let next_line = &shape[(i+1) % shape.len()];
    //         for y in line.y(&next_line, point.x) {
    //             if y < point.y {
    //                 count += 1;
    //             }
    //         }
    //     }

    //     return count;
    // }

    fn find_barely_inner_point(lines: &Vec<Self>) -> Point where Self : Sized;

    fn force_counter_clockwise(lines: &Vec<Self>) -> Vec<Self> where Self : Sized;
}

#[derive(Debug, Clone)]
pub enum AllIntersections {
    Rectangle(Rectangle),
    SoftLineSegment(LineSegment),
    LineSegment(LineSegment),
    Circle(Circle),
}

impl AllIntersections {
    pub fn from_rects(arr: Vec<Rectangle>) -> Vec<Self> {
        arr.iter().map(|x| AllIntersections::Rectangle(x.clone())).collect()
    }
    pub fn from_soft_line_segments(arr: Vec<LineSegment>) -> Vec<Self> {
        arr.iter().map(|x| AllIntersections::SoftLineSegment(x.clone())).collect()
    }
    pub fn from_line_segment(arr: Vec<LineSegment>) -> Vec<Self> {
        arr.iter().map(|x| AllIntersections::LineSegment(x.clone())).collect()
    }
    pub fn from_circle(arr: Vec<Circle>) -> Vec<Self> {
        arr.iter().map(|x| AllIntersections::Circle(x.clone())).collect()
    }
    pub fn join_all(
        rectangles: &Vec<Rectangle>,
        soft_lines: &Vec<LineSegment>,
        lines     : &Vec<LineSegment>,
        circles   : &Vec<Circle>,
    ) -> Vec<Self> {
        vec![
            Self::from_rects(rectangles.clone()),
            Self::from_soft_line_segments(soft_lines.clone()),
            Self::from_line_segment(lines.clone()),
            Self::from_circle(circles.clone()),
        ].iter().flatten().map(|x| x.clone()).collect::<Vec<Self>>()
    }

    pub fn seperate_vec(arr: &Vec<Self>) ->
        (Vec<Rectangle>, Vec<LineSegment>, Vec<LineSegment>, Vec<Circle>) {
        let mut rec    = Vec::new();
        let mut line   = Vec::new();
        let mut soft   = Vec::new();
        let mut circle = Vec::new();

        for item in arr {
            match item {
                AllIntersections::Rectangle(r) => rec.push(r.clone()),
                AllIntersections::SoftLineSegment(s) => soft.push(s.clone()),
                AllIntersections::LineSegment(l) => line.push(l.clone()),
                AllIntersections::Circle(c) => circle.push(c.clone()),
            }
        }

        (rec, line, soft, circle)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let inside_square = b*b - 4.0*a*c;

    if inside_square < 0.0 {
        return None;
    }

    let sqrt = inside_square.sqrt();

    let plus = (-b + sqrt) / (2.0 * a);
    let minus = (-b + sqrt) / (2.0 * a);

    return Some((plus, minus));
}

fn multiply_matrix(l11: f64, l12: f64, l21: f64, l22: f64, a: f64, b:f64) -> (f64, f64) {
    (
        l11 * a + l12 * b,
        l21 * a + l22 * b,
    )
}
fn multiply_matrix_m(m: (f64, f64, f64, f64), x: f64, y: f64) -> (f64, f64) {
    multiply_matrix(m.0, m.1, m.2, m.3, x, y)
}

impl Circle {
    pub fn contains_x(&self, x: f64) -> bool {
        self.center.x - self.radius <= x && self.center.x + self.radius >= x
    }
    pub fn distance_to_center(&self, line: &LineSegment) -> f64 {
        line.distance_to_point(&self.center)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    start_point: Point,
    end_point: Point,
}

impl Rectangle {
    pub fn zero() -> Self {
        Self {
            start_point: Point::zero(),
            end_point: Point::zero(),
        }
    }
    pub fn from(start_point: Point, end_point: Point) -> Self {
        Self {
            start_point: start_point,
            end_point: end_point,
        }
    }

    pub fn from_rect_add_radius(rect: &Rectangle, radius: f64) -> Self {
        let mut r = Rectangle::from(
            Point::from(rect.min_x() - radius, rect.min_y() - radius),
            Point::from(rect.max_x() + radius, rect.max_y() + radius),
        );

        if r.start_point.x > r.end_point.x {
            r.start_point.x = (rect.start_point.x + rect.end_point.x) / 2.0;
            r.end_point.x = r.start_point.x;
        }

        if r.start_point.y > r.end_point.y {
            r.start_point.y = (rect.start_point.y + rect.end_point.y) / 2.0;
            r.end_point.y = r.start_point.y;
        }

        return r;
    }

    pub fn p1(&self) -> Point {
        self.start_point
    }
    pub fn p2(&self) -> Point {
        self.end_point
    }

    pub fn contains_x(&self, x: f64) -> bool {
        (self.start_point.x >= x && self.end_point.x <= x) ||
            (self.start_point.x <= x && self.end_point.x >= x)
    }

    pub fn contains_y(&self, y: f64) -> bool {
        (self.start_point.y >= y && self.end_point.y <= y) ||
            (self.start_point.y <= y && self.end_point.y >= y)
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.contains_x(point.x) && self.contains_y(point.y)
    }

    pub fn min_x(&self) -> f64 {
        f64::min(self.start_point.x, self.end_point.x)
    }

    pub fn max_x(&self) -> f64 {
        f64::max(self.start_point.x, self.end_point.x)
    }

    pub fn min_y(&self) -> f64 {
        f64::min(self.start_point.y, self.end_point.y)
    }

    pub fn max_y(&self) -> f64 {
        f64::max(self.start_point.y, self.end_point.y)
    }

    pub fn width(&self) -> f64 {
        (self.start_point.x - self.end_point.x).abs()
    }

    pub fn height(&self) -> f64 {
        (self.start_point.y - self.end_point.y).abs()
    }

    pub fn mid_y(&self) -> f64 {
        (self.start_point.y + self.end_point.y) / 2.0
    }

    pub fn mid_x(&self) -> f64 {
        (self.start_point.x + self.end_point.x) / 2.0
    }

    pub fn join(&self, rect: &Rectangle) -> Rectangle {
        Rectangle::from(
            Point::from(
                vec![
                    self.start_point.x, self.end_point.x,
                    rect.start_point.x, rect.end_point.x,
                ].iter().cloned().fold(0./0., f64::min),
                vec![
                    self.start_point.y, self.end_point.y,
                    rect.start_point.y, rect.end_point.y,
                ].iter().cloned().fold(0./0., f64::min),
            ),
            Point::from(
                vec![
                    self.start_point.x, self.end_point.x,
                    rect.start_point.x, rect.end_point.x,
                ].iter().cloned().fold(0./0., f64::max),
                vec![
                    self.start_point.y, self.end_point.y,
                    rect.start_point.y, rect.end_point.y,
                ].iter().cloned().fold(0./0., f64::max),
            ),
        )
    }

    pub fn to_points(&self) -> Vec<Point> {
        vec![
            self.start_point,
            Point::from(self.end_point.x, self.start_point.y),
            self.end_point,
            Point::from(self.start_point.x, self.end_point.y),
        ]
    }

    pub fn to_lines(&self) -> Vec<LineSegment> {
        vec![
            LineSegment::from(
                self.start_point,
                Point::from(self.end_point.x, self.start_point.y),
            ),
            LineSegment::from(
                Point::from(self.end_point.x, self.start_point.y),
                self.end_point,
            ),
            LineSegment::from(
                self.end_point,
                Point::from(self.start_point.x, self.end_point.y),
            ),
            LineSegment::from(
                Point::from(self.start_point.x, self.end_point.y),
                self.start_point,
            ),
        ]
    }
}

pub fn bounding_box<T: Intersection>(intersections: &Vec<T>) -> Option<Rectangle> {
    let start_rect : Rectangle;
    if let Some(r) = intersections.first() {
        start_rect = r.bounding_box();
    } else {
        return None;
    }

    Some(intersections.iter().fold(start_rect, |rect, line| -> Rectangle {
        rect.join(&line.bounding_box())
    }))
}

pub fn bounding_box_itr<T: Intersection, I>(mut intersections: I) -> Option<Rectangle> 
where I: Iterator<Item=T>
{
    let start_rect : Rectangle;
    if let Some(r) = intersections.next() {
        start_rect = r.bounding_box();
    } else {
        return None;
    }

    Some(intersections.fold(start_rect, |rect, line| -> Rectangle {
        rect.join(&line.bounding_box())
    }))
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Eq for Point {}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y < other.y {
            std::cmp::Ordering::Less
        }
        else if self.y > other.y {
            std::cmp::Ordering::Greater
        }
        else if self.x < other.x {
            std::cmp::Ordering::Less
        }
        else if self.x > other.x {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}
impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}
impl std::ops::Mul<f64> for Point {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {x: self.x * scalar, y: self.y * scalar}
    }
}
impl std::ops::Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, point: Point) -> Point {
        Point {x: point.x * self, y: point.y * self}
    }
}
impl std::ops::Div<f64> for Point {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {x: self.x / scalar, y: self.y / scalar}
    }
}

impl Point {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
    pub fn from(x: f64, y: f64) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
    pub fn distance_to(&self, p: &Point) -> f64 {
        let difx = self.x - p.x;
        let dify = self.y - p.y;
        return (difx*difx + dify*dify).sqrt();
    }

    pub fn to_hashable(&self) -> (
        ordered_float::OrderedFloat<f64>,
        ordered_float::OrderedFloat<f64>
    ) {
        (ordered_float::OrderedFloat(self.x), ordered_float::OrderedFloat(self.y))
    }

    pub fn counter_clockwise_angle_to(&self, center: &Point) -> f64 {
        let degrees = ((self.x - center.x) / self.distance_to(&center)).acos();
        if self.y < center.y {
            2.0 * std::f64::consts::PI - degrees
        } else {
            degrees
        }
    }

    pub fn right_angle(
        start: &Point,
        center: &Point,
        end: &Point,
    ) -> f64 {
        let end_angle = end.counter_clockwise_angle_to(&center);
        let start_angle = start.counter_clockwise_angle_to(&center);

        let angle = end_angle - start_angle;
        if angle < 0.0 {
            angle + 2.0*std::f64::consts::PI
        } else {
            angle
        }
    }

    pub fn area(points: &Vec<Self>) -> f64 {
        let mut positive = 0.0;
        let mut negative = 0.0;

        for i in 0..points.len() {
            let j = (i+1) % points.len();
            positive += points[i].x * points[j].y;
            negative += points[i].y * points[j].x;
        }

        return (positive - negative) / 2.0;
    }

    pub fn normalize(&self) -> Self {
        let distance = self.distance_to(&Point::zero());
        if distance == 0.0 {
            return Self::zero();
        } else {
            return Self::from(self.x / distance, self.y / distance);
        }
    }

    pub fn distance_down_an_angle(theta: f64, length_between_points: f64) -> f64 {
        let a = (theta.cos() - 1.0).powi(2);
        let b = theta.sin();
        let c = -length_between_points.powi(2);

        let sqrt = (b.powi(2) - 4.0 * a * c).sqrt();
        (-b + sqrt) / (2.0 * a)
    }
}

// MARK: Custom type for sorting line segments in method all_intersections

struct Range<T> where T: Iterator {
    inner: T
}
impl<T> Iterator for Range<T> where T: Iterator {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Debug, Clone, Copy)]
struct AllIntersectionsQueueItemBook {
    point: Point,
}
impl PartialEq for AllIntersectionsQueueItemBook {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
impl Eq for AllIntersectionsQueueItemBook {}
impl Ord for AllIntersectionsQueueItemBook {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.point.y < other.point.y {
            std::cmp::Ordering::Less
        } else if self.point.y > other.point.y {
            std::cmp::Ordering::Greater
        }
        else if self.point.x < other.point.x {
            std::cmp::Ordering::Less
        } else if self.point.x > other.point.x {
            std::cmp::Ordering::Greater
        }
        else {
            std::cmp::Ordering::Equal
        }
    }
}
impl PartialOrd for AllIntersectionsQueueItemBook {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum AllIntersectionsQueueItemPointType {
    Middle,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
struct AllIntersectionsQueueItem {
    point: Point,
    point_position: AllIntersectionsQueueItemPointType,
    line_segment_index: usize
}
impl PartialEq for AllIntersectionsQueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.point_position == other.point_position &&
            self.line_segment_index == other.line_segment_index
    }
}
impl Eq for AllIntersectionsQueueItem {}
impl Ord for AllIntersectionsQueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.point.y < other.point.y {
            std::cmp::Ordering::Less
        } else if self.point.y > other.point.y {
            std::cmp::Ordering::Greater
        }
        else if self.point.x < other.point.x {
            std::cmp::Ordering::Less
        } else if self.point.x > other.point.x {
            std::cmp::Ordering::Greater
        }
        else if self.point_position < other.point_position {
            std::cmp::Ordering::Less
        } else if self.point_position > other.point_position {
            std::cmp::Ordering::Greater
        }
        else {
            std::cmp::Ordering::Equal
        }
    }
}
impl PartialOrd for AllIntersectionsQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone, Copy)]
struct AllIntersectionsTreeNode<'a> {
    line: &'a LineSegment,
    index: usize,
    latest_x: f64,
    latest_y: f64,
}
impl<'a> PartialEq for AllIntersectionsTreeNode<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl<'a> Eq for AllIntersectionsTreeNode<'a> {}
impl<'a> Ord for AllIntersectionsTreeNode<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let y = f64::max(self.latest_y, other.latest_y);
        let latest_x = if self.latest_y == other.latest_y {
            if self.latest_x < other.latest_x {
                other.latest_x
            } else {
                self.latest_x
            }
        } else if self.latest_y < other.latest_y {
            other.latest_x
        } else {
            self.latest_x
        };

        let self_x = if let Some(x) = self.line.x_endless_line(y) {
            x
        } else if latest_x < self.line.min_x() {
            self.line.min_x()
        } else if latest_x > self.line.max_x() {
            self.line.max_x()
        } else {
            latest_x
        };

        let other_x = if let Some(x) = other.line.x_endless_line(y) {
            x
        } else if latest_x < other.line.min_x() {
            other.line.min_x()
        } else if latest_x > other.line.max_x() {
            other.line.max_x()
        } else {
            latest_x
        };

        if (self_x - other_x).abs() < 0.000001 {
            // continue on
        }
        else if self_x < other_x {
            return std::cmp::Ordering::Less;
        } else if self_x > other_x {
            return std::cmp::Ordering::Greater;
        }

        // With epsilon

        let epsilon = 0.001;
        let self_x = if let Some(x) = self.line.x_endless_line(y+epsilon) {
            x
        } else {
            self.line.max_x()
        };

        let other_x = if let Some(x) = other.line.x_endless_line(y+epsilon) {
            x
        } else {
            other.line.max_x()
        };

        let is_equal = (self_x - other_x).abs() < 0.0001;
        if !is_equal && self_x < other_x {
            std::cmp::Ordering::Less
        } else if !is_equal && self_x > other_x {
            std::cmp::Ordering::Greater
        } // ending with index just incase
        else if self.index < other.index {
            std::cmp::Ordering::Less
        } else if self.index > other.index {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
impl<'a> PartialOrd for AllIntersectionsTreeNode<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineSegment {
    p1: Point,
    p2: Point,
    pub includes_first_point: bool,
    pub includes_second_point: bool,
}

impl LineSegment {
    pub fn from(p1: Point, p2: Point) -> Self {
        Self {
            p1: p1,
            p2: p2,
            includes_first_point: true,
            includes_second_point: true,
        }
    }
    pub fn from_ray(p1: Point, p2: Point) -> Self {
        Self {
            p1: p1,
            p2: p2,
            includes_first_point: true,
            includes_second_point: false,
        }
    }
    pub fn from_include(p1: Point, p2: Point,
        includes_first_point: bool, includes_second_point: bool) -> Self {
        Self {
            p1: p1,
            p2: p2,
            includes_first_point: includes_first_point,
            includes_second_point: includes_second_point,
        }
    }

    pub fn from_points(points: &Vec<Point>) -> Vec<Self> {
        let mut lines = Vec::new();

        for i in 0..points.len() {
            lines.push(Self::from_ray(
                points[i],
                points[(i+1) % points.len()],
            ));
        }

        return lines;
    }

    pub fn from_line_to_line_counter_clockwise(lines: &Vec<Self>) -> Vec<Self> {
        let area = Point::area(
            &lines.iter().map(|x| x.p1).collect()
        );

        if area < 0.0 {
            Self::from_lines_flipped(lines)
        } else {
            lines.clone()
        }
    }

    pub fn from_lines_flipped(lines: &Vec<Self>) -> Vec<Self> {
        let mut points : Vec<Point> = lines.iter().map(|x| x.p1).collect();
        points.reverse();
        Self::from_points(&points)
    }

    pub fn from_linestring(linestring: &geo_types::geometry::LineString) -> Vec<Self> {
        let mut first_point = None;
        let mut previous_point = None;
        let mut new_lines = Vec::new();
        for point in linestring {
            match previous_point {
                None => {
                    first_point = Some(point);
                    previous_point = Some(point);
                },
                Some(pp) => {
                    new_lines.push(
                        LineSegment::from(
                            Point::from(
                                pp.x, pp.y,
                            ),
                            Point::from(
                                point.x, point.y,
                            ),
                        )
                    );
                    previous_point = Some(point);
                }
            }
        }

        if let (Some(first), Some(last)) = (first_point, previous_point) {
            new_lines.push(
                LineSegment::from(
                    Point::from(
                        last.x, last.y,
                    ),
                    Point::from(
                        first.x, first.y,
                    ),
                )
            );
        }

        return new_lines;
    }

    pub fn from_multipolygon(
        multi_polygon: &geo_types::MultiPolygon,
        can_cut_in_outer: bool,
    ) -> Vec<(Vec<Self>, bool)> {
        multi_polygon
            .0
            .iter()
            .map(
                |polygon| {
                    let (outer, inner) = polygon.clone().into_inner();
                    let mut new_lines = vec![
                        (LineSegment::from_linestring(&outer), can_cut_in_outer)
                    ];

                    let mut inner_lines : Vec<(Vec<LineSegment>, bool)> = inner
                        .iter()
                        .map(|linestring|
                            (LineSegment::from_linestring(&linestring), !can_cut_in_outer)
                        )
                        .collect();
                    new_lines.append(&mut inner_lines);

                    return new_lines;
                }
            )
            .flatten()
            .collect::<Vec<(Vec<Self>, bool)>>()
    }

    pub fn point1(&self) -> Point {
        self.p1
    }
    pub fn point2(&self) -> Point {
        self.p2
    }

    pub fn mid_point(&self) -> Point {
        (self.p1 + self.p2) / 2.0
    }

    pub fn points(lines: &Vec<Self>, repeat_first_point: bool) -> Vec<Point> {
        let mut points : Vec<Point> =
            lines.iter().map(|x| x.p1).collect();
        if points.len() > 0 && repeat_first_point {
            points.push(lines[0].p1);
        }

        return points;
    }

    pub fn points_geo_type(lines: &Vec<Self>, repeat_first_point: bool) -> Vec<geo_types::Coordinate> {
        use geo_types::{Coordinate};
        return Self::points(lines, repeat_first_point)
            .iter()
            .map(|point|
                Coordinate {
                    x: point.x,
                    y: point.y,
                }
            )
            .collect();
    }

    pub fn find_barely_inner_point(lines: &Vec<Self>) -> Point {
        if lines.len() < 3 {
            panic!("Lines must be >= 3. Found: {}", lines.len());
        }

        let p1 = lines[0].p1;
        let p2 = lines[1].p1;
        let p3 = lines[2].p1;

        let p2p1 = (p1 - p2).normalize();
        let p2p3 = (p3 - p2).normalize();

        let d = (p2p1 + p2p3).normalize();

        let epsilon = 0.0001;
        let p2d = p2 + epsilon * d;

        // if in polygon
        if Self::is_inside(lines, p2d.x, p2d.y) {
            p2d
        } else {
            p2 - epsilon * d
        }
    }

    pub fn perpendicular_derivitive_point(&self) -> Point {
        let m = self.p2 - self.p1;
        let p = Point::from(-m.y, m.x);
        let distance = p.distance_to(&Point::zero());
        if distance == 0.0 {
            p
        } else {
            p / distance
        }
    }

    pub fn point_of_interception_endless_lines(
        &self, line: &LineSegment
    ) -> Option<Point> {

        let Some((b1, m1)) = self.y_intercept_and_slope() else {
            let Some((b2, m2)) = line.y_intercept_and_slope() else {
                if self.p1.x == line.p1.x {
                    if line.contains_y(self.p1.y) {
                        return Some(self.p1);
                    }
                    if line.contains_y(self.p2.y) {
                        return Some(self.p2);
                    }
                }
                return None;
            };
            let x = self.p1.x;
            let y = x * m2 + b2;
            return Some(Point::from(x, y));
        };

        let Some((b2, m2)) = line.y_intercept_and_slope() else {
            let x = line.p1.x;
            let y = x * m1 + b1;
            return Some(Point::from(x, y));
        };

        if line.contains_point(self.p2) {
            return Some(self.p2);
        } else if line.contains_point(self.p1) {
            return Some(self.p1);
        } else if self.contains_point(line.p1) {
            return Some(line.p1);
        } else if self.contains_point(line.p2) {
            return Some(line.p2);
        }

        if m1 == m2 {
            return None;
        }

        let x = (b2 - b1) / (m1 - m2);
        let y = m1 * x + b1;

        return Some(Point::from(x, y));
    }

    pub fn point_of_interception(
        &self, line: &LineSegment
    ) -> Option<Point> {
        let Some(point) = self.point_of_interception_endless_lines(
            line
        ) else {
            return None;
        };

        if self.contains_x_and_y_with_error(point) &&
            line.contains_x_and_y_with_error(point)
        {
            Some(point)
        } else {
            None
        }
    }

    pub fn does_intersect(
        poly1: &Vec<Self>,
        poly2: &Vec<Self>,
    ) -> bool {
        for line1 in poly1 {
            for line2 in poly2 {
                if line1.intersects_line(&line2) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn union(
        poly1: &Vec<Self>,
        poly2: &Vec<Self>,
        can_cut_in_outer: bool,
    ) -> Vec<(Vec<Self>, bool)> {
        use geo_clipper::Clipper;
        use geo_types::{LineString, Coordinate, Polygon};

        let poly1 = Polygon::new(
            LineString(Self::points_geo_type(poly1, true)),
            vec![],
        );

        let poly2 = Polygon::new(
            LineString(Self::points_geo_type(poly2, true)),
            vec![],
        );

        Self::from_multipolygon(
            &poly1.union(
                &poly2,
                1024.0,
            ),
            can_cut_in_outer,
        )
    }

    pub fn differance(
        poly1: &Vec<Self>,
        poly2: &Vec<Self>,
        can_cut_in_outer: bool,
    ) -> Vec<(Vec<Self>, bool)> {
        use geo_clipper::Clipper;
        use geo_types::{LineString, Coordinate, Polygon};

        let poly1 = Polygon::new(
            LineString(Self::points_geo_type(poly1, true)),
            vec![],
        );

        let poly2 = Polygon::new(
            LineString(Self::points_geo_type(poly2, true)),
            vec![],
        );

        Self::from_multipolygon(
            &poly1.difference(
                &poly2,
                1024.0,
            ),
            can_cut_in_outer,
        )
    }

    pub fn create_path(points: &Vec<Point>, enclose: bool) -> Vec<Self> {
        let mut r = Vec::new();
        let mut last_point: Option<Point> = None;
        for p in points {
            if let Some(lp) = last_point {
                r.push(LineSegment::from_ray(lp, *p));
            }
            last_point = Some(*p);
        }

        match (last_point, points.get(0), enclose) {
            (Some(p1), Some(p2), true) => {
                r.push(LineSegment::from_ray(p1, *p2));
            },
            (_, _, _) => {

            }
        }

        return r;
    }

    pub fn slope(&self) -> Option<f64> {
        let dif_x = self.p1.x - self.p2.x;
        if dif_x == 0.0 {
            None
        } else {
            Some((self.p1.y - self.p2.y) / dif_x)
        }
    }

    pub fn y_intercept(&self) -> Option<f64> {
        if let Some(m) = self.slope() {
            Some(self.p1.y - m * self.p1.x)
        } else {
            None
        }
    }

    pub fn y_intercept_and_slope(&self) -> Option<(f64, f64)> {
        if let Some(m) = self.slope() {
            Some((self.p1.y - m * self.p1.x, m))
        } else {
            None
        }
    }

    pub fn y(&self, x: f64) -> Option<f64> {
        if !self.contains_x(x) {
            None
        } else if let Some((b, m)) = self.y_intercept_and_slope() {
            Some(x * m + b)
        } else {
            None
        }
    }

    pub fn x_endless_line(&self, y: f64) -> Option<f64> {
        let Some((b, m)) = self.y_intercept_and_slope() else {
            return Some(self.p1.x);
        };
        if m == 0.0 {
            None
        } else {
            Some((y-b) / m)
        }
    }

    pub fn min_x(&self) -> f64 {
        f64::min(self.p1.x, self.p2.x)
    }
    pub fn max_x(&self) -> f64 {
        f64::max(self.p1.x, self.p2.x)
    }
    pub fn min_y(&self) -> f64 {
        f64::min(self.p1.y, self.p2.y)
    }
    pub fn max_y(&self) -> f64 {
        f64::max(self.p1.y, self.p2.y)
    }

    pub fn max_y_then_x_point(&self) -> Point {
        if self.p1.y > self.p2.y {
            self.p1
        } else if self.p1.y < self.p2.y {
            self.p2
        } else if self.p1.x > self.p2.x {
            self.p1
        } else {
            self.p2
        }
    }

    pub fn min_y_then_x_point(&self) -> Point {
        if self.p1.y < self.p2.y {
            self.p1
        } else if self.p1.y > self.p2.y {
            self.p2
        } else if self.p1.x < self.p2.x {
            self.p1
        } else {
            self.p2
        }
    }

    pub fn min_y_point(&self) -> Point {
        if self.p1.y < self.p2.y {
            self.p1
        } else {
            self.p2
        }
    }

    pub fn min_x_point(&self) -> Point {
        if self.p1.x < self.p2.x {
            self.p1
        } else {
            self.p2
        }
    }

    pub fn contains_x(&self, x: f64) -> bool {
        let first_point_gt = x > self.p1.x || (self.includes_first_point && x >= self.p1.x);
        let first_point_lt = x < self.p1.x || (self.includes_first_point && x <= self.p1.x);
        let second_point_gt = x > self.p2.x || (self.includes_second_point && x >= self.p2.x);
        let second_point_lt = x < self.p2.x || (self.includes_second_point && x <= self.p2.x);

        return (first_point_gt && second_point_lt) || (first_point_lt && second_point_gt) ||
            (self.p1.x == x && self.p2.x == x && self.p1.y != self.p2.y);
    }

    pub fn contains_x_error(&self, x: f64, error: f64) -> bool {
        let first_point_gt = x + error > self.p1.x;
        let first_point_lt = x < self.p1.x + error;
        let second_point_gt = x + error > self.p2.x;
        let second_point_lt = x < self.p2.x + error;

        return (first_point_gt && second_point_lt) || (first_point_lt && second_point_gt) ||
            (self.p1.x == x && self.p2.x == x && self.p1.y != self.p2.y);
    }

    pub fn contains_y(&self, y: f64) -> bool {
        let first_point_gt = y > self.p1.y || (self.includes_first_point && y >= self.p1.y);
        let first_point_lt = y < self.p1.y || (self.includes_first_point && y <= self.p1.y);
        let second_point_gt = y > self.p2.y || (self.includes_second_point && y >= self.p2.y);
        let second_point_lt = y < self.p2.y || (self.includes_second_point && y <= self.p2.y);

        return (first_point_gt && second_point_lt) || (first_point_lt && second_point_gt) ||
            (self.p1.y == y && self.p2.y == y && self.p1.x != self.p2.x);
    }

    pub fn contains_y_error(&self, y: f64, error: f64) -> bool {
        let first_point_gt = y + error > self.p1.y;
        let first_point_lt = y < self.p1.y + error;
        let second_point_gt = y + error > self.p2.y;
        let second_point_lt = y < self.p2.y + error;

        return (first_point_gt && second_point_lt) || (first_point_lt && second_point_gt) ||
            (self.p1.y == y && self.p2.y == y && self.p1.x != self.p2.x);
    }

    pub fn length(&self) -> f64 {
        (
            (self.p2.x - self.p1.x).powi(2) +
            (self.p2.y - self.p1.y).powi(2)
        ).sqrt()
    }

    pub fn contains_x_and_y(&self, point: Point) -> bool {
        self.contains_x(point.x) && self.contains_y(point.y)
    }

    pub fn contains_x_and_y_with_error(&self, point: Point) -> bool {
        let epsilon = 0.000000001;
        self.contains_x_error(point.x, epsilon) &&
            self.contains_y_error(point.y, epsilon)
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.contains_x(point.x) && self.contains_y(point.y) &&
            if let Some(y) = self.y(point.x) {
                (y-point.y).abs() < 0.0001
            } else {
                true
            }
    }

    pub fn contains_point_endless_line(&self, point: Point) -> bool {
        if let Some((b, m)) = self.y_intercept_and_slope() {
            m * point.x + b == point.y
        } else {
            point.x == self.p1.x
        }
    }

    pub fn closest_point_on_line_to_point(&self, point: &Point) -> Point {
        let p = self.perpendicular_derivitive_point();
        let line = Self::from(*point, *point + p);

        let (min_x_point, max_x_point) = if self.p1.x < self.p2.x {
            (self.p1, self.p2)
        } else if self.p1.x > self.p2.x {
            (self.p2, self.p1)
        } else if self.p1.y < self.p2.y {
            (self.p1, self.p2)
        } else {
            (self.p2, self.p1)
        };


        if let Some(intersection) =
            self.point_of_interception_endless_lines(&line) {
            if intersection.x > self.p1.x &&
                intersection.x > self.p2.x
            {
                max_x_point
            } else if intersection.x < self.p1.x &&
                intersection.x < self.p2.x {
                min_x_point
            } else if intersection.y > self.p1.y &&
                intersection.y > self.p2.y {
                max_x_point
            }  else if intersection.y < self.p1.y &&
                intersection.y < self.p2.y {
                min_x_point
            } else {
                intersection
            }
        } else {
            // Should never happen
            // panic!("Should never happen E04098");
            self.mid_point()
        }
    }

    pub fn distance_to_point(&self, point: &Point) -> f64 {
        // New way. Draw a perpendicular line to self through point.
        //      Find intersecting point on both lines. Find smallest distance
        //      from self.p1, self.p2, intersecting point to point given.

        // TODO: This function really needs to be speed up a bit.
        // Maybe look up the best way how instead of trying to figure
        //      out on your own.

        // theta = atan(
        //      [cos a2 (d(p2) / d(p1)) - cos a1 ] /
        //      [sin a2 (d(p2) / d(p1)) - sin a1 ]
        // )

        let degrees = if self.p1 == Point::zero() && self.p2 == Point::zero() {
            0.0
        } else if self.p1 == Point::zero() {
            let a = (self.p2.x / self.p2.distance_to(&Point::zero())).acos();
            std::f64::consts::PI/2.0 - a
        } else if self.p2 == Point::zero() {
            let a = (self.p1.x / self.p1.distance_to(&Point::zero())).acos();
            std::f64::consts::PI/2.0 - a
        } else {
            let a1 = (self.p1.x / self.p1.distance_to(&Point::zero())).acos();
            let a2 = (self.p2.x / self.p2.distance_to(&Point::zero())).acos();

            let p1d = self.p1.distance_to(&Point::zero());
            let p2d = self.p2.distance_to(&Point::zero());

            (
                (a2.cos() * (p2d / p1d) - a1.cos()) /
                (a2.sin() * (p2d / p1d) - a1.sin())
            ).atan()
        };


        let l11 =  degrees.cos();
        let l12 = -degrees.sin();
        let l21 =  degrees.sin();
        let l22 =  degrees.cos();

        let m = (l11, l12, l21, l22);
        let p1 = multiply_matrix_m(m, self.p1.x, self.p1.y);
        let p2 = multiply_matrix_m(m, self.p2.x, self.p2.y);
        let c  = multiply_matrix_m(m, point.x, point.y);

        if p1.1 > c.1 && p2.1 > c.1 {
            let c = Point::from(c.0, c.1);
            return if p1.1 > p2.1 {
                Point::from(p2.0, p2.1).distance_to(&c)
            } else {
                Point::from(p1.0, p1.1).distance_to(&c)
            };
        }

        if p1.1 < c.1 && p2.1 < c.1 {
            let c = Point::from(c.0, c.1);
            return if p1.1 < p2.1 {
                Point::from(p2.0, p2.1).distance_to(&c)
            } else {
                Point::from(p1.0, p1.1).distance_to(&c)
            };
        }

        (p1.0 - c.0).abs()
    }

    fn find_a_perpendicular_dx_dy(
        &self,
    ) -> (f64, f64) {
        if let Some(ijm) = self.slope() {
            let reverse_m = -1.0 / ijm;
            let total = (reverse_m * reverse_m + 1.0).sqrt();
            if ijm == 0.0 { (0.0, 1.0) } else { (1.0 / total, reverse_m / total) }
        } else {
            (1.0, 0.0)
        }
    }

    fn find_perpendicular_dx_dy(
        &self,
        default_to_positive: &mut bool,
        can_cut: &mut Box<impl FnMut(f64, f64) -> bool>,
    ) -> (f64, f64) {
        let mid_point = (self.p1 + self.p2) / 2.0;

        let (dx, dy) = if let Some(ijm) = self.slope() {
            let reverse_m = -1.0 / ijm;
            let total = (reverse_m * reverse_m + 1.0).sqrt();
            if ijm == 0.0 { (0.0, 1.0) } else { (1.0 / total, reverse_m / total) }
        } else {
            (1.0, 0.0)
        };

        let epsilon = 0.0000001;
        if can_cut(
            mid_point.x + dx * epsilon,
            mid_point.y + dy * epsilon,
        ) {
            *default_to_positive = true;
            (dx, dy)
        } else if can_cut(
            mid_point.x - dx * epsilon,
            mid_point.y - dy * epsilon
        ) {
            *default_to_positive = false;
            (-dx, -dy)
        } else if *default_to_positive {
            (0.0, 0.0)
        } else {
            (0.0, 0.0)
        }
    }

    pub fn is_real_border_points_based(lines: &Vec<Self>, borders: &Vec<Vec<Self>>) -> bool {
        for i in 0..lines.len() {
            let center = lines[(i+1)%lines.len()].p1;
            let mid = (lines[i].p1 + center + lines[(i+2)%lines.len()].p1) / 3.0;
            let dp = (mid - center) / (mid.distance_to(&center) * 1000.0);

            let p1 : Point = center + dp;
            let p2 : Point = center - dp;

            let mut seen1 = 0;
            let mut seen2 = 0;
            for poly in borders {
                seen1 += if !LineSegment::is_inside(poly, p1.x, p1.y) {1} else {1};
                seen2 += if !LineSegment::is_inside(poly, p2.x, p2.y) {1} else {1};
            }

            // Keep polygons with real borders
            if (seen1 > 0) == (seen2 > 0) {
                return false;
            }
        }
        return true;
    }

    pub fn is_real_border_line_based(lines: &Vec<Self>, borders: &Vec<Vec<Self>>) -> bool {
        for i in 0..lines.len() {
            let center = (lines[i].p1 + lines[i].p2) / 2.0;
            // let mid = (lines[i].p1 + lines[(i+1)%lines.len()].p1 + lines[(i+2)%lines.len()].p1) / 3.0;
            // let dp = (mid - center) / (mid.distance_to(&center) * 1000.0);

            let epsilon = 0.0001;
            let (dx, dy) = lines[i].find_a_perpendicular_dx_dy();
            let dp = Point::from(dx * epsilon, dy * epsilon);

            let p1 : Point = center + dp;
            let p2 : Point = center - dp;

            let mut seen1 = 0;
            let mut seen2 = 0;
            for poly in borders {
                seen1 += if !LineSegment::is_inside(poly, p1.x, p1.y) {1} else {0};
                seen2 += if !LineSegment::is_inside(poly, p2.x, p2.y) {1} else {0};
            }

            // Keep polygons with real borders
            if (seen1 > 0) == (seen2 > 0) &&
                seen1 > 0 && seen2 > 0 {
                return false;
            }
        }
        return true;
    }

    pub fn line_segment_overlay_to_polygons(
        overlays: &Vec<Vec<Self>>,
    ) -> Vec<Vec<Self>> {
        let mut polygons = Vec::new();

        return polygons;
    }

    pub fn find_all_polygons(
        points_graph: &Vec<(Point, Vec<usize>)>,
        valid_points: &Vec<usize>
    ) -> Vec<Vec<usize>> {

        let mut seen = Vec::new();
        for _ in 0..points_graph.len() {
            let mut new_seen = Vec::new();
            for _ in 0..points_graph.len() {
                new_seen.push(2);
            }
            seen.push(new_seen);
        }

        let mut sorted_points = Vec::new();
        {
            let valid_points_set : std::collections::HashSet<usize> =
                valid_points.iter().map(|x| *x).collect();
            for i in valid_points {
                sorted_points.push((points_graph[*i].0, *i));
                for j in &points_graph[*i].1 {
                    if valid_points_set.contains(&j) {
                        seen[*i][*j] = 0;
                    }
                }
            }
        }

        sorted_points.sort_by(|lhs, rhs| {
            if lhs.0.y < rhs.0.y {
                std::cmp::Ordering::Less
            } else if lhs.0.y > rhs.0.y {
                std::cmp::Ordering::Greater
            }
            else if lhs.0.x < rhs.0.x {
                std::cmp::Ordering::Less
            } else if lhs.0.x > rhs.0.y {
                std::cmp::Ordering::Greater
            }
            else {
                std::cmp::Ordering::Equal
            }
        });
        let mut sorted_points_first_non_used_index = 0;

        let mut polygons = Vec::new();

        'mainloop: loop {
            let mut next_point = 0; // seen[][_]
            'outerwhile: while sorted_points_first_non_used_index < sorted_points.len() {
                for i in
                    &points_graph[sorted_points[sorted_points_first_non_used_index].1].1 {
                    if seen[sorted_points[sorted_points_first_non_used_index].1][*i] < 2 {
                        next_point = *i;
                        break 'outerwhile;
                    }
                }
                sorted_points_first_non_used_index += 1;
            }
            if sorted_points_first_non_used_index >= sorted_points.len() {
                break;
            }

            let mut new_polygon_points = Vec::new();
            new_polygon_points.push(sorted_points[sorted_points_first_non_used_index].1);

            for vertex_i in
                &points_graph[new_polygon_points[0]].1 {
                if seen[new_polygon_points[0]][*vertex_i] < 2 &&
                    points_graph[new_polygon_points[0]].0
                        .counter_clockwise_angle_to(&points_graph[*vertex_i].0) <
                    points_graph[new_polygon_points[0]].0
                        .counter_clockwise_angle_to(&points_graph[next_point].0)
                {
                    next_point = *vertex_i;
                }
            }
            new_polygon_points.push(next_point);
            // seen[new_polygon_points[0]][next_point] += 1;
            // seen[next_point][new_polygon_points[0]] += 1;

            let mut new_polygon_points_to_index = std::collections::HashMap::new();
            new_polygon_points_to_index.insert(new_polygon_points[0], 0);
            new_polygon_points_to_index.insert(new_polygon_points[1], 0);

            while new_polygon_points.len() < 3 ||
                new_polygon_points[0] != new_polygon_points[new_polygon_points.len()-1] {

                let prev_index = new_polygon_points[new_polygon_points.len()-2];
                let prev_point = points_graph[prev_index].0;
                let current_point_index = new_polygon_points[new_polygon_points.len()-1];
                let current_point = points_graph[current_point_index].0;

                let mut longest_angle = None;
                for i in 0..points_graph[current_point_index].1.len() {
                    let next_index = points_graph[current_point_index].1[i];
                    if seen[current_point_index][next_index] >= 2 { continue; }
                    let next_point = points_graph[next_index].0;
                    let angle = Point::right_angle(&prev_point, &current_point, &next_point);
                    longest_angle = if let Some((la, li)) = longest_angle {

                        // first polygon we want the least angle
                        // every other polygon we want the greatest
                        // first polygon is all the way around
                        // the rest has no polygons inside of it
                        if (polygons.len() == 0 && angle < la && next_index != prev_index) ||
                            (polygons.len() > 0 && angle > la)
                        {
                            Some((angle, next_index))
                        } else {
                            Some((la, li))
                        }
                    } else if next_index != prev_index {
                        Some((angle, next_index))
                    } else {
                        None
                    };
                }

                let longest_angle_index = if let Some(l) = longest_angle {
                    l.1
                } else {
                    // panic!("Should have had another option.");
                    eprintln!("Should have had another option. Trying to remove line. E0767");
                    seen[prev_index][current_point_index] = 2;
                    seen[current_point_index][prev_index] = 2;
                    continue 'mainloop;
                    // prev_index
                };

                new_polygon_points.push(longest_angle_index);
                if new_polygon_points_to_index.contains_key(&longest_angle_index) {
                    break;
                }
                new_polygon_points_to_index.insert(longest_angle_index, new_polygon_points.len()-1);

                // seen[new_polygon_points[new_polygon_points.len()-2]][longest_angle_index] += 1;
                // seen[longest_angle_index][new_polygon_points[new_polygon_points.len()-2]] += 1;
            }

            let mut new_polygon_points_only_cycle = Vec::new();
            for i in new_polygon_points_to_index[
                &new_polygon_points[new_polygon_points.len()-1]
            ]..new_polygon_points.len() {
                let point = new_polygon_points[i];
                new_polygon_points_only_cycle.push(point);
                if i+1 < new_polygon_points.len() {
                    let next_point = new_polygon_points[i+1];
                    seen[point][next_point] += 1;
                    seen[next_point][point] += 1;
                }
            }
            polygons.push(new_polygon_points_only_cycle);
        }

        let mut seen_polygons = std::collections::HashMap::<usize, Vec<usize>>::new();

        let mut filtered_poly = Vec::new();
        for i in 0..polygons.len() {
            let mut hash = (1<<31) - 1;
            for point_index in &polygons[i] {
                hash ^= *point_index;
            }

            let mut same = false;
            if let Some(indexes) = seen_polygons.get(&hash) {
                for j in indexes {
                    let j = *j;
                    if polygons[i] == polygons[j] {
                        same = true;
                    }
                }
            }

            if !same {
                filtered_poly.push(polygons[i].clone());
                if let Some(indexes) = seen_polygons.get_mut(&hash) {
                    indexes.push(i);
                } else {
                    seen_polygons.insert(hash, vec![i]);
                }
            }
        }
        return filtered_poly;
    }

    pub fn line_segments_to_polygons(
        lines: &Vec<Self>,
    ) -> Vec<Vec<(Self, usize)>> {
        let mut polygons = Vec::new();

        let graph_points_to_points_with_line_indexes =
            LineSegment::all_intersections_combine_points(&lines, 1000.0);
        let graph_points_to_points : Vec<(Point, Vec<usize>)> =
            graph_points_to_points_with_line_indexes.iter().map(
                |point_info| (
                    point_info.0,
                    point_info.1.iter().map(|connection| connection.0).collect()
                )
            ).collect();
        let point_to_point_to_line_index : Vec<Vec<usize>> =
            graph_points_to_points_with_line_indexes.iter().map(
                |(_, connections)| {
                    let mut new_connections = Vec::new();
                    for _ in 0..graph_points_to_points_with_line_indexes.len() {
                        new_connections.push(0);
                    }
                    for connection in connections {
                        new_connections[connection.0] = connection.1;
                    }
                    return new_connections;
                }
            ).collect();
        let strict_map : Vec<Vec<usize>> = graph_points_to_points.iter().map(
            |x| x.1.clone()
        ).collect();
        let scc = algorithms::get_scc_graph_vec_indexes(
            &strict_map
        );

        // Combine each component together
        for component in scc {
            let new_polygons = LineSegment::find_all_polygons(&graph_points_to_points, &component);

            for new_poly in new_polygons {
                let mut lines = Vec::new();
                for i in 1..new_poly.len() {
                    lines.push((
                        LineSegment::from_ray(
                            graph_points_to_points[new_poly[i-1]].0,
                            graph_points_to_points[new_poly[i]].0,
                        ),
                        point_to_point_to_line_index[new_poly[i-1]][new_poly[i]]
                    ));
                }
                polygons.push(lines);
            }
        }

        return polygons;
    }

    pub fn get_outlines_and_inlines_of_touching_polygons(
        shapes: &Vec<Vec<Self>>
    ) -> Vec<Vec<Self>> {
        let mut lines = Vec::new();
        let mut line_index_to_poly = Vec::new();
        for i in 0..shapes.len() {
            for line in &shapes[i] {
                lines.push(line.clone());
                line_index_to_poly.push(i);
            }
        }

        let polygons : Vec<Vec<Self>> = Self::line_segments_to_polygons(&lines)
            .iter()
            .filter(|polygon| {
                let mut borders = Vec::new();
                let mut borders_used = Vec::new();
                for _ in shapes {
                    borders_used.push(false);
                }

                for line in *polygon {
                    let polygon_index = line_index_to_poly[line.1];
                    if borders_used[polygon_index] { continue }
                    borders_used[polygon_index] = true;
                    borders.push(shapes[polygon_index].clone());
                }

                let polygon : Vec<Self> = polygon.iter().map(
                    |x| x.0.clone()
                ).collect();
                let area = Point::area(&
                    polygon
                        .iter()
                        .map(|x| x.p1)
                        .collect()
                );

                let mut polygon_straight_lines = Vec::new();
                polygon_straight_lines.push(polygon[0].clone());
                for poly in polygon {
                    let last_poly_index = polygon_straight_lines.len()-1;
                    let last_poly = &polygon_straight_lines[last_poly_index];
                    if poly.slope() == last_poly.slope() {
                        polygon_straight_lines[last_poly_index].p2 = poly.p2;
                    } else {
                        polygon_straight_lines.push(poly);
                    }
                }

                return Self::is_real_border_line_based(&polygon_straight_lines, &borders);
                // && area.abs() > 0.005;
            })
            .map(|polygon|
                polygon.iter().map(
                    |line| line.0.clone()
                ).collect()
            ).collect();

        return polygons;
    }

    pub fn all_intersections_slow(lines: &Vec<Self>) -> Vec<(Point, usize, usize)> {
        // Make all lines include both points
        let lines : Vec<Self> = lines.iter().map(|line| {
            LineSegment::from_include(line.p1, line.p2, true, true)
        }).collect();


        let mut intersections = Vec::new();

        for i in 0..lines.len() {
            for j in (i+1)..lines.len() {

                let ip1 = lines[j].contains_point(lines[i].p1);
                let ip2 = lines[j].contains_point(lines[i].p2);

                let jp1 = lines[i].contains_point(lines[j].p1);
                let jp2 = lines[i].contains_point(lines[j].p2);

                if jp1 || jp2 || ip1 || ip2 {
                    if ip1 {
                        intersections.push((lines[i].p1, i, j));
                    }
                    if ip2 {
                        intersections.push((lines[i].p2, i, j));
                    }
                    if jp1 && lines[j].p1 != lines[i].p1 && lines[j].p1 != lines[i].p2 {
                        intersections.push((lines[j].p1, i, j));
                    }
                    if jp2 && lines[j].p2 != lines[i].p1 && lines[j].p2 != lines[i].p2 {
                        intersections.push((lines[j].p2, i, j));
                    }

                    continue;
                }

                let Some(p) = lines[i].point_of_interception(&lines[j]) else {
                    continue;
                };

                intersections.push((p, i, j));
            }
        }

        return intersections;
    }

    pub fn all_intersections_book(lines: &Vec<Self>) -> Vec<(Point, usize, usize)> {

        use core::cmp::Reverse;

        // Make all lines include both points
        let lines : Vec<Self> = lines.iter().map(|line| {
            LineSegment::from_include(line.p1, line.p2, true, true)
        }).collect();

        // Initialize Queue
        let mut q = std::collections::binary_heap::BinaryHeap::new();
        let mut q_seen = std::collections::HashSet::new();
        let mut first_point_to_line = std::collections::HashMap::new();
        for i in 0..lines.len() {
            let p1_is_first = lines[i].p1.y < lines[i].p2.y ||
                (lines[i].p1.y != lines[i].p2.y && lines[i].p1.x < lines[i].p2.x);

            if p1_is_first {
                first_point_to_line.insert(lines[i].p1.to_hashable(), i);
            } else {
                first_point_to_line.insert(lines[i].p2.to_hashable(), i);
            }

            if q_seen.insert(lines[i].p1.to_hashable()) {
                q.push(Reverse(AllIntersectionsQueueItemBook {
                    point: lines[i].p1,
                }));
            }
            if q_seen.insert(lines[i].p2.to_hashable()) {
                q.push(Reverse(AllIntersectionsQueueItemBook {
                    point: lines[i].p2,
                }));
            }
        }

        // Initialize Tree
        let mut status_structure : std::collections::BTreeSet::<AllIntersectionsTreeNode>
            = std::collections::BTreeSet::<AllIntersectionsTreeNode>::new();

        // Keep track of all intersections
        let mut intersection_points = Vec::new();

        // Status structure is always sorted up till this point
        let mut last_point = q.peek().unwrap().0.point;

        while let Some(Reverse(top)) = q.pop() {

            let check_node = AllIntersectionsTreeNode{
                line: &LineSegment::from(
                    top.point,
                    Point::from(top.point.x, top.point.y + 1.0),
                ),
                index: lines.len(),
                latest_x: top.point.x,
                latest_y: top.point.y,
            };


            // Calculate Upper, lower and interior points

            let mut upper_end_point_is_top = Vec::new();
            if let Some(line_index) = first_point_to_line.get(&(top.point.to_hashable())) {
                upper_end_point_is_top.push(*line_index);
            }
            let mut lower_end_point_is_top = Vec::new();
            let mut interior_end_point_is_top = Vec::new();

            use std::ops::Bound::*;
            for (min, max, is_reverse) in [
                (Excluded(&check_node), Unbounded, false),
                (Unbounded, Excluded(&check_node), true)
            ] {
                let mut range : Box<dyn Iterator<Item=&AllIntersectionsTreeNode>> =
                    Box::from(status_structure.range((min, max)));
                if is_reverse {
                    range = Box::from(status_structure.range((min, max)).rev());
                }

                for next in range {
                    if next.line.contains_point(top.point) {
                        if next.line.min_y_then_x_point() == top.point {
                            upper_end_point_is_top.push(next.index);
                        }
                        else if next.line.max_y_then_x_point() == top.point {
                            lower_end_point_is_top.push(next.index);
                        }
                        else {
                            interior_end_point_is_top.push(next.index);
                        }
                    } else {
                        break;
                    }
                }
            }

            // report intersections

            let mut all_intersections = upper_end_point_is_top.clone();
            all_intersections.extend(lower_end_point_is_top.clone());
            all_intersections.extend(interior_end_point_is_top.clone());
            for i in 0..all_intersections.len() {
                for j in (i+1)..all_intersections.len() {
                    intersection_points.push((
                        top.point,
                        all_intersections[i],
                        all_intersections[j],
                    ));
                }
            }

            // remove lines
            let mut delete_from = lower_end_point_is_top.clone();
            delete_from.extend(interior_end_point_is_top.clone());
            for index in delete_from {
                let delete_node = AllIntersectionsTreeNode{
                    line: &lines[index],
                    index: index,
                    latest_x: last_point.x,
                    latest_y: last_point.y,
                };
                assert!(status_structure.remove(&delete_node));
            }

            // insert lines
            let mut insert_nodes = upper_end_point_is_top.clone();
            insert_nodes.extend(interior_end_point_is_top.clone());
            for index in insert_nodes {
                let insert_node = AllIntersectionsTreeNode{
                    line: &lines[index],
                    index: index,
                    latest_x: top.point.x,
                    latest_y: top.point.y,
                };
                status_structure.insert(insert_node);
            }

            if lower_end_point_is_top.len() + interior_end_point_is_top.len() == 0 {
                // find new events between left line and right line of point p
                let Some(right) = status_structure.range((
                    Excluded(&check_node),
                    Unbounded,
                )).next() else {
                    continue;
                };
                let Some(left) = status_structure.range((
                    Unbounded,
                    Excluded(&check_node),
                )).last() else {
                    continue;
                };

                let Some(point) = right.line.point_of_interception(&left.line) else {
                    continue;
                };

                q.push(Reverse(AllIntersectionsQueueItemBook{
                    point: point,
                }));
            } else {
                // find new events of the left and right most line in U(P) U C(P)
                let nodes : Vec<AllIntersectionsTreeNode> = vec![
                    upper_end_point_is_top,
                    interior_end_point_is_top,
                ].iter().flatten().map(|line_index| {
                    AllIntersectionsTreeNode {
                        line: &lines[*line_index],
                        index: *line_index,
                        latest_x: top.point.x,
                        latest_y: top.point.y,
                    }
                }).collect();

                if let Some((min, max)) = {
                    let mut r : Option<(AllIntersectionsTreeNode, AllIntersectionsTreeNode)> = None;
                    for node in nodes {
                        r = if let Some(r) = r {
                            if node < r.0 {
                                Some((node, r.1))
                            } else if node > r.1 {
                                Some((r.0, node))
                            } else {
                                Some(r)
                            }
                        } else {
                            Some((node, node))
                        }
                    }

                    r
                } {
                    if let Some(left) = status_structure.range((
                        Unbounded,
                        Excluded(min),
                    )).last() {
                        if let Some(point) =
                            min.line.point_of_interception(&left.line) {
                            q.push(Reverse(AllIntersectionsQueueItemBook{
                                point: point,
                            }));
                        }
                    }

                    if let Some(right) = status_structure.range((
                        Excluded(max),
                        Unbounded,
                    )).next() {
                        if let Some(point) =
                            max.line.point_of_interception(&right.line) {
                            q.push(Reverse(AllIntersectionsQueueItemBook{
                                point: point,
                            }));
                        }
                    }
                }
            }

            last_point = top.point;
        }


        return intersection_points;
    }

    pub fn all_intersections_my_version(lines: &Vec<Self>) -> Vec<(Point, usize, usize)> {

        use core::cmp::Reverse;

        let lines : Vec<Self> = lines.iter().map(|line| {
            LineSegment::from_include(line.p1, line.p2, true, true)
        }).collect();

        let mut q = std::collections::binary_heap::BinaryHeap::new();
        for i in 0..lines.len() {
            let p1_top =
                lines[i].p1.y < lines[i].p2.y ||
                (lines[i].p1.y == lines[i].p2.y && lines[i].p1.x < lines[i].p2.x);
            q.push(Reverse(AllIntersectionsQueueItem {
                point: lines[i].p1,
                point_position: if p1_top {
                    AllIntersectionsQueueItemPointType::Top
                }
                else {
                    AllIntersectionsQueueItemPointType::Bottom
                },
                line_segment_index: i,
            }));
            q.push(Reverse(AllIntersectionsQueueItem {
                point: lines[i].p2,
                point_position: if !p1_top {
                    AllIntersectionsQueueItemPointType::Top
                }
                else {
                    AllIntersectionsQueueItemPointType::Bottom
                },
                line_segment_index: i,
            }));
        }

        // (x, index)
        let mut status_structure : std::collections::BTreeSet::<AllIntersectionsTreeNode>
            = std::collections::BTreeSet::<AllIntersectionsTreeNode>::new();

        let mut intersection_points = Vec::new();

        let mut last_x = Vec::new();
        let mut last_y = Vec::new();
        for _ in 0..lines.len() {
            last_x.push(q.peek().unwrap().0.point.x);
            last_y.push(q.peek().unwrap().0.point.y);
        }

        while let Some(Reverse(top)) = q.pop() {

            for s1 in &status_structure {
                let mut compare = std::cmp::Ordering::Greater;
                for s2 in &status_structure {
                    let result = Ord::cmp(s1, s2);
                    if result == compare {
                        continue;
                    }
                    if compare == std::cmp::Ordering::Greater &&
                        result == std::cmp::Ordering::Equal
                    {
                        compare = result;
                        continue;
                    }

                    if compare == std::cmp::Ordering::Equal &&
                        result == std::cmp::Ordering::Less
                    {
                        compare = result;
                        continue;
                    }

                    panic!("Status Structure is not properly sorted");
                }

                assert!(compare != std::cmp::Ordering::Greater);
            }

            let line_index = top.line_segment_index;
            let value = AllIntersectionsTreeNode{
                line: &lines[line_index],
                index: top.line_segment_index,
                latest_x: top.point.x,
                latest_y: top.point.y,
            };

            use std::ops::Bound::*;
            match top.point_position {
                AllIntersectionsQueueItemPointType::Top => {
                    status_structure.insert(value);
                },
                AllIntersectionsQueueItemPointType::Middle => {
                },
                AllIntersectionsQueueItemPointType::Bottom => {
                    let mut value_copy = value.clone();
                    value_copy.latest_x = last_x[value_copy.index];
                    value_copy.latest_y = last_y[value_copy.index];
                    assert!(status_structure.remove(&value_copy));
                },
            }

            let mut values_to_remove_and_insert = Vec::new();
            for (min, max, value, is_reverse) in
                if top.point_position != AllIntersectionsQueueItemPointType::Bottom {vec![
                (Excluded(&value), Unbounded, value, false),
                (Unbounded, Excluded(&value), value, true)
            ]} else {vec![
                if let Some(value) = status_structure.range(
                    (Excluded(&value), Unbounded)
                ).min() {vec![
                    (Excluded(value), Unbounded, *value, false),
                ]} else {
                    vec![]
                },

                if let Some(value) = status_structure.range(
                    (Unbounded, Excluded(&value))
                ).max() {vec![
                    (Unbounded, Excluded(value), *value, true),
                ]} else {
                    vec![]
                }
            ].concat()} {
                let mut range : Box<dyn Iterator<Item=&AllIntersectionsTreeNode>> =
                    Box::from(status_structure.range((min, max)));
                if is_reverse {
                    range = Box::from(status_structure.range((min, max)).rev());
                }

                for next in range {
                    if let Some(p) = value.line.point_of_interception(
                        &next.line
                    ) {
                        let (less_index, higher_index) = if value.index < next.index {
                            (value.index, next.index)
                        } else {
                            (next.index, value.index)
                        };
                        if top.point_position == AllIntersectionsQueueItemPointType::Top {
                            intersection_points.push((p, less_index, higher_index));
                            q.push(Reverse(AllIntersectionsQueueItem{
                                point: p,
                                point_position: AllIntersectionsQueueItemPointType::Middle,
                                line_segment_index: less_index,
                            }));
                        }
                        // q.push(Reverse(AllIntersectionsQueueItem{
                        //     point: p,
                        //     point_position: AllIntersectionsQueueItemPointType::Middle,
                        //     line_segment_index: higher_index,
                        // }));

                        values_to_remove_and_insert.push(AllIntersectionsTreeNode{
                            line: &lines[next.index],
                            index: next.index,
                            latest_x: top.point.x,
                            latest_y: top.point.y,
                        });
                    } else {
                        eprintln!("Did not find a point");
                        break;
                    }
                }
            }
            let status_structure_len = status_structure.len();
            for mut value in &mut values_to_remove_and_insert {
                value.latest_x = last_x[value.index];
                value.latest_y = last_y[value.index];
                assert!(status_structure.remove(&value));
            }
            for mut value in values_to_remove_and_insert {
                value.latest_x = top.point.x;
                value.latest_y = top.point.y;
                status_structure.insert(value);
            }
            assert_eq!(status_structure.len(), status_structure_len);

            last_x[top.line_segment_index] = top.point.x;
            last_y[top.line_segment_index] = top.point.y;
        }

        assert_eq!(status_structure.len(), 0);

        return intersection_points;
    }

    pub fn all_intersections(lines: &Vec<Self>) -> Vec<(Point, usize, usize)> {
        return Self::all_intersections_slow(lines);
    }

    pub fn all_intersections_combine_points(lines: &Vec<Self>, squares_in_one: f64)
        -> Vec<(Point, Vec<(usize, usize)>)> {
        let intersections = LineSegment::all_intersections(&lines);

        let mut point_to_index = std::collections::HashMap::<(usize, usize), usize>::new();
        // The graph. [point_index] = (Point, Set<next point index, line index>)
        let mut point_indexes : Vec<(Point, std::collections::HashSet<(usize, usize)>)> = Vec::new();

        let mut line_to_points = Vec::new();
        for _ in 0..lines.len() {
            line_to_points.push(Vec::new());
        }

        for i in 0..intersections.len() {
            let x_square = (intersections[i].0.x * squares_in_one) as usize;
            let y_square = (intersections[i].0.y * squares_in_one) as usize;

            let key = (x_square, y_square);
            let index = if point_to_index.contains_key(&key) {
                point_to_index[&key]
            } else {
                point_to_index.insert(key, point_indexes.len());
                point_indexes.push((intersections[i].0, std::collections::HashSet::new()));
                point_indexes.len()-1
            };

            line_to_points[intersections[i].1].push((intersections[i].0, index));
            line_to_points[intersections[i].2].push((intersections[i].0, index));
        }

        // sort line_to_points
        for i in 0..line_to_points.len() {
            line_to_points[i].sort_by(|a, b| {
                if a.0.x < b.0.x {
                    std::cmp::Ordering::Less
                } else if a.0.x > b.0.x {
                    std::cmp::Ordering::Greater
                }
                else if a.0.y < b.0.y {
                    std::cmp::Ordering::Less
                } else if a.0.y > b.0.y {
                    std::cmp::Ordering::Greater
                }
                else {
                    std::cmp::Ordering::Equal
                }
            });

            for j in 1..line_to_points[i].len() {
                let x_square_1 = (line_to_points[i][j-1].0.x * squares_in_one) as usize;
                let y_square_1 = (line_to_points[i][j-1].0.y * squares_in_one) as usize;
                let x_square_2 = (line_to_points[i][j].0.x * squares_in_one) as usize;
                let y_square_2 = (line_to_points[i][j].0.y * squares_in_one) as usize;

                let index_1 = point_to_index[&(x_square_1, y_square_1)];
                let index_2 = point_to_index[&(x_square_2, y_square_2)];

                if index_1 == index_2 { continue }
                point_indexes[index_1].1.insert((index_2, i));
                point_indexes[index_2].1.insert((index_1, i));
            }
        }

        return point_indexes.iter().map(|x| {
            let mut v = Vec::new();
            for index in &x.1 { v.push(*index) }
            (x.0, v)
        }).collect();
    }

    pub fn remove_wrong_determinate_polygons(
        lines: &Vec<Self>,
        determinate_is_positive: bool,
    ) -> Vec<Self> {

        let mut best_index = lines.len();
        let mut best_area = 0.0;
        let polygons = Self::line_segments_to_polygons(lines);
        for i in 0..polygons.len() {
            let mut score = 0;
            for j in 0..polygons[i].len() {
                let j_next = (j+1) % polygons[i].len();
                let increment =
                    (lines.len() + polygons[i][j].1 - polygons[i][j_next].1) % lines.len();
                let decrement = (lines.len() - increment) % lines.len();

                if increment > decrement {
                    // score -= decrement as i64;
                    score -= 1;
                } else {
                    // score += increment as i64;
                    score += 1;
                }
            }

            if (score > 0) == determinate_is_positive {
                let area = Point::area(
                    &LineSegment::points(
                        &polygons[i].iter().map(
                            |x| x.0.clone()
                        ).collect(),
                        false,
                    )
                ).abs();

                if area > best_area {
                    best_area = area;
                    best_index = i;
                }
            }
        }

        if best_index < lines.len() {
            return polygons[best_index].iter().map(
                |x| x.0.clone()
            ).collect();
        } else {
            return Vec::new();
        }
    }

    pub fn remove_inner_intersecting_polygons(lines: &Vec<Self>) -> Vec<Self> {
        let graph = LineSegment::all_intersections_combine_points(&lines, 1000.0);

        let mut current_index = 0;
        for i in 1..graph.len() {
            if graph[i].0.y < graph[current_index].0.y ||
                (
                    graph[i].0.y == graph[current_index].0.y &&
                    graph[i].0.x < graph[current_index].0.x
                )
            {
                current_index = i;
            }
        }

        let mut points = Vec::new();
        points.push(current_index);

        let mut next_vertex_tuple = None;
        for i in &graph[current_index].1 {
            let i = i.0;
            if i == current_index {
                continue;
            }
            let angle = graph[i].0.counter_clockwise_angle_to(&graph[current_index].0);
            let Some((next_vertex_angle, _)) = next_vertex_tuple else {
                next_vertex_tuple = Some((angle, i));
                continue;
            };

            if angle < next_vertex_angle {
                next_vertex_tuple = Some((angle, i));
            }
        }
        let Some(next_vertex) = next_vertex_tuple else {
            return Vec::new();
        };
        points.push(next_vertex.1);

        let mut points_seen = vec![false; graph.len()];
        points_seen[points[points.len()-1]] = true;

        while points.len() >= 2 && points[0] != points[points.len()-1] {
            let p1 = points[points.len()-2];
            let p2 = points[points.len()-1];

            let mut next_tuple = None;
            for p3 in &graph[p2].1 {
                if p3.0 == p1 || points_seen[p3.0] { continue; }

                let angle = Point::right_angle(
                    &graph[p1].0,
                    &graph[p2].0,
                    &graph[p3.0].0,
                );

                if let Some((next_angle, _)) = next_tuple {
                    if angle < next_angle {
                        next_tuple = Some((angle, p3.0));
                    }
                } else {
                    next_tuple = Some((angle, p3.0));
                }
            }
            let Some(next_tuple) = next_tuple else {
                eprintln!("POINTS: {:?}", points);
                for (index, (p1, next_point)) in graph.iter().enumerate() {
                    eprintln!("{}) {:?}:", index, p1);
                    eprint!("\t[");
                    for (point_index, _) in next_point {
                        eprint!("{}, ", point_index);
                    }
                    eprintln!("]");
                }
                Self::print_python_code_to_graph(&lines);
                eprintln!("Should have found another point. E0768");
                eprintln!("We will try another way.");

                // panic!("Should have found another point. E0768");

                points.pop();
                continue;
            };

            points.push(next_tuple.1);
            points_seen[next_tuple.1] = true;
        }

        let mut r = Vec::new();
        for i in 1..points.len() {
            r.push(LineSegment::from_ray(
                graph[points[i-1]].0,
                graph[points[i  ]].0,
            ));
        }

        return r;
    }

    pub fn fix_lines_too_close_to_polygon(
        origonal_polygon: &Vec<Self>,
        new_thicker_polygon: &Vec<Self>,
        radius: f64,
    ) -> Vec<Self> {

        if radius == 0.0 || new_thicker_polygon.len() == 0 {
            return new_thicker_polygon.clone();
        }

        // remove unwanted points
        let mut new_points = Vec::new();
        let mut ith_count = new_thicker_polygon[0].times_cross_line_vec(&origonal_polygon);
        for i in 0..new_thicker_polygon.len() {
            let i_next = (i+1) % new_thicker_polygon.len();
            let ith_next_count = new_thicker_polygon[i_next].times_cross_line_vec(&origonal_polygon);
            if ith_count == 0 || ith_next_count > 0 {
                new_points.push(new_thicker_polygon[i].p1);
            }
            ith_count = ith_next_count;
        }

        // Move points away from line by bit radius
        //  that are too close to line.
        let points = new_points;
        let mut new_points = Vec::new();
        for i in 0..points.len() {
            let mut passed = true;
            for j in 0..origonal_polygon.len() {
                let point = origonal_polygon[j]
                    .closest_point_on_line_to_point(&points[i]);
                let distance = point.distance_to(&points[i]);
                if distance < radius - 0.0001 {
                    passed = false;
                    // new_points[i] = point + radius * (new_points[i] - point).normalize();
                    // new_points[i] = point;
                }
            }
            if passed {
                new_points.push(points[i]);
            }
        }

        // Move line away from point by bit radius
        //  that are too close to point.
        //  TODO: I dont think its necessary

        let mut new_lines = Vec::new();
        for i in 0..new_points.len() {
            let j = (i+1) % new_points.len();
            new_lines.push(
                Self::from(
                    new_points[i],
                    new_points[j],
                )
            );
        }

        return new_lines;
    }

    pub fn print_python_code_to_graph(lines: &Vec<Self>) {
        eprintln!("import matplotlib.pyplot as plt\n");
        for line in lines {
            eprintln!("plt.plot([{}, {}], [{}, {}])",
                line.p1.x, line.p2.x, line.p1.y, line.p2.y);
        }
        eprintln!("\nplt.show()");
    }
}

impl Intersection for LineSegment {
    fn find_significant_xs(&self) -> Vec<f64> {
        vec![
            self.p1.x,
            self.p2.x,
        ]
    }
    fn y(&self, next: &Self, x: f64) -> Vec<(f64, bool)> {
        let contains_x =
            (self.p1.x < self.p2.x && x >= self.p1.x && x < self.p2.x) ||
            (self.p1.x > self.p2.x && x <= self.p1.x && x > self.p2.x);
        let end_x = x == self.p2.x;
        let lines_on_same_side_as_x =
            (self.p1.x > x && self.p2.x >= x && next.p1.x >= x && next.p2.x > x) ||
            (self.p1.x < x && self.p2.x <= x && next.p1.x <= x && next.p2.x < x);
        if let Some((b, m)) = self.y_intercept_and_slope() {
            if end_x && lines_on_same_side_as_x {
                return vec![
                    (self.p2.y, true),
                ];
            }
            if self.p1.x == x {
                return vec![(self.p1.y, self.p1.x < self.p2.x)];
            } else if contains_x {
                return vec![(m * x + b, self.p1.x < self.p2.x)];
            } else {
                return Vec::new();
            }
        } else if x == self.p1.x {
            let (little_y, big_y) = if self.p1.y < self.p2.y {
                (self.p1.y, self.p2.y)
            } else {
                (self.p2.y, self.p1.y)
            };
            let epsilon = 0.00001;
            if little_y + epsilon >= big_y - epsilon {
                return Vec::new();
            }

            return vec![
                (little_y + epsilon, true),
                (big_y - epsilon, false),
            ];
        } else {
            return Vec::new();
        }
    }
    fn times_cross_line(&self, line: &LineSegment) -> usize {
        if match (self.y_intercept_and_slope(), line.y_intercept_and_slope()) {
            (Some((b1, m1)), Some((b2, m2))) => {
                let intersecting_x = (b2-b1) / (m1-m2);
                self.contains_x(intersecting_x) &&
                    line.contains_x(intersecting_x)
            }
            (Some((b1, m1)), _) => {
                self.contains_x(line.p1.x) &&
                    line.contains_y(m1 * line.p1.x + b1)
            }
            (_, Some((b2, m2))) => {
                line.contains_x(self.p1.x) &&
                    self.contains_y(m2 * self.p1.x + b2)
            }
            (_, _) => {
                // self.x == line.x && self.range_y is in line.range_y
                self.p1.x == line.p1.x && (
                    (self.p1.y < line.p1.y && line.p1.y < self.p2.y) ||
                    (self.p1.y < line.p2.y && line.p2.y < self.p2.y) ||

                    (self.p1.y <= line.p1.y && line.p1.y < self.p2.y &&
                        self.includes_first_point && line.includes_first_point) ||
                    (self.p1.y <= line.p2.y && line.p2.y < self.p2.y &&
                        self.includes_first_point && line.includes_second_point) ||

                    (self.p1.y < line.p1.y && line.p1.y <= self.p2.y &&
                        self.includes_second_point && line.includes_first_point) ||
                    (self.p1.y < line.p2.y && line.p2.y <= self.p2.y &&
                        self.includes_second_point && line.includes_second_point) ||

                    (self.p1.y <= line.p1.y && line.p1.y <= self.p2.y &&
                        self.includes_first_point && self.includes_second_point &&
                        line.includes_first_point) ||
                    (self.p1.y <= line.p2.y && line.p2.y <= self.p2.y &&
                        self.includes_first_point && self.includes_second_point &&
                        line.includes_second_point)
                )
            }
        } {
            1
        } else {
            0
        }
    }

    fn intersects_rectangle(&self, rect : &Rectangle) -> bool {
        rect.contains_point(self.p1) || rect.contains_point(self.p2) ||
            self.intersects_line(&LineSegment::from(
                    Point::from(rect.p1().x, rect.p1().y),
                    Point::from(rect.p2().x, rect.p1().y),
            )) ||
            self.intersects_line(&LineSegment::from(
                    Point::from(rect.p2().x, rect.p1().y),
                    Point::from(rect.p2().x, rect.p2().y),
            )) ||
            self.intersects_line(&LineSegment::from(
                    Point::from(rect.p2().x, rect.p2().y),
                    Point::from(rect.p1().x, rect.p2().y),
            )) ||
            self.intersects_line(&LineSegment::from(
                    Point::from(rect.p1().x, rect.p2().y),
                    Point::from(rect.p1().x, rect.p1().y),
            ))
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle::from(self.p1, self.p2)
    }

    fn closest_distance_to_point(&self, point: &Point) -> f64 {
        self.distance_to_point(&point)
    }

    fn add_radius(
        items: &Vec<Self>,
        bit_radius: f64,
        cut_inside: bool,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized {
        use geo_types::{LineString, Coordinate, Polygon};
        let mut input : Vec<Coordinate<f64>> =
            items
                .iter()
                .map(|x| {
                    Coordinate {
                        x: x.p1.x,
                        y: x.p1.y,
                    }
                }).collect::<Vec<Coordinate<f64>>>();
        input.push(
            Coordinate {
                x: items[0].p1.x,
                y: items[0].p1.y,
            }
        );
        let input : LineString<f64> = LineString(input);

        let inner_point = Self::find_barely_inner_point(
            &items
                .iter()
                .map(|x|
                    (*x).clone()
                ).collect()
        );

        use geo_clipper::Clipper;

        let subject = Polygon::new(
            input,
            vec![],
        );

        // let result = subject.intersection(&clip, 1.0);
        let result : Vec<(Vec<LineSegment>, bool)> = subject.offset(
            if cut_inside {
                -bit_radius
            } else {
                bit_radius
            },
            // geo_clipper::JoinType::Miter(10.0),
            // geo_clipper::JoinType::Round(10.0),
            geo_clipper::JoinType::Square,
            geo_clipper::EndType::ClosedPolygon,
            1024.0,
        ).0.iter().map(
            |polygon| {
                let (outer, inner) = polygon.clone().into_inner();

                let mut new_lines = vec![
                    (LineSegment::from_linestring(&outer)
                        .iter()
                        .map(|x| x.clone())
                        .collect(),
                        cut_inside,
                    )
                ];

                let mut inner_lines : Vec<(Vec<LineSegment>, bool)> = inner
                    .iter()
                    .map(|linestring|
                        (LineSegment::from_linestring(&linestring)
                            .iter()
                            .map(|x| x.clone())
                            .collect(), !cut_inside)
                    )
                    .collect();
                new_lines.append(&mut inner_lines);

                return new_lines;
            }
        )
            .flatten()
            .collect();
        return result;
        /*
        // if let Ok(lines) = offset_polygon::offset_polygon(
        //     &input,
        //     // if can_cut(inner_point.x, inner_point.y) {
        //     //     bit_radius
        //     // } else {
        //     //     -bit_radius
        //     // },
        //     // 8.0,
        //     1.0,
        //     0.0,
        // ) {
        //     eprintln!("{:?}", lines);
        //     lines
        //         .iter()
        //         .map(|line| {
        //             let points = line.clone().into_points();
        //             let mut new_lines = Vec::new();

        //             for i in 1..points.len() {
        //                 new_lines.push(Box::from(
        //                     LineSegment::from(
        //                         Point::from(
        //                             points[i-1].x(),
        //                             points[i-1].y(),
        //                         ),
        //                         Point::from(
        //                             points[i].x(),
        //                             points[i].y(),
        //                         ),
        //                     )
        //                 ));
        //             }

        //             return new_lines;
        //         })
        //         .collect()
        // } else {
        //     Vec::new()
        // };

        let mut points = Vec::new();
        // points.push(items[0].p1);
        for line in items {
            if points.len() > 0 && line.p1 == points[points.len()-1] {
                continue;
            }
            points.push(line.p1);
        }

        let items_no_address : Vec<Self> = items.iter()
            .map(|x| (*x).clone())
            .collect();

        if points.len() > 0 && points[0] == points[points.len()-1] {
            points.remove(points.len()-1);
        }
        if points.len() < 3 { return Vec::new(); }

        let mut new_points = Vec::new();
        let mut last_dxy_is_positive = true;

        for i in 0..points.len() {
            let i = (points.len() - 1 + i) % points.len();
            let j = (i+1) % points.len();
            let k = (i+2) % points.len();
            let (
                ix, iy,
                jx, jy,
                kx, ky,
            ) = (
                points[i].x, points[i].y,
                points[j].x, points[j].y,
                points[k].x, points[k].y,
            );

            let (ij_dx, ij_dy) = LineSegment::from_ray(
                Point::from(ix, iy),
                Point::from(jx, jy),
            ).find_perpendicular_dx_dy(
                &mut last_dxy_is_positive,
                &mut can_cut,
            );

            let (jk_dx, jk_dy) = LineSegment::from_ray(
                Point::from(jx, jy),
                Point::from(kx, ky),
            ).find_perpendicular_dx_dy(
                &mut last_dxy_is_positive,
                &mut can_cut,
            );


            let line_ij = lines_and_curves::LineSegment::from(
                lines_and_curves::Point::from(
                    ix + ij_dx * bit_radius,
                    iy + ij_dy * bit_radius,
                ),
                lines_and_curves::Point::from(
                    jx + ij_dx * bit_radius,
                    jy + ij_dy * bit_radius,
                ),
            );

            let line_jk = lines_and_curves::LineSegment::from(
                lines_and_curves::Point::from(
                    jx + jk_dx * bit_radius,
                    jy + jk_dy * bit_radius,
                ),
                lines_and_curves::Point::from(
                    kx + jk_dx * bit_radius,
                    ky + jk_dy * bit_radius,
                ),
            );

            if let Some(intersect_point) =
                line_ij.point_of_interception_endless_lines(&line_jk) {
                // assert!(
                //     bit_radius < 0.001 ||
                //     can_cut(intersect_point.x, intersect_point.y)
                // );

                // NOTE: Maybe should remove if
                // if bit_radius == 0.0 ||
                //     (
                //         can_cut(intersect_point.x, intersect_point.y) &&
                //         (ij_dx.powi(2) + ij_dy.powi(2) > 0.0) && (jk_dx.powi(2) + jk_dy.powi(2) > 0.0)
                //     )
                // {
                //     // new_points.push(intersect_point);
                // }

                /*
                let line_ik = lines_and_curves::LineSegment::from(
                    points[i],
                    points[k],
                );
                let line_i_intersect_point = lines_and_curves::LineSegment::from(
                    points[i],
                    intersect_point,
                );

                let should_skip = if let Some((_, slope_ik)) = line_ik.y_intercept_and_slope() {
                    if let Some((_, slope_i_ip)) = line_i_intersect_point.y_intercept_and_slope() {
                        if let Some((_, slope_ij)) = line_ij.y_intercept_and_slope() {
                            // if ik in between ij and ip
                            (slope_ik <= slope_ij && slope_ik >= slope_i_ip) ||
                                (slope_ik >= slope_ij && slope_ik <= slope_i_ip)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                if should_skip {
                    // new_points.push(
                    //     Point::zero()
                    // );
                    let mut intersect_point = intersect_point;
                    intersect_point.y += 1.0;
                    new_points.push(intersect_point);
                } else {
                    new_points.push(intersect_point);
                }
                */
                new_points.push(intersect_point);
            } else {
                // return Vec::new();
                // try for something
                panic!("Could not find intersection point trying to add bit radius to lines.");
                assert!(bit_radius < 0.001 || can_cut(line_ij.p2.x, line_ij.p2.y));
                new_points.push(line_ij.p2);
            };
        }

        assert_eq!(points.len(), new_points.len());
        let mut new_x_points = Vec::new();
        for i in 0..new_points.len() {
            new_x_points.push(
                Point::from(
                    new_points[i].x,
                    points[i].y,
                )
            );
        }
        let new_x_area = Point::area(
            &new_x_points
        );

        let area = Point::area(
            &new_points
        );


        // Cut shorter around sharp corners
        let mut points_with_cut = Vec::new();
        for i in 0..new_points.len() {
            let i = (new_points.len() - 1 + i) % new_points.len();
            let j = (i+1) % new_points.len();
            let k = (i+2) % new_points.len();

            let line_ij = lines_and_curves::LineSegment::from_ray(
                new_points[i], new_points[j]
            );

            let line_jk = lines_and_curves::LineSegment::from_ray(
                new_points[j], new_points[k]
            );

            // let line_ik = lines_and_curves::LineSegment::from_ray(
            //     new_points[i], new_points[k]
            // );

            // let line_ij_og = lines_and_curves::LineSegment::from_ray(
            //     points[i], points[j]
            // );
            // let line_i_ip = lines_and_curves::LineSegment::from_ray(
            //     points[i], new_points[j]
            // );

            let intersect_point = new_points[j];

            /*
            let should_skip = if let Some((_, slope_ik)) = line_ik.y_intercept_and_slope() {
                if let Some((_, slope_i_ip)) = line_i_ip.y_intercept_and_slope() {
                    if let Some((_, slope_ij)) = line_ij_og.y_intercept_and_slope() {
                        // if ik in between ij and ip
                        (slope_ik <= slope_ij && slope_ik >= slope_i_ip) ||
                            (slope_ik >= slope_ij && slope_ik <= slope_i_ip)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if should_skip {
                // points_with_cut.push(line_ik.mid_point());
                // continue;
            }
            */

            let dij = (line_ij.p1 - line_ij.p2) / line_ij.length();
            let djk = (line_jk.p2 - line_jk.p1) / line_jk.length();
            let d = (dij + djk) / 2.0;
            let d = d / d.distance_to(&Point::zero());
            let inverse_d = Point::from(d.y, -d.x);

            let new_p1 = points[j] + inverse_d - d * bit_radius.abs();
            let new_p2 = points[j] - inverse_d - d * bit_radius.abs();

            let cut_line = LineSegment::from_ray(new_p1, new_p2);

            let (Some(cut_ij), Some(cut_jk)) =
                (cut_line.point_of_interception_endless_lines(&line_ij),
                 cut_line.point_of_interception_endless_lines(&line_jk))
            else {
                points_with_cut.push(intersect_point);
                continue;
            };

            if // bit_radius > 0.0 &&
                line_ij.contains_point(cut_ij) &&
                line_jk.contains_point(cut_jk) {
                points_with_cut.push(cut_ij);
                points_with_cut.push(cut_jk);
            } else {
                points_with_cut.push(intersect_point);
            }
        }
        let new_points = points_with_cut;

        let mut lines = Vec::new();
        for i in 0..new_points.len() {
            let j = (i+1) % new_points.len();
            lines.push(LineSegment::from_ray(new_points[i], new_points[j]));
        }
        let og_area = Point::area(
            &points,
            // &items
            //     .iter()
            //     .map(|x| x.p1)
            //     .collect::<Vec<Point>>()
        );

        // let lines = Self::remove_inner_intersecting_polygons(&lines);
        let lines = Self::remove_wrong_determinate_polygons(
            &lines,
            og_area < 0.0,
        );
        let lines = Self::fix_lines_too_close_to_polygon(&items_no_address, &lines, bit_radius);
        let lines : Vec<Box<Self>> = lines.iter().map(|line| {
            Box::from(line.clone())
        }).collect();

        if (og_area > 0.0) != (new_x_area > 0.0) {
            return Vec::new();
        }

        return vec![lines];
        */
    }

    fn remove_touching_shapes(
        shapes: &Vec<(Vec<Self>, bool)>,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized {
        if shapes.len() == 0 {
            return Vec::new();
        }
        let mut shapes = shapes.clone();

        let mut found_intersection = true;
        while found_intersection {
            found_intersection = false;
            'intersection_searcher: for i in 0..shapes.len() {
                for j in (i+1)..shapes.len() {

                    if Self::does_intersect(&shapes[i].0, &shapes[j].0) {
                        let shape_i_can_cut = shapes[i].1;
                        let shape_j_can_cut = shapes[j].1;

                        if shape_i_can_cut == shape_j_can_cut {
                            let mut union = Self::union(
                                &shapes[i].0, &shapes[j].0, shape_i_can_cut,
                            );
                            if union.len() == 2 &&
                                union[0].1 == shape_i_can_cut &&
                                union[1].1 == shape_j_can_cut {
                                    continue;
                            }
                            shapes.append(
                                &mut union
                            );
                        } else if shape_i_can_cut {
                            shapes.append(
                                &mut Self::differance(
                                    &shapes[i].0, &shapes[j].0, shape_i_can_cut,
                                ),
                            );
                        } else {
                            shapes.append(&mut Self::differance(
                                &shapes[j].0, &shapes[i].0, shape_j_can_cut));
                        }

                        shapes.swap_remove(j);
                        shapes.swap_remove(i);
                        found_intersection = true;
                        break 'intersection_searcher;
                    }
                }
            }
        }

        return shapes.clone();

        /*

        let polygons = Self::get_outlines_and_inlines_of_touching_polygons(&shapes);
        let polygons_box : Vec<(Vec<Self>, bool)> = polygons.iter()
            .map(
                |poly| poly.iter().map(
                    |line| line.clone()
                ).collect()
            ).collect();

        return polygons_box;
        */
    }

    fn find_barely_inner_point(lines: &Vec<Self>) -> Point where Self : Sized {
        return Self::find_barely_inner_point(lines);
    }

    fn force_counter_clockwise(lines: &Vec<Self>) -> Vec<Self> where Self : Sized {
        Self::from_line_to_line_counter_clockwise(&lines)
    }
}

impl cnc_router::CNCPath for LineSegment {
    fn to_path(
        &self
    ) -> Vec<cnc_router::OptionalCoordinate> {
        vec![
            cnc_router::OptionalCoordinate::from(
                Some(self.p2.x),
                Some(self.p2.y),
                None,
            )
        ]
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn start_path(&self) -> Option<cnc_router::Coordinate> {
        Some(cnc_router::Coordinate::from(
            self.p1.x,
            self.p1.y,
            0.0
        ))
    }
}

impl Intersection for Rectangle {
    fn find_significant_xs(&self) -> Vec<f64> {
        vec![
            self.min_x(),
            self.max_x(),
        ]
    }
    fn y(&self, _next: &Self, x: f64) -> Vec<(f64, bool)> {
        if self.contains_x(x) {
            eprintln!("RECTANGLE USED");
            vec![
                (self.min_y(), true),
                (self.max_y(), false),
            ]
        } else {
            Vec::new()
        }
    }

    fn times_cross_line(&self, line: &LineSegment) -> usize {
        LineSegment::from_ray(
            self.start_point,
            Point::from(self.end_point.x, self.start_point.y),
        ).times_cross_line(line)
            +
        LineSegment::from_ray(
            Point::from(self.end_point.x, self.start_point.y),
            self.end_point,
        ).times_cross_line(line)
            +
        LineSegment::from_ray(
            self.end_point,
            Point::from(self.start_point.x, self.end_point.y),
        ).times_cross_line(line)
            +
        LineSegment::from_ray(
            Point::from(self.start_point.x, self.end_point.y),
            self.start_point,
        ).times_cross_line(line)
        // if line.intersects_rectangle(self) {
        //     1
        // } else {
        //     0
        // }
    }

    fn intersects_rectangle(&self, rect : &Rectangle) -> bool {
        // Use below if they intersect at all. Issue is we want to treat
        //  a rect as just 4 lines.
        (self.contains_x(rect.start_point.x) || self.contains_x(rect.end_point.x) ||
            rect.contains_x(self.start_point.x)) &&
        (self.contains_y(rect.start_point.y) || self.contains_y(rect.end_point.y) ||
         rect.contains_y(self.start_point.y))
        // rect.intersects_line(
        //     &LineSegment::from(
        //         self.start_point,
        //         Point::from(self.end_point.x, self.start_point.y),
        //     )
        // )
        // || rect.intersects_line(
        //     &LineSegment::from(
        //         Point::from(self.end_point.x, self.start_point.y),
        //         self.end_point,
        //     )
        // )
        // || rect.intersects_line(
        //     &LineSegment::from(
        //         self.end_point,
        //         Point::from(self.start_point.x, self.end_point.y),
        //     )
        // )
        // || rect.intersects_line(
        //     &LineSegment::from(
        //         Point::from(self.start_point.x, self.end_point.y),
        //         self.start_point,
        //     )
        // )
    }

    fn bounding_box(&self) -> Rectangle {
        self.clone()
    }

    fn closest_distance_to_point(&self, point: &Point) -> f64 {
        let mut min = f64::MAX;
        let points = self.to_points();
        for i in 0..points.len() {
            let line = LineSegment::from(
                points[i], points[(i+1)%points.len()], 
            );
            let distance = line.distance_to_point(&point);
            if distance < min {
                min = distance;
            }
        }

        return min;
    }

    fn add_radius(
        items: &Vec<Self>,
        radius: f64,
        cut_inside: bool,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized {
        let mut new_rects = Vec::new();
        let cut_inside_factor = if cut_inside {
            -1.0
        } else {
            1.0
        };

        for rect in items {
            new_rects.push(
                Rectangle::from(
                    Point::from(
                        rect.min_x() - radius * cut_inside_factor,
                        rect.min_y() - radius * cut_inside_factor,
                    ),
                    Point::from(
                        rect.max_x() + radius * cut_inside_factor,
                        rect.max_y() + radius * cut_inside_factor,
                    ),
                )
            );
        }
        return vec![(new_rects, cut_inside)];
    }

    fn remove_touching_shapes(
        shapes: &Vec<(Vec<Self>, bool)>,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized {
        return shapes.iter().map(|x| {
            x.clone()
        }).collect();
    }

    fn find_barely_inner_point(lines: &Vec<Self>) -> Point where Self : Sized {
        let Some(rect) = lines.first() else {
            panic!("Could not find a rectangle.");
        };

        let x = rect.min_x();
        let y = rect.min_y();
        let epsilon = 0.00001;

        return Point::from(x+epsilon, y + epsilon);
    }

    fn force_counter_clockwise(lines: &Vec<Self>) -> Vec<Self> where Self : Sized {
        lines.clone()
    }
}

impl cnc_router::CNCPath for Rectangle {
    fn to_path(
        &self
    ) -> Vec<cnc_router::OptionalCoordinate> {
        vec![
            cnc_router::OptionalCoordinate::from(
                Some(self.start_point.x),
                Some(self.end_point.y),
                None,
            ),
            cnc_router::OptionalCoordinate::from(
                Some(self.end_point.x),
                Some(self.end_point.y),
                None,
            ),
            cnc_router::OptionalCoordinate::from(
                Some(self.end_point.x),
                Some(self.start_point.y),
                None,
            ),
            cnc_router::OptionalCoordinate::from(
                Some(self.start_point.x),
                Some(self.start_point.y),
                None,
            ),
        ]
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn start_path(&self) -> Option<cnc_router::Coordinate> {
        Some(cnc_router::Coordinate::from(
            self.start_point.x,
            self.start_point.y,
            0.0
        ))
    }
}

impl Intersection for Circle {
    fn find_significant_xs(&self) -> Vec<f64> {
        vec![
            self.center.x - self.radius,
            self.center.x + self.radius,
        ]
    }
    fn y(&self, _next: &Self, x: f64) -> Vec<(f64, bool)> {
        // X^2 + Y^2 = r^2
        // Y^2 = r^2 - X^2
        // Y += sqrt(r^2 - X^2)
        if self.contains_x(x) {
            let difx = x - self.center.x;
            let positive_value = (self.radius * self.radius - difx * difx).sqrt();
            vec![
                (self.center.y + positive_value, false),
                (self.center.y - positive_value, true),
            ]
        } else {
            Vec::new()
        }
    }

    fn times_cross_line(&self, line: &LineSegment) -> usize {
        if self.distance_to_center(&line) <= self.radius {
            if self.center.distance_to(&line.p1) <= self.radius &&
                self.center.distance_to(&line.p2) < self.radius {
                0
            }
            else if
                (self.center.distance_to(&line.p1) <= self.radius &&
                 self.center.distance_to(&line.p2) > self.radius) ||
                (self.center.distance_to(&line.p1) >= self.radius &&
                 self.center.distance_to(&line.p2) < self.radius)
            {
                1
            } else {
                2
            }
        } else {
            0
        }
    }

    fn intersects_rectangle(&self, rect : &Rectangle) -> bool {
        for line in &rect.to_lines() {
            if self.times_cross_line(&line) > 0 {
                return true;
            }
        }

        return false;
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle::from(
            Point::from(
                self.center.x - self.radius,
                self.center.y - self.radius,
            ),
            Point::from(
                self.center.x + self.radius,
                self.center.y + self.radius,
            ),
        )
    }

    fn closest_distance_to_point(&self, point: &Point) -> f64 {
        let distance = self.center.distance_to(&point) - self.radius;
        if distance < 0.0 {
            0.0
        } else {
            distance
        }
    }

    fn add_radius(
        items: &Vec<Self>,
        radius: f64,
        cut_inside: bool,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized {
        let mut v = Vec::new();

        for circle in items {
            v.push(Circle{
                center: circle.center,
                radius: circle.radius + if cut_inside {
                    -radius
                } else {
                    radius
                },
            });
        }

        return vec![(v, cut_inside)];
    }

    fn remove_touching_shapes(
        shapes: &Vec<(Vec<Self>, bool)>,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized {
        return shapes.iter().map(|x| {
            x.clone()
        }).collect();
    }

    fn find_barely_inner_point(lines: &Vec<Self>) -> Point where Self : Sized {
        let Some(circle) = lines.first() else {
            panic!("Could not find a circle.");
        };

        let epsilon = 0.00001;
        circle.center + Point::from(circle.radius - epsilon, 0.0)
    }
    fn force_counter_clockwise(lines: &Vec<Self>) -> Vec<Self> where Self : Sized {
        lines.clone()
    }
}

impl cnc_router::CNCPath for Circle {
    fn to_path(
        &self
    ) -> Vec<cnc_router::OptionalCoordinate> {
        Vec::new()
    }

    fn is_connected(&self) -> bool {
        false
    }

    fn start_path(&self) -> Option<cnc_router::Coordinate> {
        Some(cnc_router::Coordinate::from(
            self.center.x - self.radius,
            self.center.y,
            9.0,
        ))
    }

    fn follow_path<T: std::io::Write>(
        &self,
        cnc_router: &mut cnc_router::CNCRouter<T>,
        feed_rate: Option<f64>,
    ) {
        cnc_router.circular_interpolation_around_change_midpoint(
            true,
            feed_rate,
            self.radius,
            0.0,
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum RectangleConnectionsIterateType {
    SmallerRectangles,
    BiggerRectangles,
}

#[derive(Debug, Clone)]
pub struct RectangleConnections {
    tile_width: f64,
    tile_height: f64,
    rectangle: Rectangle,
    tiles: Vec<Vec<bool>>,
    iterate_type: RectangleConnectionsIterateType,
}

impl RectangleConnections {
    pub fn from(tile_width: f64, tile_height: f64, rectangle: Rectangle) -> Self {
        let mut tiles = Vec::new();
        for _ in 0..(rectangle.width() / tile_width) as usize {
            let mut row = Vec::new();
            for _ in 0..(rectangle.height() / tile_height) as usize {
                row.push(false);
            }
            tiles.push(row);
        }
        Self {
            tile_width: tile_width,
            tile_height: tile_height,
            tiles: tiles,
            rectangle: rectangle,
            iterate_type: RectangleConnectionsIterateType::SmallerRectangles,
        }
    }

    fn convert_x(&self, x: f64) -> usize {
        ((x - self.rectangle.min_x()) / self.tile_width) as usize
    }

    fn convert_y(&self, y: f64) -> usize {
        ((y - self.rectangle.min_y()) / self.tile_height) as usize
    }

    pub fn add_rect(&mut self, point: &lines_and_curves::Point) {
        let min_x = self.convert_x(point.x);
        let min_y = self.convert_y(point.y);
        let mut max_x = self.convert_x(point.x + self.tile_width);
        let mut max_y = self.convert_y(point.y + self.tile_height);

        if self.tiles.len() == 0 || self.tiles[0].len() == 0 {
            return;
        }

        // Rarely does it go over this amount
        if max_x >= self.tiles.len() {
            max_x = self.tiles.len();
        }
        if max_y >= self.tiles[0].len() {
            max_y = self.tiles[0].len();
        }

        for x in min_x..max_x {
            for y in min_y..max_y {
                self.tiles[x][y] = true;
                self.tiles[x][y] = true;
            }
        }
    }

    pub fn to_smaller_rect_iter(&mut self) -> &mut Self {
        self.iterate_type = RectangleConnectionsIterateType::SmallerRectangles;
        return self;
    }

    pub fn to_bigger_rect_iter(&mut self) -> &mut Self {
        self.iterate_type = RectangleConnectionsIterateType::BiggerRectangles;
        return self;
    }
}

impl IntoIterator for RectangleConnections {
    type Item = Rectangle;
    type IntoIter = RectangleConnectionsItr;

    fn into_iter(self) -> Self::IntoIter {
        RectangleConnectionsItr {
            rectangle: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RectangleConnectionsItr {
    rectangle: RectangleConnections,
    x: usize,
    y: usize,
}

impl RectangleConnectionsItr {
    fn increment_by_one_x(&mut self) {
        self.x += 1;
        if self.x >= self.rectangle.tiles.len() {
            self.x = 0;
            self.y += 1;
        }
    }
}

impl Iterator for RectangleConnectionsItr {
    type Item = Rectangle;
    fn next(&mut self) -> Option<Self::Item> {

        while self.x < self.rectangle.tiles.len() &&
            self.y < self.rectangle.tiles[0].len() &&
            !self.rectangle.tiles[self.x][self.y] {

            self.increment_by_one_x();
        }

        if self.x >= self.rectangle.tiles.len() ||
            self.y >= self.rectangle.tiles[0].len()
        {
            return None;
        }


        let x = self.x as f64 * self.rectangle.tile_width +
            self.rectangle.rectangle.min_x();
        let y = self.y as f64 * self.rectangle.tile_height +
            self.rectangle.rectangle.min_y();

        let mut dx = 1;
        let mut dy = 1;

        // Find lines dx and dy for large group rect
        if let RectangleConnectionsIterateType::BiggerRectangles = self.rectangle.iterate_type {
            // find max dx
            while self.x + dx < self.rectangle.tiles.len() &&
                self.rectangle.tiles[self.x + dx][self.y] {
                self.rectangle.tiles[self.x + dx][self.y] = false;
                dx += 1;
            }

            // find max dy
            'outer: while self.y + dy < self.rectangle.tiles[self.x+dx-1].len() {
                // ensure [0,dx][dy] is true
                for d_itr in 0..dx {
                    if !self.rectangle.tiles[self.x+d_itr][self.y+dy] {
                        break 'outer
                    }
                }
                // make them false
                for d_itr in 0..dx {
                    self.rectangle.tiles[self.x+d_itr][self.y+dy] = false;
                }

                dy += 1;
            }
        }

        let item = Rectangle::from(
            Point::from(
                x, y
            ),
            Point::from(
                x + self.rectangle.tile_width * (dx as f64),
                y + self.rectangle.tile_height * (dy as f64),
            ),
        );

        self.increment_by_one_x();

        return Some(item);
    }
}

impl Intersection for AllIntersections {
    fn find_significant_xs(&self) -> Vec<f64> {
        match self {
            AllIntersections::Rectangle(r) => {
                Intersection::find_significant_xs(r)
            }
            AllIntersections::LineSegment(s) => {
                Intersection::find_significant_xs(s)
            }
            AllIntersections::SoftLineSegment(s) => {
                Intersection::find_significant_xs(s)
            }
            AllIntersections::Circle(c) => {
                Intersection::find_significant_xs(c)
            }
        }
    }
    fn y(&self, next: &Self, x: f64) -> Vec<(f64, bool)> {
        match (self, next) {
            (AllIntersections::Rectangle(r1), AllIntersections::Rectangle(r2)) => {
                r1.y(&r2, x)
            }
            (AllIntersections::LineSegment(s1), AllIntersections::LineSegment(s2)) => {
                Intersection::y(s1, &s2, x)
            }
            (AllIntersections::SoftLineSegment(s1), AllIntersections::SoftLineSegment(s2)) => {
                Intersection::y(s1, &s2, x)
            }
            (AllIntersections::Circle(c1), AllIntersections::Circle(c2)) => {
                Intersection::y(c1, &c2, x)
            }
            _ => {
                panic!("Their are multiple types under AllIntersections in the same sign.shape");
            }
        }
    }
    fn times_cross_line(&self, line: &LineSegment) -> usize {
        match self {
            AllIntersections::Rectangle(r) => {
                r.times_cross_line(line)
            }
            AllIntersections::LineSegment(s) => {
                s.times_cross_line(line)
            }
            AllIntersections::SoftLineSegment(s) => {
                s.times_cross_line(line)
            }
            AllIntersections::Circle(c) => {
                c.times_cross_line(line)
            }
        }
    }
    fn intersects_rectangle(&self, rect : &Rectangle) -> bool {
        match self {
            AllIntersections::Rectangle(r) => {
                r.intersects_rectangle(&rect)
            }
            AllIntersections::LineSegment(s) => {
                s.intersects_rectangle(&rect)
            }
            AllIntersections::SoftLineSegment(s) => {
                s.intersects_rectangle(&rect)
            }
            AllIntersections::Circle(c) => {
                c.intersects_rectangle(&rect)
            }
        }
    }
    fn bounding_box(&self) -> Rectangle {
        match self {
            AllIntersections::Rectangle(r) => {
                r.bounding_box()
            }
            AllIntersections::LineSegment(s) => {
                s.bounding_box()
            }
            AllIntersections::SoftLineSegment(s) => {
                s.bounding_box()
            }
            AllIntersections::Circle(c) => {
                c.bounding_box()
            }
        }
    }
    fn closest_distance_to_point(&self, point: &Point) -> f64 {
        match self {
            AllIntersections::Rectangle(r) => {
                r.closest_distance_to_point(&point)
            }
            AllIntersections::LineSegment(s) => {
                s.closest_distance_to_point(&point)
            }
            AllIntersections::SoftLineSegment(s) => {
                s.closest_distance_to_point(&point)
            }
            AllIntersections::Circle(c) => {
                c.closest_distance_to_point(&point)
            }
        }
    }

    fn add_radius(
        items: &Vec<Self>,
        bit_radius: f64,
        cut_inside: bool,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized {
        if items.len() == 0 {
            return Vec::new();
        }

        match items[0] {
            AllIntersections::Rectangle(_) => {
                let v : Vec<Rectangle> = items.iter().filter_map(|x| {
                    if let AllIntersections::Rectangle(r) = x {
                        Some(r.clone())
                    } else {
                        None
                    }
                }).collect();
                Intersection::add_radius(
                    &v,
                    bit_radius,
                    cut_inside,
                ).iter().map(|r| {
                    (
                        r.0.iter().map(|r| {
                            AllIntersections::Rectangle(r.clone())
                        }).collect(),
                        r.1,
                    )
                }).collect()
            }
            AllIntersections::LineSegment(_) => {
                let v = items.iter().filter_map(|x| {
                    if let AllIntersections::LineSegment(l) = x {
                        Some(l.clone())
                    } else {
                        None
                    }
                }).collect();
                Intersection::add_radius(
                    &v,
                    bit_radius,
                    cut_inside,
                ).iter().map(|l| {
                    (
                        l.0.iter().map(|l| {
                            AllIntersections::LineSegment(l.clone())
                        }).collect(),
                        l.1,
                    )
                }).collect()
            }
            AllIntersections::SoftLineSegment(_) => {
                let v = items.iter().filter_map(|x| {
                    if let AllIntersections::SoftLineSegment(l) = x {
                        Some(l.clone())
                    } else {
                        None
                    }
                }).collect();
                Intersection::add_radius(
                    &v,
                    bit_radius,
                    cut_inside,
                ).iter().map(|l| {
                    (
                        l.0.iter().map(|l| {
                            AllIntersections::SoftLineSegment(l.clone())
                        }).collect(),
                        l.1,
                    )
                }).collect()
            }
            AllIntersections::Circle(_) => {
                let v = items.iter().filter_map(|x| {
                    if let AllIntersections::Circle(c) = x {
                        Some(c.clone())
                    } else {
                        None
                    }
                }).collect();
                Intersection::add_radius(
                    &v,
                    bit_radius,
                    cut_inside,
                ).iter().map(|c| {
                    (
                        c.0.iter().map(|c| {
                            AllIntersections::Circle(c.clone())
                        }).collect(),
                        c.1,
                    )
                }).collect()
            }
        }
    }

    fn remove_touching_shapes(
        shapes: &Vec<(Vec<Self>, bool)>,
    ) -> Vec<(Vec<Self>, bool)> where Self : Sized {
        let mut line_segment_shapes = Vec::new();
        let mut soft_line_segment_shapes = Vec::new();
        let mut circle_shapes = Vec::new();
        let mut rectangle_shapes = Vec::new();

        for shape in shapes {
            let mut line_segments = Vec::new();
            let mut soft_line_segments = Vec::new();
            let mut circles = Vec::new();
            let mut rectangles = Vec::new();
            for intersection in &shape.0 {
                let intersection = intersection.clone();
                match intersection {
                    AllIntersections::Rectangle(r) => {
                        rectangles.push(r)
                    },
                    AllIntersections::SoftLineSegment(l) => {
                        soft_line_segments.push(l)
                    },
                    AllIntersections::LineSegment(l) => {
                        line_segments.push(l)
                    },
                    AllIntersections::Circle(c) => {
                        circles.push(c)
                    },
                    _ => {},
                }
            }

            match (
                line_segments.len(), soft_line_segments.len(),
                circles.len(), rectangle_shapes.len()
            ) {
                (x, 0, 0, 0) if x > 0 => {
                    line_segment_shapes.push((line_segments, shape.1))
                }
                (0, x, 0, 0) if x > 0 => {
                    soft_line_segment_shapes.push((soft_line_segments, shape.1))
                }
                (0, 0, x, 0) if x > 0 => {
                    circle_shapes.push((circles, shape.1))
                }
                (0, 0, 0, x) if x > 0 => {
                    rectangle_shapes.push((rectangles, shape.1))
                }
                _ => {

                }
            }
        }

        let line_segment_shapes = Intersection::remove_touching_shapes(
            &line_segment_shapes,
        );
        let soft_line_segment_shapes = Intersection::remove_touching_shapes(
            &soft_line_segment_shapes,
        );

        let mut r = Vec::new();
        for shape in line_segment_shapes {
            let item = (
                shape.0.iter().map(|x| {
                    AllIntersections::LineSegment((*x).clone())
                }).collect(),
                shape.1
            );
            r.push(item);
        }

        for shape in soft_line_segment_shapes {
            let item = (
                shape.0.iter().map(|x| {
                    AllIntersections::SoftLineSegment((*x).clone())
                }).collect(),
                shape.1,
            );
            r.push(item);
        }

        return r;
    }

    fn find_barely_inner_point(lines: &Vec<Self>) -> Point where Self : Sized {
        let Some(shape) = lines.first() else {
            panic!("Could not find a shape.");
        };
        match shape {
            AllIntersections::Rectangle(_) => {
                let v : Vec<Rectangle> = lines.iter().filter_map(|x| {
                    if let AllIntersections::Rectangle(r) = x {
                        Some(r.clone())
                    } else {
                        None
                    }
                }).collect();
                Intersection::find_barely_inner_point(&v)
            }
            AllIntersections::LineSegment(_) => {
                let v : Vec<LineSegment> = lines.iter().filter_map(|x| {
                    if let AllIntersections::LineSegment(l) = x {
                        Some(l.clone())
                    } else {
                        None
                    }
                }).collect();
                Intersection::find_barely_inner_point(&v)
            }
            AllIntersections::SoftLineSegment(_) => {
                let v : Vec<LineSegment> = lines.iter().filter_map(|x| {
                    if let AllIntersections::SoftLineSegment(l) = x {
                        Some(l.clone())
                    } else {
                        None
                    }
                }).collect();
                Intersection::find_barely_inner_point(&v)
            }
            AllIntersections::Circle(_) => {
                let v : Vec<Circle> = lines.iter().filter_map(|x| {
                    if let AllIntersections::Circle(c) = x {
                        Some(c.clone())
                    } else {
                        None
                    }
                }).collect();
                Intersection::find_barely_inner_point(&v)
            }
        }
    }
    fn force_counter_clockwise(lines: &Vec<Self>) -> Vec<Self> where Self : Sized {
        let (rects, softs, lines, circles) = AllIntersections::seperate_vec(lines);
        Self::join_all(
            &Intersection::force_counter_clockwise(&rects),
            &Intersection::force_counter_clockwise(&softs),
            &Intersection::force_counter_clockwise(&lines),
            &Intersection::force_counter_clockwise(&circles),
        )
    }

}

impl cnc_router::CNCPath for AllIntersections {
    fn to_path(
        &self
    ) -> Vec<cnc_router::OptionalCoordinate> {
        match &self {
            AllIntersections::Rectangle(r) => {
                r.to_path()
            }
            AllIntersections::LineSegment(s) => {
                s.to_path()
            }
            AllIntersections::SoftLineSegment(s) => {
                s.to_path()
            }
            AllIntersections::Circle(c) => {
                c.to_path()
            }
        }
    }

    fn is_connected(&self) -> bool {
        match self {
            AllIntersections::Rectangle(r) => {
                r.is_connected()
            }
            AllIntersections::LineSegment(s) => {
                s.is_connected()
            }
            AllIntersections::SoftLineSegment(s) => {
                s.is_connected()
            }
            AllIntersections::Circle(c) => {
                c.is_connected()
            }
        }
    }

    fn start_path(&self) -> Option<cnc_router::Coordinate> {
        match self {
            AllIntersections::Rectangle(r) => {
                r.start_path()
            }
            AllIntersections::LineSegment(s) => {
                s.start_path()
            }
            AllIntersections::SoftLineSegment(s) => {
                s.start_path()
            }
            AllIntersections::Circle(c) => {
                c.start_path()
            }
        }
    }

    fn follow_path<T: std::io::Write>(
        &self,
        mut cnc_router: &mut cnc_router::CNCRouter<T>,
        feed_rate: Option<f64>,
    ) {
        match self {
            AllIntersections::Rectangle(r) => {
                r.follow_path(&mut cnc_router, feed_rate)
            }
            AllIntersections::LineSegment(s) => {
                s.follow_path(&mut cnc_router, feed_rate)
            }
            AllIntersections::SoftLineSegment(_) => {

            }
            AllIntersections::Circle(c) => {
                c.follow_path(&mut cnc_router, feed_rate)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    pub fn test_float(l: f64, r: f64) -> bool {
        if (l.is_nan() || r.is_nan()) && l != r {
            println!("L: {}, R: {}", l, r);
            return false;
        }
        if (l-r).abs() >= 0.000000000001 {
            println!("L: {}, R: {}", l, r);
            return false;
        }
        return true;
    }

    pub fn test_point(l: Point, r: Point) -> bool {
        test_float(l.x, r.x) && test_float(l.y, r.y)
    }

    pub fn test_option_point(l: Option<Point>, r: Option<Point>) -> bool {
        match (l, r) {
            (Some(l), Some(r)) => test_point(l, r),
            (None, None) => true,
             (_, _) => false,
        }
    }

    #[test]
    pub fn test_contains_point() {
        let l1 = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 0.0),
        );
        let l2 = LineSegment::from(
            Point::from(1.0, 0.0),
            Point::from(2.0, 0.0),
        );

        let point = Point::from(1.0, 0.0);
        assert!(l1.contains_point(point));
        assert!(l2.contains_point(point));
    }

    #[test]
    pub fn test_point_of_interception_example1() {
        let line1 = LineSegment::from(
            Point { x: 1.9998131482908181, y: 1.9999000000000002 },
            Point { x: 6.0001868517091825, y: 1.9999 }
        );
        let line2 = LineSegment::from(
            Point { x: 6.0001868517091825, y: 1.9999 },
            Point { x: 4.0, y: 5.000180277563774 }
        );

        let expected_point = Point { x: 6.0001868517091825, y: 1.9999 };
        let r = line1.point_of_interception(&line2);
        assert!(test_option_point(r, Some(expected_point)));
    }

    #[test]
    pub fn test_point_of_interception_example2() {
        let line1 = LineSegment {
            p1: Point { x: 7.192695823314598, y: 5.874999999999999 },
            p2: Point { x: 7.8149446973032, y: 5.875 },
            includes_first_point: true, includes_second_point: false,
        };
        let line2 = LineSegment {
            p1: Point { x: 7.567629814147949, y: 5.875 },
            p2: Point { x: 9.772912979125977, y: 5.875 },
            includes_first_point: true, includes_second_point: false,
        };

        let expected_point = Point { x: 7.8149446973032, y: 5.875 };
        let r = line1.point_of_interception(&line2);
        assert!(test_option_point(r, Some(expected_point)));
    }

    #[test]
    pub fn test_point_of_interception_horizontal_line() {
        let l1 = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 0.0),
        );
        let l2 = LineSegment::from(
            Point::from(1.0, 0.0),
            Point::from(2.0, 0.0),
        );

        let expected_point = Point::from(1.0, 0.0);
        let r = l1.point_of_interception(&l2);

        if let Some(p) = r {
            if !test_point(p, expected_point) {
                assert_eq!(p, expected_point);
            }
        } else {
            assert_eq!(
                r, Some(expected_point)
            );
        }
    }

    #[test]
    pub fn test_point_of_interception_vertical_line() {
        let l1 = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(0.0, 1.0),
        );
        let l2 = LineSegment::from(
            Point::from(0.0, 1.0),
            Point::from(0.0, 2.0),
        );

        let expected_point = Point::from(0.0, 1.0);
        let r = l1.point_of_interception(&l2);

        if let Some(p) = r {
            if !test_point(p, expected_point) {
                assert_eq!(p, expected_point);
            }
        } else {
            assert_eq!(
                r, Some(expected_point)
            );
        }
    }

    #[test]
    pub fn test_point_counter_clockwise_angle_to() {
        let center = Point::from(0.0, 0.0);
        let point1 = Point::from(1.0, 0.0);
        let point2 = Point::from(0.0, 1.0);
        let point3 = Point::from(-1.0, 0.0);
        let point4 = Point::from(0.0, -1.0);

        let PI = std::f64::consts::PI;
        assert!(test_float(point1.counter_clockwise_angle_to(&center), 0.0));
        assert!(test_float(point2.counter_clockwise_angle_to(&center), 0.5 * PI));
        assert!(test_float(point3.counter_clockwise_angle_to(&center), PI));
        assert!(test_float(point4.counter_clockwise_angle_to(&center), 1.5*PI));
    }

    #[test]
    pub fn test_point_counter_clockwise_angle_to_example1() {
        let center = Point::from(4.66666, 4.0);
        let point1  = Point::from(4.0, 3.0);

        let PI = std::f64::consts::PI;
        assert!(test_float(point1.counter_clockwise_angle_to(&center), 4.124390992235938));
    }

    #[test]
    pub fn test_point_right_angle() {
        let center = Point::from(0.0, 0.0);
        let point1 = Point::from(1.0, 0.0);
        let point2 = Point::from(0.0, 1.0);
        let point3 = Point::from(-1.0, 0.0);
        let point4 = Point::from(0.0, -1.0);

        let PI = std::f64::consts::PI;
        assert!(test_float(Point::right_angle(&point4, &center, &point2), PI));
        assert!(test_float(Point::right_angle(&point2, &center, &point3), 0.5*PI));
        assert!(test_float(Point::right_angle(&point3, &center, &point2), 1.5*PI));
        assert!(test_float(Point::right_angle(&point4, &center, &point1), 0.5*PI));
        assert!(test_float(Point::right_angle(&point1, &center, &point4), 1.5*PI));
    }

    #[test]
    pub fn test_point_right_angle_line_segment_indexexample_1() {
        let prev    = Point::from(6.0, 2.0);
        let current = Point::from(4.66666, 4.0);
        let point1 = Point::from(4.0, 3.0);
        let point2 = Point::from(4.0, 5.0);
        let point3 = Point::from(6.0, 6.0);

        let PI = std::f64::consts::PI;
        assert!(test_float(Point::right_angle(&prev, &current, &point1), 5.107182407794509));
        assert!(test_float(Point::right_angle(&prev, &current, &point2), 3.1415857305022192));
        assert!(test_float(Point::right_angle(&prev, &current, &point3), 1.9655828311171426));
    }


    #[test]
    pub fn test_line_contains_xy() {
        let mut line = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 0.0),
        );

        assert_eq!(line.contains_x(0.0), true);
        assert_eq!(line.contains_x(0.5), true);
        assert_eq!(line.contains_x(1.0), true);
        assert_eq!(line.contains_x(1.1), false);
        assert_eq!(line.contains_x(-0.1), false);
        assert_eq!(line.contains_y(0.0), true);
        assert_eq!(line.contains_y(-0.1), false);
        assert_eq!(line.contains_y(0.1), false);

        line.includes_second_point = false;
        assert_eq!(line.contains_x(0.0), true);
        assert_eq!(line.contains_x(0.5), true);
        assert_eq!(line.contains_x(1.0), false);
        assert_eq!(line.contains_x(1.1), false);
        assert_eq!(line.contains_x(-0.1), false);
        assert_eq!(line.contains_y(0.0), true);
        assert_eq!(line.contains_y(-0.1), false);
        assert_eq!(line.contains_y(0.1), false);

        line.includes_first_point = false;
        assert_eq!(line.contains_x(0.0), false);
        assert_eq!(line.contains_x(0.5), true);
        assert_eq!(line.contains_x(1.0), false);
        assert_eq!(line.contains_x(1.1), false);
        assert_eq!(line.contains_x(-0.1), false);
        assert_eq!(line.contains_y(0.0), true);
        assert_eq!(line.contains_y(-0.1), false);
        assert_eq!(line.contains_y(0.1), false);

        let line2 = LineSegment::from_include(
            Point::from(0.0, -1.0),
            Point::from(0.0, 0.0),
            true,
            false,
        );
        assert_eq!(line2.contains_y(0.0), false);
    }

    #[test]
    pub fn test_line_cross_line() {
        let line_h = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 0.0),
        );

        let line_h_shifted = LineSegment::from(
            Point::from(1.1, 0.0),
            Point::from(2.0, 0.0),
        );

        let line_v = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(0.0, 1.0),
        );

        assert_eq!(line_h.times_cross_line(&line_h), 0);
        assert_eq!(line_h.times_cross_line(&line_v), 1);
        assert_eq!(line_h.times_cross_line(&line_h_shifted), 0);
        assert_eq!(line_h_shifted.times_cross_line(&line_v), 0);
    }

    #[test]
    pub fn test_intersects_line() {
        let line_h = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 0.0),
        );

        let line_h_shifted = LineSegment::from(
            Point::from(1.1, 0.0),
            Point::from(2.0, 0.0),
        );

        let line_v = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(0.0, 1.0),
        );

        assert_eq!(line_h.intersects_line(&line_h), false);
        assert_eq!(line_h.intersects_line(&line_v), true);
        assert_eq!(line_h.intersects_line(&line_h_shifted), false);
        assert_eq!(line_h_shifted.intersects_line(&line_v), false);
    }

    #[test]
    pub fn test_intersects_rectangle() {
        let r1 = Rectangle::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 1.0),
        );

        let line_diagonal_through_touching = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 1.0),
        );

        let line_diagonal_through_passing_one_side1 = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(2.0, 2.0),
        );

        let line_diagonal_through_passing_one_side2 = LineSegment::from(
            Point::from(-1.0, -1.0),
            Point::from(1.0, 1.0),
        );

        let line_diagonal_through_passing_both = LineSegment::from(
            Point::from(-1.0, -1.0),
            Point::from(1.0, 1.0),
        );

        let line_inside = LineSegment::from(
            Point::from(0.25, 0.25),
            Point::from(0.75, 0.75),
        );

        let line_horizontal = LineSegment::from(
            Point::from(-1.0, 0.0),
            Point::from(0.0, 0.0),
        );

        let line_vertical = LineSegment::from(
            Point::from(0.5, -1.0),
            Point::from(0.5, 0.1),
        );

        let vertial_below = LineSegment::from(
            Point::from(0.5, -0.1),
            Point::from(0.5, -1.0),
        );

        assert!(line_diagonal_through_touching.intersects_rectangle(&r1));
        assert!(line_diagonal_through_passing_one_side1.intersects_rectangle(&r1));
        assert!(line_diagonal_through_passing_one_side2.intersects_rectangle(&r1));
        assert!(line_diagonal_through_passing_both.intersects_rectangle(&r1));
        assert!(line_inside.intersects_rectangle(&r1));
        assert!(line_horizontal.intersects_rectangle(&r1));
        assert!(line_vertical.intersects_rectangle(&r1));
        assert!(!vertial_below.intersects_rectangle(&r1));
    }

    #[test]
    pub fn test_length() {
        let line_zero = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(0.0, 0.0),
        );

        let line_one = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 0.0),
        );

        let line_neg = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(-1.0, 0.0),
        );

        let line_1_diag = LineSegment::from(
            Point::from(5.0, 5.0),
            Point::from(6.0, 6.0),
        );

        assert_eq!(line_zero.length(), 0.0);
        assert_eq!(line_one.length(), 1.0);
        assert_eq!(line_neg.length(), 1.0);
        assert_eq!(line_1_diag.length(), 2.0_f64.sqrt());
    }
    #[test]
    pub fn test_line_times_cross_ray() {
        let ray_h = LineSegment::from_ray(
            Point::from(0.0, 0.0),
            Point::from(1.0, 0.0),
        );

        let ray_v_before_touching = LineSegment::from_ray(
            Point::from(0.0, -1.0),
            Point::from(0.0, 0.0),
        );

        let ray_v_after_touching = LineSegment::from_ray(
            Point::from(0.0, 0.0),
            Point::from(0.0, 1.0),
        );

        assert_eq!(ray_h.times_cross_line(&ray_h), 0);
        assert_eq!(ray_h.times_cross_line(&ray_v_before_touching), 0);
        assert_eq!(ray_h.times_cross_line(&ray_v_after_touching), 1);

        assert_eq!(ray_v_before_touching.times_cross_line(&ray_h), 0);
        assert_eq!(ray_v_after_touching.times_cross_line(&ray_h), 1);

        assert_eq!(ray_v_before_touching.times_cross_line(&ray_v_after_touching), 0);
        assert_eq!(ray_v_after_touching.times_cross_line(&ray_v_before_touching), 0);
    }
    #[test]
    pub fn test_rect_join() {
        let r1 = Rectangle::from(
            Point::from(3.0, 4.0),
            Point::from(5.0, 7.0),
        );

        let r2 = Rectangle::from(
            Point::from(1.0, 8.0),
            Point::from(6.0, 1.0),
        );

        let joined = r1.join(&r2);

        assert_eq!(joined.start_point.x, 1.0);
        assert_eq!(joined.start_point.y, 1.0);
        assert_eq!(joined.end_point.x,   6.0);
        assert_eq!(joined.end_point.y,   8.0);
    }

    #[test]
    pub fn test_line_segment_intersection_y_up_down() {
        let line = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 1.0),
        );

        let next = LineSegment::from(
            Point::from(1.0, 1.0),
            Point::from(2.0, 0.0),
        );

        assert_eq!(Intersection::y(&line, &next, -0.1), vec![]);
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![(0.0, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![(0.1, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![(0.5, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![(0.9, true)]);
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![]);
        assert_eq!(Intersection::y(&line, &next, 1.1), vec![]);
    }

    #[test]
    pub fn test_line_segment_intersection_y_down_up() {
        let line = LineSegment::from(
            Point::from(0.0, 1.0),
            Point::from(1.0, 0.0),
        );

        let next = LineSegment::from(
            Point::from(1.0, 0.0),
            Point::from(2.0, 1.0),
        );

        assert_eq!(Intersection::y(&line, &next, -0.1), vec![]);
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![(1.0, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![(0.9, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![(0.5, true)]);
        // assert_eq!(Intersection::y(&line, &next, 0.9), vec![(0.1, true)]);
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![]);
        assert_eq!(Intersection::y(&line, &next, 1.1), vec![]);
    }

    #[test]
    pub fn test_line_segment_intersection_up_up() {
        let line = LineSegment::from_ray(
            Point::from(0.0, 0.0),
            Point::from(1.0, 1.0),
        );

        let next = LineSegment::from_ray(
            Point::from(1.0, 1.0),
            Point::from(2.0, 2.0),
        );

        assert_eq!(Intersection::y(&line, &next, -0.1), vec![]);
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![(0.0, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![(0.1, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![(0.5, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![(0.9, true)]);
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![]);
        assert_eq!(Intersection::y(&line, &next, 1.1), vec![]);
    }

    #[test]
    pub fn test_line_segment_intersection_leftside() {
        let line = LineSegment::from_ray(
            Point::from(0.0, 0.0),
            Point::from(1.0, 1.0),
        );

        let next = LineSegment::from_ray(
            Point::from(1.0, 1.0),
            Point::from(0.0, 2.0),
        );

        assert_eq!(Intersection::y(&line, &next, -0.1), vec![]);
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![(0.0, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![(0.1, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![(0.5, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![(0.9, true)]);
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![(1.0, true)]);
        assert_eq!(Intersection::y(&line, &next, 1.1), vec![]);
    }

    #[test]
    pub fn test_line_segment_intersection_rightside() {
        let line = LineSegment::from_ray(
            Point::from(1.0, 1.0),
            Point::from(0.0, 0.0),
        );

        let next = LineSegment::from_ray(
            Point::from(0.0, 0.0),
            Point::from(1.0, -1.0),
        );

        assert_eq!(Intersection::y(&line, &next, -0.1), vec![]);
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![(0.0, true)]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![(0.1, false)]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![(0.5, false)]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![(0.9, false)]);
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![(1.0, false)]);
        assert_eq!(Intersection::y(&line, &next, 1.1), vec![]);
    }


    #[test]
    pub fn test_line_segment_intersection_straight_up_right() {
        let line = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(0.0, 1.0),
        );

        let next = LineSegment::from(
            Point::from(0.0, 1.0),
            Point::from(2.0, 1.0),
        );

        assert_eq!(Intersection::y(&line, &next, -0.1), vec![]);
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![(1e-5, true), (0.99999, false)]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![]);
    }

    #[test]
    pub fn test_line_segment_intersection_left_up_90degrees() {
        let line = LineSegment::from(
            Point::from(1.0, 0.0),
            Point::from(0.0, 0.0),
        );

        let next = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(0.0, 1.0),
        );

        assert_eq!(Intersection::y(&line, &next, 1.1), vec![]);
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![(0.0, false)]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![(0.0, false)]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![(0.0, false)]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![(0.0, false)]);
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![]);
        assert_eq!(Intersection::y(&line, &next, -0.1), vec![]);
    }

    #[test]
    pub fn test_circle_distance_center1() {
        let circle = Circle {
            center: Point::from(0.0, 0.0),
            radius: 1.0,
        };

        let line1 = LineSegment::from(
            Point::from(-1.0, 1.0),
            Point::from(1.0, 1.0),
        );

        let line2 = LineSegment::from(
            Point::from(1.0, 2.0),
            Point::from(1.0, 1.0),
        );

        let line3 = LineSegment::from(
            Point::from(1.0, -2.0),
            Point::from(1.0, -1.0),
        );

        assert!(test_float(circle.distance_to_center(&line1), 1.0));
        assert!(test_float(circle.distance_to_center(&line2), 2.0_f64.sqrt()));
        assert!(test_float(circle.distance_to_center(&line3), 2.0_f64.sqrt()));
    }

    #[test]
    pub fn test_circle_distance_center2() {
        let circle = Circle {
            center: Point::from(5.0, 5.0),
            radius: 1.0,
        };

        let line1 = LineSegment::from(
            Point::from(4.0, 6.0),
            Point::from(6.0, 6.0),
        );

        let line2 = LineSegment::from(
            Point::from(6.0, 8.0),
            Point::from(6.0, 6.0),
        );

        let line3 = LineSegment::from(
            Point::from(6.0, 4.0),
            Point::from(6.0, 0.0),
        );

        let line4 = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(0.0, 0.0),
        );

        let line5 = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(5.0, 5.0),
        );

        let line6 = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(9.0, 9.0),
        );

        let line7 = LineSegment::from(
            Point::from(-1.0, 0.0),
            Point::from(-4.0, 1.0),
        );

        assert!(test_float(circle.distance_to_center(&line1), 1.0));
        assert!(test_float(circle.distance_to_center(&line2), 2.0_f64.sqrt()));
        assert!(test_float(circle.distance_to_center(&line3), 2.0_f64.sqrt()));
        assert!(test_float(circle.distance_to_center(&line4), 50.0_f64.sqrt()));
        assert!(test_float(circle.distance_to_center(&line5), 0.0));
        assert!(test_float(circle.distance_to_center(&line6), 0.0));
        assert!(test_float(circle.distance_to_center(&line7), (25.0 + 36.0 as f64).sqrt()));
    }

    #[test]
    pub fn test_point_of_interception_endless_lines() {
        let line1 = LineSegment::from(
            Point::from(0.0, 0.0),
            Point::from(1.0, 1.0),
        );
        let line2 = LineSegment::from(
            Point::from(0.0, 1.0),
            Point::from(1.0, 0.0),
        );
        let line_up = LineSegment::from(
            Point::from(0.0, 0.1),
            Point::from(0.0, 1.0),
        );
        let line_right = LineSegment::from(
            Point::from(0.1, 0.0),
            Point::from(1.0, 0.0),
        );

        assert_eq!(
            line1.point_of_interception_endless_lines(&line2),
            Some(Point::from(0.5, 0.5))
        );

        assert_eq!(
            line1.point_of_interception_endless_lines(&line_up),
            Some(Point::from(0.0, 0.0))
        );

        assert_eq!(
            line1.point_of_interception_endless_lines(&line_right),
            Some(Point::from(0.0, 0.0))
        );

        assert_eq!(
            line_up.point_of_interception_endless_lines(&line_right),
            Some(Point::from(0.0, 0.0))
        );
    }

    #[test]
    pub fn test_all_intersections_queue_item() {
        let top0 = AllIntersectionsQueueItem {
            point: Point::from(0.0, 0.0),
            point_position: AllIntersectionsQueueItemPointType::Top,
            line_segment_index: 0,
        };

        let bot0 = AllIntersectionsQueueItem {
            point: Point::from(1.0, 1.0),
            point_position: AllIntersectionsQueueItemPointType::Bottom,
            line_segment_index: 0,
        };

        let top1 = AllIntersectionsQueueItem {
            point: Point::from(1.0, 0.0),
            point_position: AllIntersectionsQueueItemPointType::Top,
            line_segment_index: 1,
        };

        let bot1 = AllIntersectionsQueueItem {
            point: Point::from(0.0, 1.0),
            point_position: AllIntersectionsQueueItemPointType::Bottom,
            line_segment_index: 1,
        };

        assert!(top0 < bot0);
        assert!(top1 < bot1);
        assert!(top0 < top1);
        assert!(bot1 < bot0);
    }

    #[test]
    pub fn test_all_intersections_tree_node() {
        let lines = vec![
            LineSegment::from(
                Point::from(0.0, 0.0),
                Point::from(1.0, 1.0),
            ),
            LineSegment::from(
                Point::from(0.0, 0.0),
                Point::from(1.0, 1.0),
            ),
        ];

        let node1 = AllIntersectionsTreeNode {
            line: &lines[0],
            index: 0,
            latest_x: lines[0].p1.x,
            latest_y: 0.0,
        };
        let node2 = AllIntersectionsTreeNode {
            line: &lines[0],
            index: 9,
            latest_x: lines[0].p1.x,
            latest_y: 0.0,
        };
        let node3 = AllIntersectionsTreeNode {
            line: &lines[0],
            index: 0,
            latest_x: lines[0].p1.x,
            latest_y: 1.0,
        };
        let node4 = AllIntersectionsTreeNode {
            line: &lines[1],
            index: 0,
            latest_x: lines[0].p1.x,
            latest_y: 0.0,
        };
        let node5 = AllIntersectionsTreeNode {
            line: &lines[1],
            index: 1,
            latest_x: lines[0].p1.x,
            latest_y: 0.0,
        };
        let node6 = AllIntersectionsTreeNode {
            line: &lines[1],
            index: 1,
            latest_x: lines[0].p1.x,
            latest_y: 1.0,
        };

        assert_eq!(node1, node3);
        assert_eq!(node1, node4);
        assert_eq!(node3, node4);
        assert_eq!(node5, node6);

        assert_ne!(node1, node2);
        assert_ne!(node1, node5);
        assert_ne!(node1, node6);

        assert_ne!(node5, node1);
        assert_ne!(node5, node2);
        assert_ne!(node5, node3);
        assert_ne!(node5, node4);
    }

    #[test]
    pub fn test_all_intersections_tree_node_example1() {
        let lines = vec![
            LineSegment::from(
                Point::from(0.0, 5.0),
                Point::from(2.0, 5.01),
            ),
            LineSegment::from(
                Point::from(1.0, 0.0),
                Point::from(1.0, 10.0),
            ),
        ];

        let point_intersection = lines[0].point_of_interception(&lines[1]).unwrap();

        let node1 = AllIntersectionsTreeNode {
            line: &lines[0],
            index: 0,
            latest_x: point_intersection.x + 0.1,
            latest_y: point_intersection.y - 0.1,
        };
        let node2 = AllIntersectionsTreeNode {
            line: &lines[1],
            index: 1,
            latest_x: point_intersection.x,
            latest_y: point_intersection.y,
        };

        assert!(node2 < node1);
    }

    #[test]
    pub fn test_all_intersections_tree_node_example2() {
        let lines = vec![
            LineSegment {
                p1: Point { x: 5.125, y: 5.135626562490234 },
                p2: Point { x: 3.125, y: 5.125626562490234 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 3.875, y: 2.374373437509766 },
                p2: Point { x: 3.875, y: 5.624376562490234 },
                includes_first_point: true, includes_second_point: false
            }
        ];

        let point_intersection = lines[0].point_of_interception(&lines[1]).unwrap();

        let node1 = AllIntersectionsTreeNode {
            line: &lines[0],
            index: 0,
            latest_x: point_intersection.x,
            latest_y: point_intersection.y,
        };
        let node2 = AllIntersectionsTreeNode {
            line: &lines[1],
            index: 1,
            latest_x: point_intersection.x,
            latest_y: point_intersection.y,
        };

        assert!(node2 < node1);
    }

    fn output_diff(
        expected: &Vec<(Point, usize, usize)>,
        actual  : &Vec<(Point, usize, usize)>,
    ) {
        let mut min = std::cmp::min(expected.len(), actual.len());
        for i in 0..min {
            println!("\t{:?} \t--- \t{:?}", expected[i], actual[i]);
        }
        for i in min..expected.len() {
            println!("\t{:?}  \t--- ", expected[i]);
        }
        for i in min..actual.len() {
            println!("\t\t\t\t\t  \t--- \t{:?}", actual[i]);
        }
    }

    fn compare_output_all_intersections(
        expected: &Vec<(Point, usize, usize)>,
        actual  : &Vec<(Point, usize, usize)>,
    ) {
        let mut expected = expected.clone();
        let mut actual = actual.clone();
        let sort_method = |l : &(Point, usize, usize), r : &(Point, usize, usize)| {
            if l.1 < r.1 {
                std::cmp::Ordering::Less
            } else if l.1 > r.1 {
                std::cmp::Ordering::Greater
            }
            else if l.2 < r.2 {
                std::cmp::Ordering::Less
            } else if l.2 > r.2 {
                std::cmp::Ordering::Greater
            }
            else if l.0 < r.0 {
                std::cmp::Ordering::Less
            } else if l.0 > r.0 {
                std::cmp::Ordering::Greater
            }
            else {
                std::cmp::Ordering::Equal
            }
        };
        expected.sort_by(sort_method);
        actual.sort_by(sort_method);
        if expected.len() != actual.len() {
            println!("Wrong len");
            output_diff(&expected, &actual);
            assert_eq!(expected, actual);
        }
        for i in 0..expected.len() {
            if !test_point(expected[i].0, actual[i].0) ||
                expected[i].1 != actual[i].1 ||
                expected[i].2 != actual[i].2 {
                output_diff(&expected, &actual);
                println!("{:?} != {:?}", expected[i], actual[i]);
                assert_eq!(expected, actual);
            }
        }
    }

    #[test]
    pub fn test_all_intersections_basic() {
        let mut v = std::collections::BTreeSet::new();
        for x in vec![0,3,4,5,6,7,8,9,11,24,56,77,89] {
            v.insert(x);
        }

        let lines = vec![
            LineSegment::from(
                Point::from(0.0, 0.0),
                Point::from(1.0, 1.0),
            ),
            LineSegment::from(
                Point::from(0.0, 1.0),
                Point::from(1.0, 0.0),
            ),
        ];

        assert_eq!(
            LineSegment::all_intersections(&lines),
            vec![
                (
                    Point::from(0.5, 0.5),
                    0, 1
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_correct_position_in_tree() {
        let lines = vec![
            LineSegment::from(
                Point::from(5.0, 5.0),
                Point::from(0.0, 0.0),
            ), // 1x
            LineSegment::from(
                Point::from(6.0, 4.0),
                Point::from(1.0, 1.0),
            ), // 3/5x + 2/5
            LineSegment::from(
                Point::from(4.0, 3.0),
                Point::from(9.0, 2.0),
            ), // -1/5x + 3+4/5
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(1.0, 1.0),
                    0, 1
                ),
                (
                    Point::from(4.25, 2.95),
                    1, 2
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_tops1() {
        let lines = vec![
            LineSegment::from(
                Point::from(0.0, 5.0),
                Point::from(3.0, 0.0),
            ), // 1x
            LineSegment::from(
                Point::from(0.0, 5.0),
                Point::from(-3.0, 0.0),
            ), // 1x
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(0.0, 5.0),
                    0, 1
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_tops2() {
        let lines = vec![
            LineSegment::from(
                Point::from(0.0, 5.0),
                Point::from(-3.0, 0.0),
            ),
            LineSegment::from(
                Point::from(0.0, 5.0),
                Point::from(3.0, 0.0),
            ),
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(0.0, 5.0),
                    0, 1
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_horizontal() {
        let lines = vec![
            LineSegment::from_ray(
                Point::from(0.0, 0.0),
                Point::from(1.0, 0.0),
            ),
            LineSegment::from_ray(
                Point::from(1.0, 0.0),
                Point::from(2.0, 0.0),
            ),
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(1.0, 0.0),
                    0, 1
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_vertical() {
        let lines = vec![
            LineSegment::from_ray(
                Point::from(0.0, 0.0),
                Point::from(0.0, 1.0),
            ),
            LineSegment::from_ray(
                Point::from(0.0, 1.0),
                Point::from(0.0, 2.0),
            ),
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(0.0, 1.0),
                    0, 1
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example1() {
        let lines = vec![
            LineSegment::from_ray(
                Point::from(2.0, 2.0),
                Point::from(6.0, 2.0),
            ),
            LineSegment::from_ray(
                Point::from(6.0, 2.0),
                Point::from(4.0, 6.0),
            ),
            LineSegment::from_ray(
                Point::from(4.0, 6.0),
                Point::from(2.0, 2.0),
            ),
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(2.0, 2.0),
                    0, 2
                ),
                (
                    Point::from(6.0, 2.0),
                    0, 1
                ),
                (
                    Point::from(4.0, 6.0),
                    1, 2
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example2() {
        let lines = vec![
            LineSegment::from_ray(
                Point::from(2.0, 2.0),
                Point::from(6.0, 2.0),
            ),
            LineSegment::from_ray(
                Point::from(6.0, 2.0),
                Point::from(4.0, 4.0),
            ),
            LineSegment::from_ray(
                Point::from(4.0, 4.0),
                Point::from(2.0, 2.0),
            ),
            LineSegment::from_ray(
                Point::from(2.0, 2.0),
                Point::from(2.0, 6.0),
            ),
            LineSegment::from_ray(
                Point::from(2.0, 6.0),
                Point::from(6.0, 6.0),
            ),
            LineSegment::from_ray(
                Point::from(6.0, 6.0),
                Point::from(4.0, 4.0),
            ),
            LineSegment::from_ray(
                Point::from(4.0, 4.0),
                Point::from(2.0, 6.0),
            ),
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(2.0, 2.0),
                    0, 2
                ),
                (
                    Point::from(2.0, 2.0),
                    2, 3
                ),
                (
                    Point::from(2.0, 2.0),
                    0, 3
                ),
                (
                    Point::from(6.0, 2.0),
                    0, 1
                ),
                (
                    Point::from(4.0, 4.0),
                    1, 2
                ),
                (
                    Point::from(4.0, 4.0),
                    1, 5
                ),
                (
                    Point::from(4.0, 4.0),
                    2, 5
                ),
                (
                    Point::from(4.0, 4.0),
                    2, 6
                ),
                (
                    Point::from(4.0, 4.0),
                    5, 6
                ),
                (
                    Point::from(4.0, 4.0),
                    1, 6
                ),
                (
                    Point::from(2.0, 6.0),
                    3, 6
                ),
                (
                    Point::from(6.0, 6.0),
                    4, 5
                ),
                (
                    Point::from(2.0, 6.0),
                    4, 6
                ),
                (
                    Point::from(2.0, 6.0),
                    3, 4
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example3() {
        let lines = vec![
            LineSegment::from_ray(
                Point::from(2.0, 2.0),
                Point::from(6.0, 2.0),
            ),
            LineSegment::from_ray(
                Point::from(6.0, 2.0),
                Point::from(4.0, 4.0),
            ),
            LineSegment::from_ray(
                Point::from(2.0, 2.0),
                Point::from(2.0, 6.0),
            ),
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(2.0, 2.0),
                    0, 2
                ),
                (
                    Point::from(6.0, 2.0),
                    0, 1
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example4() {
        let lines = vec![
            LineSegment::from_ray( // 0
                Point::from(2.0, 2.0),
                Point::from(6.0, 2.0),
            ),
            LineSegment::from_ray( // 1
                Point::from(6.0, 2.0),
                Point::from(4.0, 4.0),
            ),
            LineSegment::from_ray( // 2
                Point::from(4.0, 4.0),
                Point::from(2.0, 2.0),
            ),
            LineSegment::from_ray( // 3
                Point::from(2.0, 6.0),
                Point::from(6.0, 6.0),
            ),
            LineSegment::from_ray( // 4
                Point::from(6.0, 6.0),
                Point::from(4.0, 4.0),
            ),
            LineSegment::from_ray( // 5
                Point::from(4.0, 4.0),
                Point::from(2.0, 6.0),
            ),
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(2.0, 2.0),
                    0, 2
                ),
                (
                    Point::from(2.0, 6.0),
                    3, 5
                ),
                (
                    Point::from(4.0, 4.0),
                    1, 2
                ),
                (
                    Point::from(4.0, 4.0),
                    1, 4
                ),
                (
                    Point::from(4.0, 4.0),
                    1, 5
                ),
                (
                    Point::from(4.0, 4.0),
                    2, 4
                ),
                (
                    Point::from(4.0, 4.0),
                    4, 5
                ),
                (
                    Point::from(4.0, 4.0),
                    2, 5
                ),
                (
                    Point::from(6.0, 2.0),
                    0, 1
                ),
                (
                    Point::from(6.0, 6.0),
                    3, 4
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example5() {
        let lines = vec![
            LineSegment::from_ray(
                Point::from(6.0, 2.0),
                Point::from(4.0, 4.0),
            ), // slope -1
            LineSegment::from_ray(
                Point::from(4.0, 4.0),
                Point::from(2.0, 2.0),
            ), // slope 1
            LineSegment::from_ray(
                Point::from(4.0, 4.0),
                Point::from(2.0, 6.0),
            ), // slope -1
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(4.0, 4.0),
                    0, 1
                ),
                (
                    Point::from(4.0, 4.0),
                    1, 2
                ),
                (
                    Point::from(4.0, 4.0),
                    0, 2
                ),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example6() {
        let lines = vec![
            LineSegment {
                p1: Point { x: 1.6869327075771883, y: 1.875 },
                p2: Point { x: 6.313067292422812, y: 1.875 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 6.313067292422812, y: 1.875 },
                p2: Point { x: 4.000000000000001, y: 4.072413927801671 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 4.000000000000001, y: 4.072413927801671 },
                p2: Point { x: 1.6869327075771883, y: 1.875 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 1.6869327075771887, y: 6.125 },
                p2: Point { x: 6.313067292422812, y: 6.125 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 6.313067292422812, y: 6.125 },
                p2: Point { x: 4.000000000000001, y: 3.927586072198329 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 4.000000000000001, y: 3.927586072198329 },
                p2: Point { x: 1.6869327075771887, y: 6.125 },
                includes_first_point: true, includes_second_point: false
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 1.6869327075771885, y: 1.875 }, 0, 2),
                (Point { x: 1.6869327075771885, y: 6.125 }, 3, 5),
                (Point { x: 3.923774812840347, y: 4.0 }, 2, 5),
                (Point { x: 4.000000000000001, y: 3.9275860721983293 }, 4, 5),
                (Point { x: 4.000000000000001, y: 4.072413927801671 }, 1, 2),
                (Point { x: 4.076225187159654, y: 3.9999999999999996 }, 1, 4),
                (Point { x: 6.313067292422812, y: 1.875 }, 0, 1),
                (Point { x: 6.313067292422812, y: 6.125 }, 3, 4),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example6_1() {
        let lines = vec![
            LineSegment { // 0 bottom right
                p1: Point { x: 6.313067292422812, y: 1.875 },
                p2: Point { x: 4.000000000000001, y: 4.072413927801671 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment { // 1 bottom left
                p1: Point { x: 4.000000000000001, y: 4.072413927801671 },
                p2: Point { x: 1.6869327075771883, y: 1.875 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment { // 2 top right
                p1: Point { x: 6.313067292422812, y: 6.125 },
                p2: Point { x: 4.000000000000001, y: 3.927586072198329 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment { // 3 top left
                p1: Point { x: 4.000000000000001, y: 3.927586072198329 },
                p2: Point { x: 1.6869327075771887, y: 6.125 },
                includes_first_point: true, includes_second_point: false
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 3.923774812840347, y: 4.0 }, 1, 3),
                (Point { x: 4.000000000000001, y: 3.927586072198329 }, 2, 3),
                (Point { x: 4.000000000000001, y: 4.072413927801671 }, 0, 1),
                (Point { x: 4.076225187159654, y: 3.9999999999999996 }, 0, 2),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example6_2() {
        let lines = vec![
            LineSegment { // 0 bottom
                p1: Point { x: 1.6869327075771883, y: 1.875 },
                p2: Point { x: 6.313067292422812, y: 1.875 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment { // 1 bottom right
                p1: Point { x: 6.313067292422812, y: 1.875 },
                p2: Point { x: 4.000000000000001, y: 4.072413927801671 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment { // 2 bottom left
                p1: Point { x: 4.000000000000001, y: 4.072413927801671 },
                p2: Point { x: 1.6869327075771883, y: 1.875 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment { // 3 top right
                p1: Point { x: 6.313067292422812, y: 6.125 },
                p2: Point { x: 4.000000000000001, y: 3.927586072198329 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment { // 4 top left
                p1: Point { x: 4.000000000000001, y: 3.927586072198329 },
                p2: Point { x: 1.6869327075771887, y: 6.125 },
                includes_first_point: true, includes_second_point: false
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 1.6869327075771885, y: 1.875 }, 0, 2),
                (Point { x: 4.000000000000001, y: 4.072413927801671 }, 1, 2),
                (Point { x: 6.313067292422812, y: 1.875 }, 0, 1),
                (Point { x: 4.000000000000001, y: 3.927586072198329 }, 3, 4),
                (Point { x: 4.076225187159654, y: 4.0 }, 1, 3),
                (Point { x: 3.923774812840347, y: 4.0 }, 2, 4),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example7() {
        let lines = vec![
            LineSegment {
                p1: Point { x: 6.313067292422812, y: 1.875 },
                p2: Point { x: 4.000000000000001, y: 4.072413927801671 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 4.000000000000001, y: 4.072413927801671 },
                p2: Point { x: 1.6869327075771883, y: 1.875 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 6.313067292422812, y: 6.125 },
                p2: Point { x: 4.000000000000001, y: 3.927586072198329 },
                includes_first_point: true, includes_second_point: false
            },
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point { x: 4.000000000000001, y: 4.072413927801671 },
                    0, 1
                ),
                (
                    Point { x: 4.076225187159654, y: 3.9999999999999996 },
                    0, 2
                )
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example8() {
        let lines = vec![
            LineSegment { // 0
                p1: Point { x: 5.0, y: 6.0 },
                p2: Point { x: 5.0, y: 5.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 1
                p1: Point { x: 5.0, y: 5.0 },
                p2: Point { x: 3.0, y: 5.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 2
                p1: Point { x: 3.0, y: 5.0 },
                p2: Point { x: 3.0, y: 3.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 3
                p1: Point { x: 3.0, y: 3.0 },
                p2: Point { x: 5.0, y: 3.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 4
                p1: Point { x: 5.0, y: 3.0 },
                p2: Point { x: 5.0, y: 2.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 5
                p1: Point { x: 5.0, y: 2.0 },
                p2: Point { x: 2.0, y: 2.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 6
                p1: Point { x: 2.0, y: 2.0 },
                p2: Point { x: 2.0, y: 6.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 7
                p1: Point { x: 2.0, y: 6.0 },
                p2: Point { x: 5.0, y: 6.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 8
                p1: Point { x: 4.0, y: 5.8 },
                p2: Point { x: 4.0, y: 5.4 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 9
                p1: Point { x: 4.0, y: 5.4 },
                p2: Point { x: 8.0, y: 5.4 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 10
                p1: Point { x: 8.0, y: 5.4 },
                p2: Point { x: 8.0, y: 2.8 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 11
                p1: Point { x: 8.0, y: 2.8 },
                p2: Point { x: 4.0, y: 2.8 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 12
                p1: Point { x: 4.0, y: 2.8 },
                p2: Point { x: 4.0, y: 2.4 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 13
                p1: Point { x: 4.0, y: 2.4 },
                p2: Point { x: 9.0, y: 2.4 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 14
                p1: Point { x: 9.0, y: 2.4 },
                p2: Point { x: 9.0, y: 5.8 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 15
                p1: Point { x: 9.0, y: 5.8 },
                p2: Point { x: 4.0, y: 5.8 },
                includes_first_point: true,
                includes_second_point: false
            },
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 2.0, y: 2.0 }, 5, 6),
                (Point { x: 2.0, y: 6.0 }, 6, 7),
                (Point { x: 3.0, y: 3.0 }, 2, 3),
                (Point { x: 3.0, y: 5.0 }, 1, 2),
                (Point { x: 4.0, y: 2.4 }, 12, 13),
                (Point { x: 4.0, y: 2.8 }, 11, 12),
                (Point { x: 4.0, y: 5.4 }, 8, 9),
                (Point { x: 4.0, y: 5.8 }, 8, 15),
                (Point { x: 5.0, y: 2.0 }, 4, 5),
                (Point { x: 5.0, y: 2.4 }, 4, 13),
                (Point { x: 5.0, y: 2.8 }, 4, 11),
                (Point { x: 5.0, y: 3.0 }, 3, 4),
                (Point { x: 5.0, y: 5.0 }, 0, 1),
                (Point { x: 5.0, y: 5.4 }, 0, 9),
                (Point { x: 5.0, y: 5.8 }, 0, 15),
                (Point { x: 5.0, y: 6.0 }, 0, 7),
                (Point { x: 8.0, y: 5.4 }, 9, 10),
                (Point { x: 9.0, y: 5.8 }, 14, 15),
                (Point { x: 9.0, y: 2.4 }, 13, 14),
                (Point { x: 8.0, y: 2.8 }, 10, 11),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example9() {
        let lines = vec![
            LineSegment { // 0
                p1: Point { x: 5.0, y: 3.0 },
                p2: Point { x: 5.0, y: 2.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 1
                p1: Point { x: 4.0, y: 2.4 },
                p2: Point { x: 9.0, y: 2.4 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 2
                p1: Point { x: 9.0, y: 2.4 },
                p2: Point { x: 9.0, y: 5.8 },
                includes_first_point: true,
                includes_second_point: false
            },
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 5.0, y: 2.4 }, 0, 1),
                (Point { x: 9.0, y: 2.4 }, 1, 2),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example10() {
        let lines = vec![
            LineSegment { // 0
                p1: Point { x: 5.0, y: 6.0 },
                p2: Point { x: 5.0, y: 5.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 1
                p1: Point { x: 5.0, y: 3.0 },
                p2: Point { x: 5.0, y: 2.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 2
                p1: Point { x: 5.0, y: 2.0 },
                p2: Point { x: 2.0, y: 2.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 3
                p1: Point { x: 4.0, y: 5.8 },
                p2: Point { x: 4.0, y: 5.4 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 4
                p1: Point { x: 4.0, y: 5.4 },
                p2: Point { x: 8.0, y: 5.4 },
                includes_first_point: true,
                includes_second_point: false
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 4.0, y: 5.4 }, 3, 4),
                (Point { x: 5.0, y: 2.0 }, 1, 2),
                (Point { x: 5.0, y: 5.4 }, 0, 4),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example11() {
        let lines = vec![
            LineSegment { // 0
                p1: Point { x: 0.0, y: 0.0 },
                p2: Point { x: 4.0, y: 0.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 1
                p1: Point { x: 0.0, y: 1.0 },
                p2: Point { x: 4.0, y: 1.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 2
                p1: Point { x: 0.0, y: 2.0 },
                p2: Point { x: 4.0, y: 2.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 3
                p1: Point { x: 0.0, y: 3.0 },
                p2: Point { x: 4.0, y: 3.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 4
                p1: Point { x: 0.0, y: 4.0 },
                p2: Point { x: 4.0, y: 4.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 5
                p1: Point { x: 0.0, y: 0.0 },
                p2: Point { x: 0.0, y: 4.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 6
                p1: Point { x: 4.0, y: 0.0 },
                p2: Point { x: 4.0, y: 4.0 },
                includes_first_point: true,
                includes_second_point: false
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 0.0, y: 0.0 }, 0, 5),
                (Point { x: 0.0, y: 1.0 }, 1, 5),
                (Point { x: 0.0, y: 2.0 }, 2, 5),
                (Point { x: 0.0, y: 3.0 }, 3, 5),
                (Point { x: 0.0, y: 4.0 }, 4, 5),
                (Point { x: 4.0, y: 0.0 }, 0, 6),
                (Point { x: 4.0, y: 1.0 }, 1, 6),
                (Point { x: 4.0, y: 2.0 }, 2, 6),
                (Point { x: 4.0, y: 3.0 }, 3, 6),
                (Point { x: 4.0, y: 4.0 }, 4, 6),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example12() {
        let lines = vec![
            LineSegment { // 0
                p1: Point { x: 0.0, y: 0.0 },
                p2: Point { x: 0.0, y: 1.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 1
                p1: Point { x: 1.0, y: 0.0 },
                p2: Point { x: 1.0, y: 1.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 2
                p1: Point { x: 2.0, y: 0.0 },
                p2: Point { x: 2.0, y: 1.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 3
                p1: Point { x: 3.0, y: 0.0 },
                p2: Point { x: 3.0, y: 1.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 4
                p1: Point { x: 0.0, y: 0.0 },
                p2: Point { x: 3.0, y: 0.0 },
                includes_first_point: true,
                includes_second_point: false
            },
            LineSegment { // 5
                p1: Point { x: 0.0, y: 1.0 },
                p2: Point { x: 3.0, y: 1.0 },
                includes_first_point: true,
                includes_second_point: false
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 0.0, y: 0.0 }, 0, 4),
                (Point { x: 1.0, y: 0.0 }, 1, 4),
                (Point { x: 2.0, y: 0.0 }, 2, 4),
                (Point { x: 3.0, y: 0.0 }, 3, 4),
                (Point { x: 0.0, y: 1.0 }, 0, 5),
                (Point { x: 1.0, y: 1.0 }, 1, 5),
                (Point { x: 2.0, y: 1.0 }, 2, 5),
                (Point { x: 3.0, y: 1.0 }, 3, 5),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example13() {
        let lines = vec![
            LineSegment {
                p1: Point { x: 5.125, y: 6.1358361110802475 },
                p2: Point { x: 5.125, y: 5.135626562490234 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 5.125, y: 5.135626562490234 },
                p2: Point { x: 3.125, y: 5.125626562490234 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 3.125, y: 5.125626562490234 },
                p2: Point { x: 3.125, y: 2.875623437509766 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 3.125, y: 2.875623437509766 },
                p2: Point { x: 4.875, y: 2.884373437509766 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 4.875, y: 2.884373437509766 },
                p2: Point { x: 4.875, y: 1.8841638889197523 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 4.875, y: 1.8841638889197523 },
                p2: Point { x: 1.875, y: 1.8641638889197525 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 1.875, y: 1.8641638889197525 },
                p2: Point { x: 1.875, y: 6.114169444413582 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 1.875, y: 6.114169444413582 },
                p2: Point { x: 5.125, y: 6.1358361110802475 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 3.875, y: 5.624376562490234 },
                p2: Point { x: 6.125, y: 5.635626562490234 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 6.125, y: 5.635626562490234 },
                p2: Point { x: 6.125, y: 2.3856234375097656 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 6.125, y: 2.3856234375097656 },
                p2: Point { x: 3.875, y: 2.374373437509766 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 3.875, y: 2.374373437509766 },
                p2: Point { x: 3.875, y: 5.624376562490234 },
                includes_first_point: true, includes_second_point: false
            }
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 1.875, y: 1.8641638889197525 }, 5, 6),
                (Point { x: 1.875, y: 6.114169444413582 }, 6, 7),
                (Point { x: 3.125, y: 2.875623437509766 }, 2, 3),
                (Point { x: 3.125, y: 5.125626562490234 }, 1, 2),
                (Point { x: 3.875, y: 2.374373437509766 }, 10, 11),
                (Point { x: 3.875, y: 2.879373437509766 }, 3, 11),
                (Point { x: 3.875, y: 5.129376562490234 }, 1, 11),
                (Point { x: 3.875, y: 5.624376562490234 }, 8, 11),
                (Point { x: 4.875, y: 1.8841638889197523 }, 4, 5),
                (Point { x: 4.875, y: 2.379373437509766 }, 4, 10),
                (Point { x: 4.875, y: 2.884373437509766 }, 3, 4),
                (Point { x: 5.125, y: 5.135626562490234 }, 0, 1),
                (Point { x: 5.125, y: 5.630626562490233 }, 0, 8),
                (Point { x: 5.125, y: 6.1358361110802475 }, 0, 7),
                (Point { x: 6.125, y: 2.3856234375097656 }, 9, 10),
                (Point { x: 6.125, y: 5.635626562490234 }, 8, 9),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example14() {
        let lines = vec![
            LineSegment {
                p1: Point { x: 5.125, y: 6.1358361110802475 },
                p2: Point { x: 5.125, y: 5.135626562490234 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 5.125, y: 5.135626562490234 },
                p2: Point { x: 3.125, y: 5.125626562490234 },
                includes_first_point: true, includes_second_point: false
            },
            LineSegment {
                p1: Point { x: 3.875, y: 2.374373437509766 },
                p2: Point { x: 3.875, y: 5.624376562490234 },
                includes_first_point: true, includes_second_point: false
            }
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 5.125, y: 5.135626562490234 }, 0, 1),
                (Point { x: 3.875, y: 5.129376562490234 }, 1, 2),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example15() {
        let lines = vec![
            LineSegment {
                p1: Point { x: 2.200923442840576, y: 3.0 },
                p2: Point { x: 2.200923442840576, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 2.200923442840576, y: 5.355677127838135 },
                p2: Point { x: 1.375, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 1.375, y: 5.355677127838135 },
                p2: Point { x: 1.375, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 1.375, y: 5.875 },
                p2: Point { x: 3.5802841186523438, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 3.5802841186523438, y: 5.875 },
                p2: Point { x: 3.5802841186523438, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 3.5802841186523438, y: 5.355677127838135 },
                p2: Point { x: 2.7579517364501953, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 2.7579517364501953, y: 5.355677127838135 },
                p2: Point { x: 2.7579517364501953, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 2.7579517364501953, y: 3.0 },
                p2: Point { x: 2.200923442840576, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 3.727086067199707, y: 3.0 },
                p2: Point { x: 3.727086067199707, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 3.727086067199707, y: 5.875 },
                p2: Point { x: 5.438611030578613, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.438611030578613, y: 5.875 },
                p2: Point { x: 5.438611030578613, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.438611030578613, y: 5.355677127838135 },
                p2: Point { x: 4.282318115234375, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 4.282318115234375, y: 5.355677127838135 },
                p2: Point { x: 4.282318115234375, y: 4.756412506103516 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 4.282318115234375, y: 4.756412506103516 },
                p2: Point { x: 5.372178077697754, y: 4.756412506103516 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.372178077697754, y: 4.756412506103516 },
                p2: Point { x: 5.372178077697754, y: 4.240680694580078 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.372178077697754, y: 4.240680694580078 },
                p2: Point { x: 4.282318115234375, y: 4.240680694580078 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 4.282318115234375, y: 4.240680694580078 },
                p2: Point { x: 4.282318115234375, y: 3.5193228721618652 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 4.282318115234375, y: 3.5193228721618652 },
                p2: Point { x: 5.438611030578613, y: 3.5193228721618652 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.438611030578613, y: 3.5193228721618652 },
                p2: Point { x: 5.438611030578613, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.438611030578613, y: 3.0 },
                p2: Point { x: 3.727086067199707, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 7.231243872291176, y: 3.0 },
                p2: Point { x: 6.59119565394872, y: 4.043521440830737 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 6.59119565394872, y: 4.043521440830737 },
                p2: Point { x: 5.941425375931249, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.941425375931249, y: 3.0 },
                p2: Point { x: 5.3187876869681565, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.3187876869681565, y: 3.0 },
                p2: Point { x: 6.272807746257463, y: 4.492737852325395 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 6.272807746257463, y: 4.492737852325395 },
                p2: Point { x: 5.378286559813183, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 5.378286559813183, y: 5.875 },
                p2: Point { x: 6.016635762592474, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 6.016635762592474, y: 5.875 },
                p2: Point { x: 6.603059113306081, y: 4.938329019826689 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 6.603059113306081, y: 4.938329019826689 },
                p2: Point { x: 7.192695823314598, y: 5.874999999999999 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 7.192695823314598, y: 5.874999999999999 },
                p2: Point { x: 7.8149446973032, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 7.8149446973032, y: 5.875 },
                p2: Point { x: 6.92191888204544, y: 4.497921348517647 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 6.92191888204544, y: 4.497921348517647 },
                p2: Point { x: 7.8754980994305495, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 7.8754980994305495, y: 3.0 },
                p2: Point { x: 7.231243872291176, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 8.393552780151367, y: 3.0 },
                p2: Point { x: 8.393552780151367, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 8.393552780151367, y: 5.355677127838135 },
                p2: Point { x: 7.567629814147949, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 7.567629814147949, y: 5.355677127838135 },
                p2: Point { x: 7.567629814147949, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 7.567629814147949, y: 5.875 },
                p2: Point { x: 9.772912979125977, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 9.772912979125977, y: 5.875 },
                p2: Point { x: 9.772912979125977, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 9.772912979125977, y: 5.355677127838135 },
                p2: Point { x: 8.950581550598145, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 8.950581550598145, y: 5.355677127838135 },
                p2: Point { x: 8.950581550598145, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 8.950581550598145, y: 3.0 },
                p2: Point { x: 8.393552780151367, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        /*
        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 2.200923442840576, y: 5.355677127838135 }, 0, 1),
                (Point { x: 2.200923442840576, y: 3.0 }, 0, 7),
                (Point { x: 1.375, y: 5.355677127838135 }, 1, 2),
                (Point { x: 1.375, y: 5.875 }, 2, 3),
                (Point { x: 3.5802841186523438, y: 5.875 }, 3, 4),
                (Point { x: 3.5802841186523438, y: 5.355677127838135 }, 4, 5),
                (Point { x: 2.7579517364501953, y: 5.355677127838135 }, 5, 6),
                (Point { x: 2.7579517364501953, y: 3.0 }, 6, 7),
                (Point { x: 3.727086067199707, y: 5.875 }, 8, 9),
                (Point { x: 3.727086067199707, y: 3.0 }, 8, 19),
                (Point { x: 5.438611030578613, y: 5.875 }, 9, 10),
                (Point { x: 5.378286559813183, y: 5.875 }, 9, 24),
                (Point { x: 5.438611030578613, y: 5.875 }, 9, 25),
                (Point { x: 5.438611030578613, y: 5.355677127838135 }, 10, 11),
                (Point { x: 5.438611030578613, y: 5.781783390062552 }, 10, 24),
                (Point { x: 5.438611030578613, y: 5.875 }, 10, 25),
                (Point { x: 4.282318115234375, y: 5.355677127838135 }, 11, 12),
                (Point { x: 4.282318115234375, y: 4.756412506103516 }, 12, 13),
                (Point { x: 5.372178077697754, y: 4.756412506103516 }, 13, 14),
                (Point { x: 5.372178077697754, y: 4.240680694580078 }, 14, 15),
                (Point { x: 4.282318115234375, y: 4.240680694580078 }, 15, 16),
                (Point { x: 4.282318115234375, y: 3.5193228721618652 }, 16, 17),
                (Point { x: 5.438611030578613, y: 3.5193228721618652 }, 17, 18),
                (Point { x: 5.438611030578613, y: 3.0 }, 18, 19),
                (Point { x: 5.438611030578613, y: 3.0 }, 18, 22),
                (Point { x: 5.438611030578613, y: 3.1874854085696747 }, 18, 23),
                (Point { x: 5.438611030578613, y: 3.0 }, 19, 22),
                (Point { x: 5.3187876869681565, y: 3.0 }, 19, 23),
                (Point { x: 6.59119565394872, y: 4.043521440830737 }, 20, 21),
                (Point { x: 7.231243872291176, y: 3.0 }, 20, 31),
                (Point { x: 5.941425375931249, y: 3.0 }, 21, 22),
                (Point { x: 5.3187876869681565, y: 3.0 }, 22, 23),
                (Point { x: 6.272807746257463, y: 4.492737852325395 }, 23, 24),
                (Point { x: 5.378286559813183, y: 5.875 }, 24, 25),
                (Point { x: 6.016635762592474, y: 5.875 }, 25, 26),
                (Point { x: 6.603059113306081, y: 4.938329019826689 }, 26, 27),
                (Point { x: 7.192695823314598, y: 5.874999999999999 }, 27, 28),
                (Point { x: 7.8149446973032, y: 5.875 }, 28, 29),
                (Point { x: 7.567629814147949, y: 5.874999999999999 }, 28, 34),
                (Point { x: 7.8149446973032, y: 5.875 }, 28, 35),
                (Point { x: 6.92191888204544, y: 4.497921348517647 }, 29, 30),
                (Point { x: 7.567629814147949, y: 5.493631354247416 }, 29, 34),
                (Point { x: 7.8149446973032, y: 5.875 }, 29, 35),
                (Point { x: 7.8754980994305495, y: 3.0 }, 30, 31),
                (Point { x: 8.393552780151367, y: 5.355677127838135 }, 32, 33),
                (Point { x: 8.393552780151367, y: 3.0 }, 32, 39),
                (Point { x: 7.567629814147949, y: 5.355677127838135 }, 33, 34),
                (Point { x: 7.567629814147949, y: 5.875 }, 34, 35),
                (Point { x: 9.772912979125977, y: 5.875 }, 35, 36),
                (Point { x: 9.772912979125977, y: 5.355677127838135 }, 36, 37),
                (Point { x: 8.950581550598145, y: 5.355677127838135 }, 37, 38),
                (Point { x: 8.950581550598145, y: 3.0 }, 38, 39),
            ],
        );
        */
    }

    #[test]
    pub fn test_all_intersections_touching_example16() {
        let lines = vec![
            LineSegment {
                p1: Point { x: 7.192695823314598, y: 5.874999999999999 },
                p2: Point { x: 7.8149446973032, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 7.567629814147949, y: 5.875 },
                p2: Point { x: 9.772912979125977, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment {
                p1: Point { x: 9.772912979125977, y: 5.875 },
                p2: Point { x: 9.772912979125977, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        /*
        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 7.8149446973032, y: 5.875 }, 0, 1),
                (Point { x: 9.772912979125977, y: 5.875 }, 1, 2),
            ],
        );
        */
    }

    #[test]
    pub fn test_all_intersections_touching_example17() {
        // (Point { x: 7.567629814147949, y: 5.875 }, 28, 35)
        let lines = vec![
            LineSegment { // 0
                p1: Point { x: 2.200923442840576, y: 3.0 },
                p2: Point { x: 2.200923442840576, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 1
                p1: Point { x: 2.200923442840576, y: 5.355677127838135 },
                p2: Point { x: 1.375, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 2
                p1: Point { x: 1.375, y: 5.355677127838135 },
                p2: Point { x: 1.375, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 3
                p1: Point { x: 1.375, y: 5.875 },
                p2: Point { x: 3.5802841186523438, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 4
                p1: Point { x: 3.5802841186523438, y: 5.875 },
                p2: Point { x: 3.5802841186523438, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 5
                p1: Point { x: 3.5802841186523438, y: 5.355677127838135 },
                p2: Point { x: 2.7579517364501953, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 6
                p1: Point { x: 2.7579517364501953, y: 5.355677127838135 },
                p2: Point { x: 2.7579517364501953, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 7
                p1: Point { x: 2.7579517364501953, y: 3.0 },
                p2: Point { x: 2.200923442840576, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 8
                p1: Point { x: 3.727086067199707, y: 3.0 },
                p2: Point { x: 3.727086067199707, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 9
                p1: Point { x: 3.727086067199707, y: 5.875 },
                p2: Point { x: 5.438611030578613, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 10
                p1: Point { x: 5.438611030578613, y: 5.875 },
                p2: Point { x: 5.438611030578613, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 11
                p1: Point { x: 5.438611030578613, y: 5.355677127838135 },
                p2: Point { x: 4.282318115234375, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 12
                p1: Point { x: 4.282318115234375, y: 5.355677127838135 },
                p2: Point { x: 4.282318115234375, y: 4.756412506103516 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 13
                p1: Point { x: 4.282318115234375, y: 4.756412506103516 },
                p2: Point { x: 5.372178077697754, y: 4.756412506103516 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 14
                p1: Point { x: 5.372178077697754, y: 4.756412506103516 },
                p2: Point { x: 5.372178077697754, y: 4.240680694580078 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 15
                p1: Point { x: 5.372178077697754, y: 4.240680694580078 },
                p2: Point { x: 4.282318115234375, y: 4.240680694580078 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 16
                p1: Point { x: 4.282318115234375, y: 4.240680694580078 },
                p2: Point { x: 4.282318115234375, y: 3.5193228721618652 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 17
                p1: Point { x: 4.282318115234375, y: 3.5193228721618652 },
                p2: Point { x: 5.438611030578613, y: 3.5193228721618652 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 18
                p1: Point { x: 5.438611030578613, y: 3.5193228721618652 },
                p2: Point { x: 5.438611030578613, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 19
                p1: Point { x: 5.438611030578613, y: 3.0 },
                p2: Point { x: 3.727086067199707, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 20
                p1: Point { x: 7.231243872291176, y: 3.0 },
                p2: Point { x: 6.59119565394872, y: 4.043521440830737 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 21
                p1: Point { x: 6.59119565394872, y: 4.043521440830737 },
                p2: Point { x: 5.941425375931249, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 22
                p1: Point { x: 5.941425375931249, y: 3.0 },
                p2: Point { x: 5.3187876869681565, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 23
                p1: Point { x: 5.3187876869681565, y: 3.0 },
                p2: Point { x: 6.272807746257463, y: 4.492737852325395 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 24
                p1: Point { x: 6.272807746257463, y: 4.492737852325395 },
                p2: Point { x: 5.378286559813183, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 25
                p1: Point { x: 5.378286559813183, y: 5.875 },
                p2: Point { x: 6.016635762592474, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 26
                p1: Point { x: 6.016635762592474, y: 5.875 },
                p2: Point { x: 6.603059113306081, y: 4.938329019826689 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 27
                p1: Point { x: 6.603059113306081, y: 4.938329019826689 },
                p2: Point { x: 7.192695823314598, y: 5.874999999999999 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 28
                p1: Point { x: 7.192695823314598, y: 5.874999999999999 },
                p2: Point { x: 7.8149446973032, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 29
                p1: Point { x: 7.8149446973032, y: 5.875 },
                p2: Point { x: 6.92191888204544, y: 4.497921348517647 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 30
                p1: Point { x: 6.92191888204544, y: 4.497921348517647 },
                p2: Point { x: 7.8754980994305495, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 31
                p1: Point { x: 7.8754980994305495, y: 3.0 },
                p2: Point { x: 7.231243872291176, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 32
                p1: Point { x: 8.393552780151367, y: 3.0 },
                p2: Point { x: 8.393552780151367, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 33
                p1: Point { x: 8.393552780151367, y: 5.355677127838135 },
                p2: Point { x: 7.567629814147949, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 34
                p1: Point { x: 7.567629814147949, y: 5.355677127838135 },
                p2: Point { x: 7.567629814147949, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 35
                p1: Point { x: 7.567629814147949, y: 5.875 },
                p2: Point { x: 9.772912979125977, y: 5.875 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 36
                p1: Point { x: 9.772912979125977, y: 5.875 },
                p2: Point { x: 9.772912979125977, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 37
                p1: Point { x: 9.772912979125977, y: 5.355677127838135 },
                p2: Point { x: 8.950581550598145, y: 5.355677127838135 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 38
                p1: Point { x: 8.950581550598145, y: 5.355677127838135 },
                p2: Point { x: 8.950581550598145, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 39
                p1: Point { x: 8.950581550598145, y: 3.0 },
                p2: Point { x: 8.393552780151367, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        /*
        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 2.200923442840576, y: 5.355677127838135 }, 0, 1),
                (Point { x: 2.200923442840576, y: 3.0 }, 0, 7),
                (Point { x: 1.375, y: 5.355677127838135 }, 1, 2),
                (Point { x: 1.375, y: 5.875 }, 2, 3),
                (Point { x: 3.5802841186523438, y: 5.875 }, 3, 4),
                (Point { x: 3.5802841186523438, y: 5.355677127838135 }, 4, 5),
                (Point { x: 2.7579517364501953, y: 5.355677127838135 }, 5, 6),
                (Point { x: 2.7579517364501953, y: 3.0 }, 6, 7),
                (Point { x: 3.727086067199707, y: 5.875 }, 8, 9),
                (Point { x: 3.727086067199707, y: 3.0 }, 8, 19),
                (Point { x: 5.438611030578613, y: 5.875 }, 9, 10),
                (Point { x: 5.378286559813183, y: 5.875 }, 9, 24),
                (Point { x: 5.438611030578613, y: 5.875 }, 9, 25),
                (Point { x: 5.438611030578613, y: 5.355677127838135 }, 10, 11),
                (Point { x: 5.438611030578613, y: 5.781783390062552 }, 10, 24),
                (Point { x: 5.438611030578613, y: 5.875 }, 10, 25),
                (Point { x: 4.282318115234375, y: 5.355677127838135 }, 11, 12),
                (Point { x: 4.282318115234375, y: 4.756412506103516 }, 12, 13),
                (Point { x: 5.372178077697754, y: 4.756412506103516 }, 13, 14),
                (Point { x: 5.372178077697754, y: 4.240680694580078 }, 14, 15),
                (Point { x: 4.282318115234375, y: 4.240680694580078 }, 15, 16),
                (Point { x: 4.282318115234375, y: 3.5193228721618652 }, 16, 17),
                (Point { x: 5.438611030578613, y: 3.5193228721618652 }, 17, 18),
                (Point { x: 5.438611030578613, y: 3.0 }, 18, 19),
                (Point { x: 5.438611030578613, y: 3.0 }, 18, 22),
                (Point { x: 5.438611030578613, y: 3.1874854085696747 }, 18, 23),
                (Point { x: 5.438611030578613, y: 3.0 }, 19, 22),
                (Point { x: 5.3187876869681565, y: 3.0 }, 19, 23),
                (Point { x: 6.59119565394872, y: 4.043521440830737 }, 20, 21),
                (Point { x: 7.231243872291176, y: 3.0 }, 20, 31),
                (Point { x: 5.941425375931249, y: 3.0 }, 21, 22),
                (Point { x: 5.3187876869681565, y: 3.0 }, 22, 23),
                (Point { x: 6.272807746257463, y: 4.492737852325395 }, 23, 24),
                (Point { x: 5.378286559813183, y: 5.875 }, 24, 25),
                (Point { x: 6.016635762592474, y: 5.875 }, 25, 26),
                (Point { x: 6.603059113306081, y: 4.938329019826689 }, 26, 27),
                (Point { x: 7.192695823314598, y: 5.874999999999999 }, 27, 28),
                (Point { x: 7.8149446973032, y: 5.875 }, 28, 29),
                (Point { x: 7.567629814147949, y: 5.874999999999999 }, 28, 34),
                (Point { x: 7.8149446973032, y: 5.875 }, 28, 35),
                (Point { x: 6.92191888204544, y: 4.497921348517647 }, 29, 30),
                (Point { x: 7.567629814147949, y: 5.493631354247416 }, 29, 34),
                (Point { x: 7.8149446973032, y: 5.875 }, 29, 35),
                (Point { x: 7.8754980994305495, y: 3.0 }, 30, 31),
                (Point { x: 8.393552780151367, y: 5.355677127838135 }, 32, 33),
                (Point { x: 8.393552780151367, y: 3.0 }, 32, 39),
                (Point { x: 7.567629814147949, y: 5.355677127838135 }, 33, 34),
                (Point { x: 7.567629814147949, y: 5.875 }, 34, 35),
                (Point { x: 9.772912979125977, y: 5.875 }, 35, 36),
                (Point { x: 9.772912979125977, y: 5.355677127838135 }, 36, 37),
                (Point { x: 8.950581550598145, y: 5.355677127838135 }, 37, 38),
                (Point { x: 8.950581550598145, y: 3.0 }, 38, 39),
            ],
        );
        */
    }

    #[test]
    pub fn test_all_intersections_touching_example18() {
        // (Point { x: 7.567629814147949, y: 5.875 }, 28, 35)
        let lines = vec![
            LineSegment { // 0
                p1: Point { x: 0.0, y: 0.0 },
                p2: Point { x: 2.0, y: 0.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 1
                p1: Point { x: 1.0, y: 0.0 },
                p2: Point { x: 3.0, y: 0.0 },
                includes_first_point: true, includes_second_point: false,
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 1.0, y: 0.0 }, 0, 1),
                (Point { x: 2.0, y: 0.0 }, 0, 1),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_example19() {
        // (Point { x: 7.567629814147949, y: 5.875 }, 28, 35)
        let lines = vec![
            LineSegment { // 0
                p1: Point { x: 0.0, y: 0.0 },
                p2: Point { x: 0.0, y: 2.0 },
                includes_first_point: true, includes_second_point: false,
            },
            LineSegment { // 1
                p1: Point { x: 0.0, y: 1.0 },
                p2: Point { x: 0.0, y: 3.0 },
                includes_first_point: true, includes_second_point: false,
            },
        ];

        LineSegment::print_python_code_to_graph(&lines);

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (Point { x: 0.0, y: 1.0 }, 0, 1),
                (Point { x: 0.0, y: 2.0 }, 0, 1),
            ],
        );
    }

    #[test]
    pub fn test_all_intersections_touching_bottoms() {
        let lines = vec![
            LineSegment::from(
                Point::from(0.0, 0.0),
                Point::from(3.0, 5.0),
            ), // 1x
            LineSegment::from(
                Point::from(0.0, 0.0),
                Point::from(-3.0, 5.0),
            ), // 1x
        ];

        compare_output_all_intersections(
            &LineSegment::all_intersections(&lines),
            &vec![
                (
                    Point::from(0.0, 0.0),
                    0, 1
                ),
            ],
        );
    }

    fn compare_output_all_intersections_combine_points(
        mut expected: Vec<(Point, Vec<(usize, usize)>)>,
        mut actual  : Vec<(Point, Vec<(usize, usize)>)>,
    ) {
        assert_eq!(expected.len(), actual.len());
        'outer: for i in 0..expected.len() {
            for j in 0..actual.len() {
                if !test_point(expected[i].0, actual[j].0) {
                    continue;
                }
                expected[i].1.sort();
                actual[j].1.sort();
                if expected[i].1 != actual[j].1 {
                    println!("{:?} != {:?}", expected[i], actual[j]);
                    assert_eq!(expected, actual);
                }

                continue 'outer;
            }
            assert_eq!(expected, actual);
        }
    }

    #[test]
    pub fn test_all_intersections_combine_points() {
        let lines = vec![
            LineSegment::from(
                Point::from(5.0, 5.0),
                Point::from(0.0, 0.0),
            ), // 1x
            LineSegment::from(
                Point::from(6.0, 4.0),
                Point::from(1.0, 1.0),
            ), // 3/5x + 2/5
            LineSegment::from(
                Point::from(4.0, 3.0),
                Point::from(9.0, 2.0),
            ), // -1/5x + 3+4/5
        ];

        compare_output_all_intersections_combine_points(
            LineSegment::all_intersections_combine_points(&lines, 1000.0),
            vec![
                (
                    Point::from(1.0, 1.0),
                    vec![
                        (1, 1)
                    ],
                ),
                (
                    Point::from(4.25, 2.95),
                    vec![
                        (0, 1)
                    ],
                ),
            ]
        );
    }

    #[test]
    pub fn test_distance_down_an_angle() {
        test_float(
            Point::distance_down_an_angle(std::f64::consts::PI / 2.0, 2.0_f64.sqrt()),
            1.0,
        );
    }
}
