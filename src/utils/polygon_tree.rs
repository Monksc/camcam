use super::*;

#[derive(Debug, Clone)]
pub struct PolygonTree<T : lines_and_curves::Intersection + Clone> {
    heads: Vec<PolygonTreeItem<T>>,
    can_cut_inside: bool,
    total_count_added: usize,
}

#[derive(Debug, Clone)]
pub struct PolygonTreeItem<T : lines_and_curves::Intersection + Clone> {
    bounding_box: lines_and_curves::Rectangle,
    polygon: Vec<T>,
    inner_polygons: Vec<Box<PolygonTreeItem<T>>>,
    can_cut_inside: bool,
    id: usize,
}

impl<T : lines_and_curves::Intersection + Clone> Default for PolygonTree<T> {
    fn default() -> Self {
        Self {
            heads: Vec::new(),
            can_cut_inside: false,
            total_count_added: 0,
        }
    }
}

impl<T : lines_and_curves::Intersection + Clone> From<&Vec<(Vec<T>, bool)>> for PolygonTree<T> {
    fn from(tree: &Vec<(Vec<T>, bool)>) -> Self {
        let mut poly_tree = PolygonTree::default();
        for i in 0..tree.len() {
            let mut poly = PolygonTreeItem::from(
                tree[i].clone(),
            );
            poly_tree.add_polygon(
                &mut poly
            );
        }

        return poly_tree;
    }
}

impl<T : lines_and_curves::Intersection + Clone> PolygonTree<T> {
    fn add_polygon(&mut self, polygon_tree_item: &mut PolygonTreeItem<T>) {
        polygon_tree_item.id = self.total_count_added;
        self.total_count_added += 1;
        for i in 0..self.heads.len() {
            if self.heads[i].add_polygon_if_inside(polygon_tree_item) {
                return;
            }
        }
        self.heads.push(polygon_tree_item.clone());
    }

    pub fn remove_inner_of_same_cut(&mut self) -> Vec<usize> {
        let mut ids = Vec::new();

        let mut i = 0;
        while i < self.heads.len() {
            if self.heads[i].can_cut_inside == self.can_cut_inside {
                let mut polys : Vec<PolygonTreeItem<T>> = self.heads
                    .swap_remove(i)
                    .inner_polygons
                    .iter()
                    .map(|x| *x.clone())
                    .collect();
                self.heads.append(&mut polys);
                continue;
            }
            self.heads[i].remove_inner_of_same_cut(&mut ids);
            i+=1;
        }

        return ids;
    }

    pub fn total_can_cuts(&self) -> (u32, u32) {
        let mut can = 0;
        let mut not = 0;

        for head in &self.heads {
            if head.can_cut_inside {
                can += 1;
            } else {
                not += 1;
            }
        }

        (can, not)
    }

    pub fn set_outer_can_cut_inside(&mut self, can_cut : bool) {
        self.can_cut_inside = can_cut;
    }
}

impl<T : lines_and_curves::Intersection + Clone> From<(Vec<T>, bool)> for PolygonTreeItem<T> {
    fn from(item: (Vec<T>, bool)) -> Self {
        let bounds = if item.0.len() == 0 {
            lines_and_curves::Rectangle::zero()
        } else {
            let mut bounds = item.0[0].bounding_box();
            for i in 1..item.0.len() {
                bounds = bounds.join(&item.0[i].bounding_box());
            }
            bounds
        };

        Self {
            bounding_box: bounds,
            polygon: item.0,
            inner_polygons: Vec::new(),
            can_cut_inside: item.1,
            id: 0,
        }
    }
}

impl<T : lines_and_curves::Intersection + Clone> PolygonTreeItem<T> {
    fn is_inner_polygon(&self, polygon: &PolygonTreeItem<T>) -> bool {
        if !self.bounding_box.contains_rect(&polygon.bounding_box) {
            return false;
        }

        let inner_point = lines_and_curves::Intersection::find_barely_inner_point(
            &polygon.polygon
        );

        use crate::utils::lines_and_curves::Intersection;
        lines_and_curves::LineSegment::is_inside(
            &self.polygon,
            inner_point.x,
            inner_point.y,
        )
    }

    fn add_polygon_if_inside(&mut self, polygon: &PolygonTreeItem<T>) -> bool {
        if !self.is_inner_polygon(polygon) {
            return false;
        }

        for inner in &mut self.inner_polygons {
            if inner.add_polygon_if_inside(&polygon) {
                return true;
            }
        }

        self.inner_polygons.push(Box::from(polygon.clone()));

        return true;
    }

    fn remove_inner_of_same_cut(&mut self, ids: &mut Vec<usize>) {
        let mut index = 0;
        while index < self.inner_polygons.len() {
            let inner = &self.inner_polygons[index];
            if inner.can_cut_inside == self.can_cut_inside {
                let inner = self.inner_polygons[index].clone();
                self.inner_polygons.extend(inner.inner_polygons);
                ids.push(inner.id);
                self.inner_polygons.swap_remove(index);
                continue;
            }

            self.inner_polygons[index].remove_inner_of_same_cut(ids);
            index+=1;
        }
    }
}

