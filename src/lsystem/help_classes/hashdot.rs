use std::{
    hash::{Hash, Hasher},
    ops::Add,
};

use nannou::geom::{pt2, Point2};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct HashDot(pub Point2);

impl Eq for HashDot {}

impl Hash for HashDot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash both x and y components of the Point2
        let point = self.0;
        point.x.to_bits().hash(state);
        point.y.to_bits().hash(state);
    }
}

impl Add for HashDot {
    type Output = HashDot;

    fn add(self, rhs: HashDot) -> Self::Output {
        HashDot(pt2(self.0.x + rhs.0.x, self.0.y + rhs.0.y))
    }
}
