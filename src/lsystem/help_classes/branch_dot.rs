use nannou::geom::Point2;

pub struct BranchDot {
    pub pos: Point2,
    pub connected_branches_id: Vec<usize>,
}
