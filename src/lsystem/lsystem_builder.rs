use std::collections::HashMap;

use crate::misc::hex_to_rgb;

use super::{
    help_classes::Rules,
    lsystem_config::LsystemConfig,
    lsystem_tree::{Branch, LsystemTree},
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
    id: usize,
    pos: Point2,
    dir: Point2,
    scale: f32,
}

impl DotData {
    fn new(id: usize, pos: Point2, dir: Point2, scale: f32) -> DotData {
        DotData {
            id,
            pos,
            dir,
            scale,
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
        self.lsystem_to_dots(&lsystem)
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
    fn lsystem_to_dots(&self, lsystem: &String) -> LsystemTree {
        let startpoint = pt2(0.0, 0.0);
        // todo multiple colors

        let mut dots = vec![startpoint];
        // main branch start on startdot dot and ends on the lsystem.len +1 dot
        let mut branches = vec![Branch::new(0, lsystem.len() + 1)];

        let mut dot = DotData::new(0, startpoint, self.start_direction, self.scale_start);

        let mut fork_dots: Vec<DotData> = vec![];

        for (i, ch) in lsystem.chars().enumerate() {
            // i+1 because the element with 0 id is our startpoint
            // this is the number of the dot in tree vector (the tree is a vector of dots)
            let dot_id = i + 1;
            if let Some(beh) = self.rules.get_behaviour(&ch) {
                match beh {
                    Behaviour::DrawForward => {
                        dot.pos += dot.dir * dot.scale;
                        dot.scale = self.scale_min.max(dot.scale + self.scale_delta);
                        dots.push(dot.pos);
                    }
                    Behaviour::RotateLeft => dot.dir = dot.dir.rotate(self.rotation_factor),
                    Behaviour::RotateRight => dot.dir = dot.dir.rotate(-self.rotation_factor),
                    Behaviour::Branch => {
                        fork_dots.push(DotData::new(dot_id, dot.pos, dot.dir, dot.scale));
                    }
                    Behaviour::BranchStop => {
                        let branch_end = dot_id;
                        dot = fork_dots.pop().expect("There are to many ] in lsystem");
                        let branch_start = dot.id;
                        branches.push(Branch::new(branch_start, branch_end));
                    }
                }
            } else {
                unimplemented!("The meaning of the {ch} char is not implemented");
            }
        }

        if let Some(beh) = self.rules.get_behaviour(&lsystem.chars().last().unwrap()) {
            match beh {
                Behaviour::DrawForward => dots.push(dot.pos),

                _ => {}
            }
        }

        LsystemTree::new(dots, branches)
    }
}
