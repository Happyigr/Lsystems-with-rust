mod behaviour;
mod lsystem_dots;
mod misc;
mod rule;
mod rules;

pub use behaviour::Behaviour;
pub use lsystem_dots::LsystemDots;
pub use misc::lsystems_to_dots;
pub use rule::Rule;

use nannou::geom::Point2;
use rules::Rules;
use std::fmt::Display;

// class lsystem, that have the start string and a list of rules from Vec<Rule>
pub struct LsystemConfig {
    axiom: String,
    rules: Rules,

    main_color: &'static str,
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

impl Display for LsystemConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Axiom: {}\nRules:\n{}", self.axiom, self.rules)?;
        writeln!(
            f,
            "Main color:{}\nStart direction:{}\nRotation factor:{}\nScale factor:{}\nMin scale factor:{}",
            self.main_color, self.start_direction, self.rotation_factor, self.scale_delta, self.scale_min)?;
        Ok(())
    }
}

impl LsystemConfig {
    pub fn new(
        axiom: &str,
        rules: Vec<Rule>,
        main_color: &'static str,
        start_direction: Point2,
        rotation_factor: f32,
        scale_start: f32,
        scale_delta: f32,
        scale_min: f32,
    ) -> LsystemConfig {
        LsystemConfig {
            axiom: axiom.to_string(),
            rules: Rules::new(rules),
            main_color,
            start_direction,
            rotation_factor,
            scale_start,
            scale_delta,
            scale_min,
        }
    }

    // generating new sequence to given lvl
    pub fn generate(&self, lvl: &usize) -> String {
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
}
