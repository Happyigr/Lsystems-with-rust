use nannou::{color::Rgb, geom::Point2};

#[derive(Clone)]
pub struct LsystemDots {
    dots: Vec<(Point2, Rgb)>,
    current: usize,
}

impl Iterator for LsystemDots {
    type Item = (Point2, Rgb);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.dots.len() {
            let item = self.dots[self.current].clone(); // Clone the item to return
            self.current += 1; // Move to the next position
            Some(item)
        } else {
            None // No more items to return
        }
    }
}

impl LsystemDots {
    pub fn new(dots: Vec<(Point2, Rgb)>) -> LsystemDots {
        LsystemDots { dots, current: 0 }
    }

    pub fn move_dots(&mut self, delta: Point2) {
        for (pos, _) in self.dots.iter_mut() {
            *pos += delta;
        }
    }
}
