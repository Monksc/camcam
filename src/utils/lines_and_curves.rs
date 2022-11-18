#![allow(dead_code)]
// /src/utils/lines_and_curves.rs
use super::*;

pub trait Intersection {
    fn y(&self, next: &Self, x: f64) -> Vec<f64>;
    fn times_cross_line(&self, line: &LineSegment) -> usize;
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
}

#[derive(Debug, Clone)]
pub enum AllIntersections {
    Rectangle(Rectangle),
    SoftLineSegment(LineSegment),
    LineSegment(LineSegment),
    Circle(Circle),
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
        // TODO: This function really needs to be speed up a bit.
        // Maybe look up the best way how instead of trying to figure
        //      out on your own.

        // theta = atan(
        //      [cos a2 (d(p2) / d(p1)) - cos a1 ] /
        //      [sin a2 (d(p2) / d(p1)) - sin a1 ]
        // )

        let degrees = if line.p1 == Point::zero() && line.p2 == Point::zero() {
            0.0
        } else if line.p1 == Point::zero() {
            let a = (line.p2.x / line.p2.distance_to(&Point::zero())).acos();
            std::f64::consts::PI/2.0 - a
        } else if line.p2 == Point::zero() {
            let a = (line.p1.x / line.p1.distance_to(&Point::zero())).acos();
            std::f64::consts::PI/2.0 - a
        } else {
            let a1 = (line.p1.x / line.p1.distance_to(&Point::zero())).acos();
            let a2 = (line.p2.x / line.p2.distance_to(&Point::zero())).acos();

            let p1d = line.p1.distance_to(&Point::zero());
            let p2d = line.p2.distance_to(&Point::zero());

            (
                (a2.cos() * (p2d / p1d) - a1.cos()) /
                (a2.sin() * (p2d / p1d) - a1.sin())
            ).atan()
        };


        let l11 = degrees.cos();
        let l12 = -degrees.sin();
        let l21 = degrees.sin();
        let l22 = degrees.cos();

        let m = (l11, l12, l21, l22);
        let p1 = multiply_matrix_m(m, line.p1.x, line.p1.y);
        let p2 = multiply_matrix_m(m, line.p2.x, line.p2.y);
        let c  = multiply_matrix_m(m, self.center.x, self.center.y);

        // println!("P1: ({}, {})", p1.0, p1.1);
        // println!("P2: ({}, {})", p2.0, p2.1);
        // println!("C : ({}, {})", c.0, c.1);
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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
    pub fn point1(&self) -> Point {
        self.p1
    }
    pub fn point2(&self) -> Point {
        self.p2
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
        }
        else if let Some((b, m)) = self.y_intercept_and_slope() {
            Some(x * m + b)
        } else {
            None
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

    pub fn contains_x(&self, x: f64) -> bool {
        let first_point_gt = x > self.p1.x || (self.includes_first_point && x >= self.p1.x);
        let first_point_lt = x < self.p1.x || (self.includes_first_point && x <= self.p1.x);
        let second_point_gt = x > self.p2.x || (self.includes_second_point && x >= self.p2.x);
        let second_point_lt = x < self.p2.x || (self.includes_second_point && x <= self.p2.x);

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

    pub fn length(&self) -> f64 {
        (
            (self.p2.x - self.p1.x).powi(2) +
            (self.p2.y - self.p1.y).powi(2)
        ).sqrt()
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.contains_x(point.x) && self.contains_y(point.y)
    }

    pub fn contains_point_endless_line(&self, point: Point) -> bool {
        if let Some((b, m)) = self.y_intercept_and_slope() {
            m * point.x + b == point.y
        } else {
            point.x == self.p1.x
        }
    }
}

impl Intersection for LineSegment {
    fn y(&self, next: &Self, x: f64) -> Vec<f64> {
        let contains_x =
            (self.p1.x < self.p2.x && x >= self.p1.x && x < self.p2.x) ||
            (self.p1.x > self.p2.x && x <= self.p1.x && x > self.p2.x);
        let end_x = x == self.p2.x;
        let lines_on_same_side_as_x =
            (self.p1.x >= x && self.p2.x >= x && next.p1.x >= x && next.p2.x > x) ||
            (self.p1.x <= x && self.p2.x <= x && next.p1.x <= x && next.p2.x < x);
        if let Some((b, m)) = self.y_intercept_and_slope() {
            if contains_x || (end_x && lines_on_same_side_as_x) {
                return vec![m * x + b];
            } else {
                return Vec::new();
            }
        } else if self.p1.x == x {
            return vec![
                self.p1.y,
            ];
        } else {
            return Vec::new();
        }

        // if let Some(y) = self.y(x) {
        //     return vec![
        //         y
        //     ];
        // }

        // if let Some(y) = (LineSegment::from_include(
        //         self.p1,
        //         self.p2,
        //         true,
        //         true
        //     )).y(x) {

        //     if (self.p1.x > self.p2.x && next.p2.x > next.p1.x) ||
        //         (self.p1.x < self.p2.x && next.p2.x < next.p1.x) {
        //         return vec![
        //             y
        //         ];
        //     }
        // }
        // return Vec::new()
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
    fn y(&self, _next: &Self, x: f64) -> Vec<f64> {
        if self.contains_x(x) {
            vec![
                self.start_point.y,
                self.end_point.y,
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
    fn y(&self, _next: &Self, x: f64) -> Vec<f64> {
        // X^2 + Y^2 = r^2
        // Y^2 = r^2 - X^2
        // Y += sqrt(r^2 - X^2)
        if self.contains_x(x) {
            let difx = x - self.center.x;
            let positive_value = (self.radius * self.radius - difx * difx).sqrt();
            vec![
                self.center.y + positive_value,
                self.center.y - positive_value,
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
    fn y(&self, next: &Self, x: f64) -> Vec<f64> {
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
            AllIntersections::SoftLineSegment(_) => {
                false
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
            AllIntersections::SoftLineSegment(_) => {
                Vec::new()
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
            AllIntersections::SoftLineSegment(_) => {
                false
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
            AllIntersections::SoftLineSegment(_) => {
                None
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
        if (l-r).abs() >= 0.00000000000001 {
            println!("L: {}, R: {}", l, r);
            return false;
        }
        return true;
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
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![0.0]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![0.1]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![0.5]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![0.9]);
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
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![1.0]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![0.9]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![0.5]);
        // assert_eq!(Intersection::y(&line, &next, 0.9), vec![0.1]);
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
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![0.0]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![0.1]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![0.5]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![0.9]);
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
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![0.0]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![0.1]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![0.5]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![0.9]);
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![1.0]);
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
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![0.0]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![0.1]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![0.5]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![0.9]);
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![1.0]);
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
        assert_eq!(Intersection::y(&line, &next, 0.0), vec![0.0]);
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
        assert_eq!(Intersection::y(&line, &next, 1.0), vec![0.0]);
        assert_eq!(Intersection::y(&line, &next, 0.9), vec![0.0]);
        assert_eq!(Intersection::y(&line, &next, 0.5), vec![0.0]);
        assert_eq!(Intersection::y(&line, &next, 0.1), vec![0.0]);
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
}
