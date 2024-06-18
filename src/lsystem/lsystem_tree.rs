use std::collections::HashMap;

use nannou::geom::{pt2, Point2};

use super::help_classes::BranchDot;

// This structure is used to store and changing the builded LsystemTree
#[derive(Clone)]
pub struct LsystemTree {
    pub dots: Vec<Point2>,
    pub branches: HashMap<usize, Vec<BranchDot>>,
}

impl LsystemTree {
    pub fn move_tree(&mut self, to_point: Point2) {
        for (_, branch) in self.branches.iter_mut() {
            for dot in branch.iter_mut() {
                dot.pos = pt2(to_point.x + dot.pos.x, to_point.y + dot.pos.y);
            }
        }

        for dot in self.dots.iter_mut() {
            *dot += to_point;
        }
    }
}
