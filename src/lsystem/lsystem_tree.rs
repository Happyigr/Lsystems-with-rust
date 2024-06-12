use nannou::{color::Rgb, geom::Point2};

// This structure is used to store and changing the builded LsystemTree
pub struct LsystemTree {
    pub dots: Vec<(Point2, Rgb)>,
    current: usize,
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
    pub fn new(dots: Vec<(Point2, Rgb)>) -> LsystemTree {
        LsystemTree { dots, current: 0 }
    }

    pub fn move_tree(&mut self, delta: Point2) {
        for (pos, _) in self.dots.iter_mut() {
            *pos += delta;
        }
    }

    pub fn get_uncolored(&self) -> Vec<Point2> {
        self.dots
            .iter()
            .map(|(pos, _)| *pos)
            .collect::<Vec<Point2>>()
    }
}
