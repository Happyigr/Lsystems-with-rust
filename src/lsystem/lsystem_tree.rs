use std::collections::HashMap;

use nannou::geom::Point2;

use super::help_classes::HashDot;

// This structure is used to store and changing the builded LsystemTree
pub struct LsystemTree {
    pub dots: Vec<Point2>,
    pub start_point_to_branch: HashMap<HashDot, Vec<Point2>>,
    pub branches: HashMap<usize, Vec<Point2>>,
    pub dot_to_branch: HashMap<HashDot, usize>,
    pub dot_id_to_branch: HashMap<usize, usize>,
}

impl LsystemTree {
    pub fn move_tree(&mut self, to_point: Point2) {
        self.start_point_to_branch = HashMap::new();
        for (_, branch) in self.branches.iter_mut() {
            for pos in branch.iter_mut() {
                *pos += to_point;
            }
            self.start_point_to_branch
                .insert(HashDot(*branch.first().unwrap()), branch.to_vec());
        }
        // let mut temp = HashMap::new();
        // for (dot, branch_id) in self.dot_to_branch.iter() {
        //     let new_dot = *dot + HashDot(to_point);
        //     temp.insert(new_dot, branch_id);
        // }

        for dot in self.dots.iter_mut() {
            *dot += to_point;
        }
    }
}
