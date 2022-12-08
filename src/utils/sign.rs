#![allow(dead_code)]
use super::*;


#[derive(Debug, Clone)]
pub struct Sign<T : lines_and_curves::Intersection + Clone> {
    bounding_rect: lines_and_curves::Rectangle,
    shapes: Vec<Shape<T>>,
    contains_rect: ContainsRectangle<T>
}

#[derive(Debug, Clone)]
pub struct Shape<T: lines_and_curves::Intersection> {
    lines: Vec<T>,
    tool_type: cnc_router::ToolType,
    layers: std::collections::HashMap
        <usize, Vec<(f64, Vec<f64>)>>, // [x_index][x] = [y]
}

#[derive(Debug, Clone)]
struct ContainsRectangle<T: lines_and_curves::Intersection + Clone> {
    bounding_rect: lines_and_curves::Rectangle,
    split: Box<SplitOrLines<T>>,
}

#[derive(Debug, Clone)]
enum SplitOrLines<T: lines_and_curves::Intersection + Clone> {
    Split(Vec<ContainsRectangle<T>>),
    Lines(Vec<T>)
}

fn f64_to_usize_block(x: f64) -> usize {
    let mut x = x * 32.0;
    if x < 0.0 {
        x = 0.0-x;
    }
    if x < 1.0 {
        x += 1.0
    }
    x.round() as usize
}

impl<T: lines_and_curves::Intersection + Clone> Sign<T> {
    pub fn from(
        bounding_rect: lines_and_curves::Rectangle,
        shapes: Vec<Shape<T>>
    ) -> Self {
        let mut lines = Vec::new();
        for shape in &shapes {
            lines.extend(shape.lines().clone());
        }

        let contains_rect = ContainsRectangle::from(&bounding_rect, &lines);

        Self {
            bounding_rect: bounding_rect,
            shapes: shapes,
            contains_rect: contains_rect
        }
    }

    pub fn bounding_rect(&self) -> &lines_and_curves::Rectangle {
        &self.bounding_rect
    }

    pub fn shapes(&self) -> &Vec<Shape<T>> {
        &self.shapes
    }

    pub fn y_values_before(&mut self, x: f64, y: f64) -> usize {
        let mut seen = 0;
        for shape in &mut self.shapes {
            seen += shape.y_values_before(x, y);
        }

        return seen;
    }

    pub fn get_next_y_value(&mut self, x: f64, y: f64) -> f64 {
        let mut min_y = None;
        for shape in &mut self.shapes {
            match (shape.get_next_y_value(x, y), min_y) {
                (Some(y), None) => min_y = Some(y),
                (Some(y), Some(old_min_y)) => if y < old_min_y {
                    min_y = Some(y)
                },
                (_, _) => {

                }
            }
        }

        if let Some(y) = min_y {
            y
        } else {
            self.bounding_rect.max_y()
        }
    }

    pub fn get_prev_y_value(&mut self, x: f64, y: f64) -> f64 {
        let mut max_y = None;
        for shape in &mut self.shapes {
            match (shape.get_prev_y_value(x, y), max_y) {
                (Some(y), None) => max_y = Some(y),
                (Some(y), Some(old_max_y)) => if y > old_max_y {
                    max_y = Some(y)
                },
                (_, _) => {

                }
            }
        }

        if let Some(y) = max_y {
            y
        } else {
            self.bounding_rect.min_y()
        }
    }

    pub fn line_collides_wth_rect(&mut self, rectangle: &lines_and_curves::Rectangle) -> bool {
        self.contains_rect.contains_rect(rectangle)
    }
}

impl<T: lines_and_curves::Intersection> Shape<T> {
    pub fn from(
        tool_type: cnc_router::ToolType,
        lines: Vec<T>,
    ) -> Self {
        Self {
            lines: lines,
            layers: std::collections::HashMap::new(),
            tool_type: tool_type,
        }
    }

    pub fn tool_type(&self) -> cnc_router::ToolType {
        self.tool_type
    }

    pub fn bounding_box(&self) -> Option<lines_and_curves::Rectangle> {
        lines_and_curves::bounding_box(&self.lines)
    }

    pub fn lines(&self) -> &Vec<T> {
        &self.lines
    }

    pub fn add_x_layer(&mut self, x: f64) {
        if self.get_y_values(x).len() > 0 {
            return;
        }
        let x_index = f64_to_usize_block(x);

        let mut y_values = Vec::new();
        for (i, line) in self.lines.iter().enumerate() {
            for y in line.y(&self.lines[(i+1)%self.lines.len()], x) {
                y_values.push(y);
            }
        }

        y_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        match self.layers.get_mut(&x_index) {
            Some(arr) => {
                arr.push((x, y_values));
            },
            None => {
                self.layers.insert(x_index, vec![(x, y_values)]);
            }
        }
    }

    fn get_y_values(&self, x: f64) -> Vec<&Vec<f64>> {
        let index = f64_to_usize_block(x);
        let mut arrays = Vec::new();
        for d in [-1, 0, 1] {
            let indexd = ((index as i64) + d) as usize;
            if !self.layers.contains_key(&indexd) {
                continue;
            }
            for (x_itr, arr) in
                &self.layers[&indexd] {
                if x == *x_itr {
                    arrays.push(arr);
                }
            }
        }

        return arrays;
    }

    pub fn y_values_before(&mut self, x: f64, y: f64) -> usize {
        self.add_x_layer(x);
        let mut seen = 0;

        for y_values in self.get_y_values(x) {
            seen += algorithms::seen_before(y_values, y);
        }

        return seen;
    }

