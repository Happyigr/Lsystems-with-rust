use std::collections::HashMap;

use super::{
    help_classes::{HashDot, Rules},
    lsystem_config::LsystemConfig,
    lsystem_tree::LsystemTree,
    Behaviour,
};
use nannou::{
    geom::{pt2, Point2},
    math::Vec2Rotate,
};

// class lsystem, that have the start string and a list of rules from Vec<Rule>
pub struct LsystemBuilder {
    // main things
    axiom: String,
    rules: Rules,

    // the step with which the dot jumps further
    start_direction: Point2,
    // rotation in radian
    rotation_factor: f32,
    // the scale factor of the groth_step in distance (1 for constant growing, -0.5 for smaller
    // growing on the end of the plant)
    // this factor will be added to the start_direction by growing of our plant
    scale_delta: f32,
    scale_start: f32,
    scale_min: f32,
}

// help struct for generating lsystem tree
struct DotData {
    pos: Point2,
    dir: Point2,
    scale: f32,
    branch_id: usize,
}

impl DotData {
    fn new(pos: Point2, dir: Point2, scale: f32, branch_id: usize) -> DotData {
        DotData {
            pos,
            dir,
            scale,
            branch_id,
        }
    }
}

impl LsystemBuilder {
    pub fn new(config: &LsystemConfig) -> LsystemBuilder {
        LsystemBuilder {
            axiom: config.axiom.clone(),
            rules: config.rules.clone(),
            start_direction: config.start_direction,
            rotation_factor: config.rotation_factor,
            scale_delta: config.scale_delta,
            scale_start: config.scale_start,
            scale_min: config.scale_min,
        }
    }

    // gives a LsystemTree from self with
    pub fn build_tree(&self, lvl: &usize) -> LsystemTree {
        let lsystem = self.generate_sequence(lvl);
        self.lsystem_to_tree(&lsystem)
    }

    // generating new string lsystem to given lvl
    fn generate_sequence(&self, lvl: &usize) -> String {
        // sequence of every lvl
        let mut lvl_sequence = self.axiom.clone();

        for _ in 0..*lvl {
            // changed res
            let mut temp = String::new();

            for ch in lvl_sequence.clone().chars() {
                temp.push_str(
                    self.rules
                        .get_text(&ch)
                        .expect(format!("No rule for {}", ch).as_str()),
                );
            }

            // changing the previous sequence with newer one
            lvl_sequence = temp;
        }

        lvl_sequence
    }

    fn sequence_ro_branches(&self, lsystem: &String) -> HashMap<usize, String> {
        let mut branches_str: HashMap<usize, String> = HashMap::new();

        let mut current_str = String::new();
        let mut current_branch = 0;
        let mut last_created_branch = 0;

        let mut queued_branches = vec![];
        let mut queued_branches_id = vec![];

        for ch in lsystem.chars().into_iter() {
            if let Some(beh) = self.rules.get_behaviour(&ch) {
                match beh {
                    Behaviour::Branch => {
                        // queue the branch
                        queued_branches.push(current_str);
                        queued_branches_id.push(current_branch);

                        last_created_branch += 1;

                        current_str = String::new();
                        current_branch = last_created_branch;
                    }
                    Behaviour::BranchStop => {
                        branches_str.insert(current_branch, current_str);

                        current_str = queued_branches.pop().unwrap();
                        current_branch = queued_branches_id.pop().unwrap();
                    }
                    _ => {
                        current_str.push(ch);
                    }
                }
            }
        }
        branches_str.insert(current_branch, current_str.clone());

        branches_str
    }
    // encodes the given lsystem string in the 2D points friom startpoint = (0.0,0.0)
    fn lsystem_to_tree(&self, lsystem: &String) -> LsystemTree {
        let startpoint = pt2(0.0, 0.0);
        // todo multiple colors

        let mut res = vec![startpoint];
        let mut branches: HashMap<usize, Vec<Point2>> = HashMap::new();
        let mut dot_to_branch: HashMap<HashDot, usize> = HashMap::new();
        let mut dot_id_to_branch: HashMap<usize, usize> = HashMap::new();

        let mut dot = DotData::new(startpoint, self.start_direction, self.scale_start, 0);
        let mut fork_dots: Vec<DotData> = vec![];

        let mut current_dots: Vec<Point2> = vec![];
        let mut queued_branches: Vec<Vec<Point2>> = vec![];
        let mut queued_branches_id: Vec<usize> = vec![];

        let mut last_created = 0;
        let mut current_branch_id = 0;

        for ch in lsystem.chars() {
            if let Some(beh) = self.rules.get_behaviour(&ch) {
                match beh {
                    Behaviour::DrawForward => {
                        dot.pos += dot.dir * dot.scale;
                        dot.scale = self.scale_min.max(dot.scale + self.scale_delta);
                        res.push(dot.pos);

                        // for branches
                        current_dots.push(dot.pos);
                    }
                    Behaviour::RotateLeft => dot.dir = dot.dir.rotate(self.rotation_factor),
                    Behaviour::RotateRight => dot.dir = dot.dir.rotate(-self.rotation_factor),
                    // on branching push the current dots in the previos branch and start a new uniqe branch
                    Behaviour::Branch => {
                        fork_dots.push(DotData::new(
                            dot.pos,
                            dot.dir,
                            dot.scale,
                            current_branch_id,
                        ));

                        // insert the connect dot of the branch
                        let dot_pos = HashDot(dot.pos);
                        dot_to_branch.insert(dot_pos, last_created);
                        // res.len()-1 is the number of the dot in result dots
                        dot_id_to_branch.insert(res.len() - 1, last_created);
                        // queue the current branch
                        queued_branches.push(current_dots);
                        queued_branches_id.push(current_branch_id);

                        // create a new one
                        current_dots = vec![];
                        last_created += 1;
                        current_branch_id = last_created;
                    }
                    Behaviour::BranchStop => {
                        // getting the fork dot info
                        dot = fork_dots.pop().expect("There are to many ] in lsystem");

                        // insert the info about closed branch
                        branches.insert(current_branch_id, current_dots);

                        current_dots = queued_branches.pop().unwrap();
                        current_branch_id = queued_branches_id.pop().unwrap();
                    }
                }
            } else {
                unimplemented!("The meaning of the {ch} char is not implemented");
            }
        }
        branches.insert(current_branch_id, current_dots);

        if let Some(beh) = self.rules.get_behaviour(&lsystem.chars().last().unwrap()) {
            match beh {
                Behaviour::DrawForward => res.push(dot.pos),
                _ => {}
            }
        }

        LsystemTree {
            dots: res,
            branches,
            dot_to_branch,
            dot_id_to_branch,
            start_point_to_branch: HashMap::new(),
        }
    }
}
