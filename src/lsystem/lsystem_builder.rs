use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use super::{
    help_classes::{BranchDot, Rules},
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

struct HashDot {
    pos: Point2,
}
impl Hash for HashDot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash both x and y coordinates to uniquely identify the point
        self.pos.x.to_bits().hash(state);
        self.pos.y.to_bits().hash(state);
    }
}
impl Eq for HashDot {}

impl PartialEq for HashDot {
    fn eq(&self, other: &Self) -> bool {
        self.pos.x == other.pos.x && self.pos.y == other.pos.y
    }
}

// help struct for generating lsystem tree
struct DotData {
    pos: Point2,
    dir: Point2,
    scale: f32,
}

impl DotData {
    fn new(pos: Point2, dir: Point2, scale: f32) -> DotData {
        DotData { pos, dir, scale }
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

    // encodes the given lsystem string in the 2D points friom startpoint = (0.0,0.0)
    fn lsystem_to_tree(&self, lsystem: &String) -> LsystemTree {
        let startpoint = pt2(0.0, 0.0);
        // todo multiple colors

        let mut res = vec![startpoint];
        let mut branches: HashMap<usize, Vec<BranchDot>> = HashMap::new();

        let mut dot = DotData::new(startpoint, self.start_direction, self.scale_start);
        let mut fork_dots: Vec<DotData> = vec![];

        let mut current_dots: Vec<BranchDot> = vec![];

        let mut queued_branches: Vec<Vec<BranchDot>> = vec![];
        let mut queued_branches_id: Vec<usize> = vec![];
        let mut dir_changed = false;

        let mut last_created = 0;
        let mut current_branch_id = 0;

        // todo think about graph as the tree structure
        let mut res_cutted = vec![startpoint];
        let mut branches_cutted: HashMap<usize, Vec<BranchDot>> = HashMap::new();
        let mut fork_dots_cutted: HashMap<HashDot, Vec<usize>> = HashMap::new();
        let mut current_dots_cutted: Vec<BranchDot> = vec![];
        let mut queued_branches_cutted: Vec<Vec<BranchDot>> = vec![];
        let mut queued_branches_id_cutted: Vec<usize> = vec![];
        let mut last_created_cutted = 0;
        let mut current_branch_id_cutted = 0;

        for ch in lsystem.chars() {
            if let Some(beh) = self.rules.get_behaviour(&ch) {
                match beh {
                    Behaviour::DrawForward => {
                        dot.pos += dot.dir * dot.scale;
                        dot.scale = self.scale_min.max(dot.scale + self.scale_delta);
                        res.push(dot.pos);

                        // for branches
                        let branch_dot = BranchDot {
                            pos: dot.pos,
                            connected_branches_id: vec![],
                        };
                        current_dots.push(branch_dot.clone());
                        if dir_changed {
                            res_cutted.push(dot.pos);
                            current_dots_cutted.push(branch_dot.clone());
                            dir_changed = false;
                        }
                    }
                    Behaviour::RotateLeft => {
                        dir_changed = true;
                        dot.dir = dot.dir.rotate(self.rotation_factor);
                    }
                    Behaviour::RotateRight => {
                        dir_changed = true;
                        dot.dir = dot.dir.rotate(-self.rotation_factor);
                    }

                    // on branching push the current dots in the previos branch and start a new uniqe branch
                    Behaviour::Branch => {
                        fork_dots.push(DotData::new(dot.pos, dot.dir, dot.scale));

                        if current_dots.len() == 0 {
                            let mut i = queued_branches.len() - 1;
                            let mut temp = queued_branches.get_mut(i).unwrap();
                            while temp.len() == 0 {
                                i -= 1;
                                temp = queued_branches.get_mut(i).unwrap();
                            }
                            temp.last_mut()
                                .unwrap()
                                .connected_branches_id
                                .push(last_created + 1);
                        } else {
                            current_dots
                                .last_mut()
                                .unwrap()
                                .connected_branches_id
                                .push(last_created + 1);
                        }

                        // queue the current branch
                        queued_branches.push(current_dots);
                        queued_branches_id.push(current_branch_id);

                        // create a new one
                        current_dots = vec![];
                        last_created += 1;
                        current_branch_id = last_created;

                        // todo in res_cutted only the dots, that are new branching or new
                        // direction
                        // this is the fork_dots to branches hashmap, at the end change the dots in
                        // the res_cutted so, that they are connected to those branches :)
                        if let Some(connected_branches) =
                            fork_dots_cutted.get_mut(&HashDot { pos: dot.pos })
                        {
                            connected_branches.push(last_created_cutted + 1);
                        } else {
                            fork_dots_cutted
                                .insert(HashDot { pos: dot.pos }, vec![last_created_cutted + 1]);
                            res_cutted.push(dot.pos);
                        }
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
        branches_cutted.insert(current_branch_id_cutted, current_dots_cutted);

        if let Some(beh) = self.rules.get_behaviour(&lsystem.chars().last().unwrap()) {
            match beh {
                Behaviour::DrawForward => res.push(dot.pos),
                _ => {}
            }
        }

        // adding the connected branch to cutted_dots
        let res_cutted = res_cutted
            .iter_mut()
            .map(|dot| {
                if let Some(branches) = fork_dots_cutted.get(&HashDot { pos: *dot }) {
                    return BranchDot {
                        pos: *dot,
                        connected_branches_id: branches.clone(),
                    };
                } else {
                    return BranchDot {
                        pos: *dot,
                        connected_branches_id: vec![],
                    };
                }
            })
            .collect::<Vec<BranchDot>>();

        LsystemTree {
            dots: res,
            dots_cutted: res_cutted,
            branches,
            branches_cutted,
        }
    }
}