    pub fn get_next_y_value(&mut self, x: f64, y: f64) -> Option<f64> {
        self.add_x_layer(x);

        let mut min_y = None;

        for y_values in self.get_y_values(x) {
            let y_index = algorithms::seen_before_or_equal(y_values, y);
            if y_index >= y_values.len() { continue }
            let y_value = y_values[y_index];

            if let Some(some_min_y) = min_y {
                if some_min_y < y_value {
                    min_y = Some(y_value);
                }
            } else {
                min_y = Some(y_value);
            }
        }

        return min_y;
    }

    pub fn get_prev_y_value(&mut self, x: f64, y: f64) -> Option<f64> {
        self.add_x_layer(x);

        let mut max_y = None;

        for y_values in self.get_y_values(x) {
            let y_index = algorithms::seen_before_or_equal(y_values, y);
            if y_index == 0 { continue }
            let y_value = y_values[y_index-1];

            if let Some(some_max_y) = max_y {
                if some_max_y > y_value {
                    max_y = Some(y_value);
                }
            } else {
                max_y = Some(y_value);
            }
        }

        return max_y;
    }
}

impl<T: lines_and_curves::Intersection + Clone> ContainsRectangle<T> {
    fn from(bounding_rect: &lines_and_curves::Rectangle, lines: &Vec<T>) -> Self {
        Self {
            bounding_rect: bounding_rect.clone(),
            split: Box::from(
                SplitOrLines::from(
                    &bounding_rect, lines.clone(), 16
                ),
            ),
        }
    }

    fn contains_rect(&self, rect: &lines_and_curves::Rectangle) -> bool {
        use crate::utils::lines_and_curves::Intersection;
        self.bounding_rect.intersects_rectangle(&rect) && self.split.contains_rect(rect)
    }
}

impl<T: lines_and_curves::Intersection + Clone> SplitOrLines<T> {
    fn split_rectangle(
        rect: &lines_and_curves::Rectangle, lines: &Vec<T>
    ) -> Vec<lines_and_curves::Rectangle> {
        assert!(lines.len() > 0);
        if rect.width() > rect.height() {
            let mean_y = lines.into_iter().map(
                |x| {
                    x.bounding_box().mid_x()
                }
            ).reduce(|l, r| l+r).unwrap() / lines.len() as f64;
            vec![
                lines_and_curves::Rectangle::from(
                    lines_and_curves::Point::from(rect.p1().x, rect.p1().y),
                    lines_and_curves::Point::from(rect.p2().x, mean_y),
                ),
                lines_and_curves::Rectangle::from(
                    lines_and_curves::Point::from(rect.p1().x, mean_y),
                    lines_and_curves::Point::from(rect.p2().x, rect.p2().y),
                ),
            ]
        } else {
            let mean_x = lines.into_iter().map(
                |x| {
                    x.bounding_box().mid_y()
                }
            ).reduce(|l, r| l+r).unwrap() / lines.len() as f64;
            vec![
                lines_and_curves::Rectangle::from(
                    lines_and_curves::Point::from(rect.p1().x, rect.p1().y),
                    lines_and_curves::Point::from(mean_x, rect.p2().y),
                ),
                lines_and_curves::Rectangle::from(
                    lines_and_curves::Point::from(mean_x, rect.p1().y),
                    lines_and_curves::Point::from(rect.p2().x, rect.p2().y),
                ),
            ]
        }
    }

    fn from(bounding_rect: &lines_and_curves::Rectangle, lines: Vec<T>, max_depth: usize) -> Self {
        if lines.len() <= 16 || max_depth == 0 {
            return Self::Lines(lines);
        }

        let mut splits : Vec<ContainsRectangle<T>> = Vec::new();

        for rect in Self::split_rectangle(bounding_rect, &lines) {
            let mut new_lines : Vec<T> = Vec::new();
            for line in &lines {
                if line.intersects_rectangle(&rect) {
                    new_lines.push(line.clone());
                }
            }

            let split = SplitOrLines::from(
                &rect, new_lines, max_depth-1
            );

            splits.push(
                ContainsRectangle {
                    bounding_rect: rect,
                    split: Box::from(split)
                }
            );
        }

        return Self::Split(splits);
    } 

    fn contains_rect(&self, rect: &lines_and_curves::Rectangle) -> bool {
        match self {
            SplitOrLines::Split(contains_rects) => {
                for cr in contains_rects {
                    if cr.contains_rect(rect) {
                        return true;
                    }
                }
            }
            SplitOrLines::Lines(lines) => {
                for line in lines {
                    if line.intersects_rectangle(rect) {
                        return true
                    }
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{cnc_router};

    #[test]
    pub fn test_times_cross_line_and_rect() {

        let mut sign = sign::Sign::from(
            lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(10.0, 10.0),
                lines_and_curves::Point::from(25.0, 25.0)
            ),
            vec![
                sign::Shape::from(
                    cnc_router::ToolType::Text,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(17.5, 15.0),
                        lines_and_curves::Point::from(20.0, 17.5),
                        lines_and_curves::Point::from(17.5, 20.0),
                        lines_and_curves::Point::from(15.0, 17.5),
                    ], true),
                ),
            ]
        );

        assert_eq!(
            sign.line_collides_wth_rect(&lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(9.0, 9.0),
                lines_and_curves::Point::from(11.0, 11.0),
            )),
            false
        );

        assert_eq!(
            sign.line_collides_wth_rect(&lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(17.0, 15.0),
                lines_and_curves::Point::from(18.0, 16.0),
            )),
            true
        );

        assert_eq!(
            sign.y_values_before(19.17142857142857, 17.025000000000002), 1
        );

        assert_eq!(
            sign.y_values_before(17.5, 17.5), 1
        );
    }
}

