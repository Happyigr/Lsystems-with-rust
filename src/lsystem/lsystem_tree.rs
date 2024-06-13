use std::collections::HashMap;

use nannou::geom::Point2;

// struct that has the start point id and the end point id in tree
pub struct Branch {
    pub start: usize,
    pub end: usize,
}

impl Branch {
    pub fn new(branch_start: usize, branch_end: usize) -> Branch {
        Branch {
            start: branch_start,
            end: branch_end,
        }
    }
}

// This structure is used to store and changing the builded LsystemTree
pub struct LsystemTree {
    pub dots: Vec<Point2>,
    pub branches: HashMap<usize, usize>,
    // current: usize,
}

// impl Iterator for LsystemTree {
//     type Item = (Point2, Rgb);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current < self.dots.len() {
//             let item = self.dots[self.current].clone(); // Clone the item to return
//             self.current += 1; // Move to the next position
//             Some(item)
//         } else {
//             None // No more items to return
//         }
//     }
// }

impl LsystemTree {
    pub fn new(dots: Vec<Point2>, branches: Vec<Branch>) -> LsystemTree {
        let branches = branches
            .into_iter()
            .map(|branch| (branch.start, branch.end))
            .collect();
        LsystemTree { dots, branches }
    }

    pub fn move_tree(&mut self, to_point: Point2) {
        for pos in self.dots.iter_mut() {
            *pos += to_point;
        }
    }

    // pub fn get_part_of_tree(&self, from: usize, to: usize) -> Vec<(Point2, Rgb)> {
    //     self.dots[from..to].to_vec()
    // }
    //
    // pub fn get_uncolored_part(&self, from: usize, to: usize) -> Vec<Point2> {
    //     self.get_uncolored()[from..to].to_vec()
    // }
    //
    // pub fn get_uncolored(&self) -> Vec<Point2> {
    //     self.dots
    //         .iter()
    //         .map(|(pos, _)| *pos)
    //         .collect::<Vec<Point2>>()
    // }
}
