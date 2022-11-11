#![allow(dead_code)]

pub struct Path<T> {
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
    transform: Box<dyn Fn (usize, usize) -> (T, T)>,
    next_itr: fn (&mut PathItr<T>) -> bool,
}

pub struct PathItr<T> {
    path: Path<T>,
    x: usize,
    y: usize,
    is_down: bool,
    is_end: bool,
}

impl<T> std::fmt::Display for Path<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(width: {}, height: {}, x: {}, y: {})",
            self.width, self.height, self.start_x, self.start_y)
    }
}

impl<T> std::fmt::Display for PathItr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, is_down: {}, is_end: {}, path: {})",
            self.x, self.y, self.is_down, self.is_end, self.path)
    }
}

impl<T> Path<T> {
    pub fn from(width: usize, height: usize,
        transform: Box<dyn Fn (usize, usize) -> (T, T)>,
        next_itr: fn (&mut PathItr<T>) -> bool,
    ) -> Self {
        Self {
            width: width,
            height: height,
            start_x: 0,
            start_y: 0,
            transform: transform,
            next_itr: next_itr,
        }
    }

    pub fn start_middle(mut self) -> Self {
        self.start_x = self.width / 2;
        self.start_y = self.height / 2;
        self
    }

    pub fn set_x(mut self, x: usize) -> Self {
        self.start_x = x;
        self
    }

    pub fn set_y(mut self, y: usize) -> Self {
        self.start_y = y;
        self
    }

    pub fn path_x_then_y(path_itr: &mut PathItr<T>) -> bool {
        if path_itr.y % 2 == 0 {
            if path_itr.x + 1 >= path_itr.path.width {
                path_itr.y += 1;
            } else {
                path_itr.x += 1;
            }
        } else {
            if path_itr.x == 0 {
                path_itr.y += 1;
            } else {
                path_itr.x -= 1;
            }
        }
        return path_itr.y < path_itr.path.height;
    }

    pub fn path_y_then_x(path_itr: &mut PathItr<T>) -> bool {
        if path_itr.x % 2 == 0 {
            if path_itr.y + 1 >= path_itr.path.height {
                path_itr.x += 1;
            } else {
                path_itr.y += 1;
            }
        } else {
            if path_itr.y == 0 {
                path_itr.x += 1;
            } else {
                path_itr.y -= 1;
            }
        }
        return path_itr.x < path_itr.path.width;
    }

