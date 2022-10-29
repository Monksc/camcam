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
    LineSegment(LineSegment),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone)]
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
        if let Some(y) = self.y(x) {
            return vec![
                y
            ];
        }

        if let Some(y) = (LineSegment::from_include(
                self.p1,
                self.p2,
                true,
                true
            )).y(x) {

            if (self.p1.x > self.p2.x && next.p2.x > next.p1.x) || 
                (self.p1.x < self.p2.x && next.p2.x < next.p1.x) {
                return vec![
                    y
                ];
            }
        }
        return Vec::new()
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

impl Intersection for Rectangle {
    fn y(&self, _next: &Self, x: f64) -> Vec<f64> {
        if self.contains_x(x) {
            Vec::new()
        } else {
            vec![
                self.start_point.y,
                self.end_point.y,
            ]
        }
    }

    fn times_cross_line(&self, line: &LineSegment) -> usize {
        if line.intersects_rectangle(self) {
            1
        } else {
            0
        }
    }

    fn intersects_rectangle(&self, rect : &Rectangle) -> bool {
        (self.contains_x(rect.start_point.x) || self.contains_x(rect.end_point.x)) &&
            (self.contains_y(rect.start_point.y) || self.contains_y(rect.end_point.y))
    }

    fn bounding_box(&self) -> Rectangle {
        self.clone()
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
        let x = self.convert_x(point.x);
        let y = self.convert_y(point.y);
        self.tiles[x][y] = true;
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
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
