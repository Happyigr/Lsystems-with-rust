use nannou::{
    geom::{pt2, Point2},
    math::Vec2Rotate,
};

use crate::misc::hex_to_rgb;

use super::{Behaviour, LsystemConfig, LsystemDots};

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

pub fn lsystems_to_dots(lsystems: &Vec<String>, config: &LsystemConfig) -> Vec<LsystemDots> {
    let mut res: Vec<LsystemDots> = vec![];
    for lsystem in lsystems.iter() {
        res.push(lsystem_to_dots(lsystem, config, pt2(0.0, 0.0)));
    }
    res
}

fn lsystem_to_dots(lsystem: &String, config: &LsystemConfig, startpoint: Point2) -> LsystemDots {
    let mut dots = vec![(startpoint, hex_to_rgb(config.main_color))];

    let current_color = hex_to_rgb(config.main_color);
    let mut dot = DotData::new(startpoint, config.start_direction, config.scale_start);

    let mut branches: Vec<DotData> = vec![];

    for ch in lsystem.chars() {
        if let Some(beh) = config.rules.get_behaviour(&ch) {
            match beh {
                Behaviour::DrawForward => {
                    dot.pos += dot.dir * dot.scale;
                    dot.scale = config.scale_min.max(dot.scale + config.scale_delta);
                    dots.push((dot.pos, current_color));
                }
                Behaviour::RotateLeft => dot.dir = dot.dir.rotate(config.rotation_factor),
                Behaviour::RotateRight => dot.dir = dot.dir.rotate(-config.rotation_factor),
                Behaviour::Branch => branches.push(DotData::new(dot.pos, dot.dir, dot.scale)),
                Behaviour::BranchStop => {
                    dot = branches.pop().expect("There are to many ] in lsystem");
                }
            }
        } else {
            unimplemented!("The meaning of the {ch} char is not implemented");
        }
    }

    if let Some(beh) = config.rules.get_behaviour(&lsystem.chars().last().unwrap()) {
        match beh {
            Behaviour::DrawForward => dots.push((dot.pos, current_color)),
            _ => {}
        }
    }

    LsystemDots::new(dots)
}