    pub fn spiral_in_out(path_itr: &mut PathItr<T>) -> bool {

        if !path_itr.is_down {
            path_itr.is_down = true;
            return true;
        }

        let total = path_itr.path.start_x + path_itr.path.start_y;
        let y_intercept = path_itr.path.start_y as i64 - path_itr.path.start_x as i64;

        // We change it to false only the few times we pick it up
        path_itr.is_down = true;

        if path_itr.x + path_itr.y >= total {
            // Move right
            if path_itr.y as i64 - path_itr.x as i64 > y_intercept {
                path_itr.x += 1;
            } else { // Move Down
                // Alternatively could use std::num::Wrapping
                if path_itr.y == 0 { 
                    if path_itr.path.width > path_itr.path.height {
                        path_itr.is_down = false;
                        if 2 * path_itr.path.start_x >= path_itr.x + 1 {
                            path_itr.x = 2 * path_itr.path.start_x - path_itr.x - 1;
                        }
                        else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    path_itr.y -= 1;
                }
            }
        } else {
            // Move Left
            if (path_itr.y as i64 - path_itr.x as i64) < y_intercept {
                // Alternatively could use std::num::Wrapping
                if path_itr.x == 0 {
                    if path_itr.path.height > path_itr.path.width {
                        path_itr.is_down = false;
                        if 2 * path_itr.path.start_y >= path_itr.y {
                            path_itr.y = 2 * path_itr.path.start_y - path_itr.y;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    path_itr.x -= 1;
                }
            } else { // Move Up
                path_itr.y += 1;
            }
        }

        if path_itr.x >= path_itr.path.width &&
            path_itr.path.height > path_itr.path.width {
            path_itr.x = path_itr.path.width - 1;
            if 2 * path_itr.path.start_y < path_itr.y + 1 {
                return false;
            }
            path_itr.y = 2 * path_itr.path.start_y - path_itr.y - 1;
            path_itr.is_down = false;
        }

        if path_itr.y >= path_itr.path.height &&
            path_itr.path.width > path_itr.path.height {
            path_itr.y = path_itr.path.height - 1;
            if 2 * path_itr.path.start_x < path_itr.x {
                return false;
            }
            path_itr.x = 2 * path_itr.path.start_x - path_itr.x;
            path_itr.is_down = false;
        }

        path_itr.x < path_itr.path.width &&
            path_itr.y < path_itr.path.height
    }
}

impl<T : From<usize>> Path<T> {
    pub fn identity(l: usize, r: usize) -> (T, T) {
        (l.into(), r.into())
    }
    // pub fn identity_to_f64(l: usize, r: usize) -> (f64, f64) {
    //     (l as f64, r as f64)
    // }
}

impl<T> IntoIterator for Path<T> {
    type Item = (T, T, bool);
    type IntoIter = PathItr<T>;

    fn into_iter(self) -> Self::IntoIter {
        let x = self.start_x;
        let y = self.start_y;
        PathItr {
            path: self,
            x: x,
            y: y,
            is_down: true,
            is_end: false,
        }
    }
}

impl<T> Iterator for PathItr<T> {
    type Item = (T, T, bool);
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end { return None; }
        let pos = (self.x, self.y, self.is_down);
        {
            let f = self.path.next_itr;
            if !f(self) {
                self.is_end = true;
            }
        }

        let t = &self.path.transform;
        let (x, y) = t(pos.0, pos.1);
        Some((x, y, pos.2))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn print_has_seen(has_seen: &Vec<Vec<bool>>, x_pos: usize, y_pos: usize, is_down: bool) {
        for x_index in 0..has_seen.len() {
            for y_index in 0..has_seen[x_index].len() {
                print!("{}",
                    if x_index == x_pos && y_index == y_pos {
                        if is_down {
                            " V "
                        } else {
                            " ^ "
                        }
                    } else {
                        if has_seen[x_index][y_index] { " * " } else { " . " }
                    }
                );
            }
            println!("");
        }
    }

    fn test_path_complete(
        next_itr: fn (&mut PathItr<usize>) -> bool,
        width: usize, height: usize, start_x : usize, start_y: usize) -> bool {
        let mut has_seen = Vec::new();
        for _ in 0..width {
            let mut new_row = Vec::new();
            for _ in 0..height {
                new_row.push(false);
            }
            has_seen.push(new_row);
        }

        let mut last_x = start_x;
        let mut last_y = start_y;

        for (x, y, is_down) in Path::from(
            width, height,
            Box::from(Path::<usize>::identity),
            next_itr
        ).set_x(start_x).set_y(start_y) {
            // println!("X{} Y{}{}", x, y, if is_down { "" } else { " UP"} );
            // print_has_seen(&has_seen, x, y, is_down);
            if !is_down { 
                last_x = x;
                last_y = y;
                continue;
            }

            if (last_x as i64 - x as i64 ).abs() + (last_y as i64 - y as i64).abs() > 1 {
                println!("To big of a jump from x{} y{} to x{} y{}.",
                    last_x, last_y, x, y);
                return false;
            }
            last_x = x;
            last_y = y;

            if has_seen[x][y] {
                // println!("Seen two of the same on X{} Y{}", x, y);
                return false;
            }
            has_seen[x][y] = true;
        }

        for r in has_seen {
            for x in r {
                if !x {
                    println!("Did not see one");
                    return false;
                }
            }
        }

        return true;
    }

    #[test]
    pub fn test_x_then_y() {
        assert!(
            test_path_complete(Path::path_x_then_y, 30, 100, 0, 0),
            "Failed test_x_then_y 30, 100, 0, 0"
        );

        assert!(
            test_path_complete(Path::path_x_then_y, 100, 100, 0, 0),
            "Failed test_x_then_y 100, 100, 0, 0"
        );

        assert!(
            test_path_complete(Path::path_x_then_y, 1, 1, 0, 0),
            "Failed test_x_then_y 1, 1, 0, 0"
        );

        assert!(
            test_path_complete(Path::path_x_then_y, 100, 1, 0, 0),
            "Failed test_x_then_y 100, 1, 0, 0"
        );
    }

    #[test]
    pub fn test_y_then_x() {
        assert!(
            test_path_complete(Path::path_y_then_x, 30, 100, 0, 0),
            "Failed test_y_then_x 30, 100, 0, 0"
        );

        assert!(
            test_path_complete(Path::path_y_then_x, 100, 100, 0, 0),
            "Failed test_y_then_x 100, 100, 0, 0"
        );

        assert!(
            test_path_complete(Path::path_y_then_x, 1, 1, 0, 0),
            "Failed test_y_then_x 1, 1, 0, 0"
        );

        assert!(
            test_path_complete(Path::path_y_then_x, 100, 1, 0, 0),
            "Failed test_y_then_x 100, 1, 0, 0"
        );
    }

    #[test]
    pub fn test_spiral_in_out() {
        assert!(
            test_path_complete(Path::spiral_in_out, 10, 10, 5, 5),
            "Failed test_path_complete 10, 10, 5, 5"
        );

        assert!(
            test_path_complete(Path::spiral_in_out, 8, 8, 4, 4),
            "Failed test_path_complete 8, 8, 4, 4"
        );

        assert!(
            test_path_complete(Path::spiral_in_out, 7, 7, 3, 3),
            "Failed test_path_complete 7, 7, 3, 3"
        );

        assert!(
            test_path_complete(Path::spiral_in_out, 1, 1, 0, 0),
            "Failed test_path_complete 1, 1, 0, 0"
        );

        assert!(
            test_path_complete(Path::spiral_in_out, 16, 8, 8, 4),
            "Failed test_path_complete 16, 8, 8, 4"
        );

        assert!(
            test_path_complete(Path::spiral_in_out, 8, 16, 4, 8),
            "Failed test_path_complete 8, 16, 4, 8"
        );

        assert!(
            test_path_complete(Path::spiral_in_out, 16, 1, 8, 0),
            "Failed test_path_complete 16, 1, 8, 0"
        );

        assert!(
            test_path_complete(Path::spiral_in_out, 1, 16, 0, 8),
            "Failed test_path_complete 1, 16, 0, 8"
        );
    }
}

