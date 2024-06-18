use super::help_classes::Rules;
use nannou::geom::Point2;
use std::fmt::Display;

#[derive(Clone)]
pub struct LsystemConfig {
    // main things
    pub axiom: String,
    pub rules: Rules,

    // for LsystemBuilder
    pub main_color: String,
    // the step with which the dot jumps further
    pub start_direction: Point2,
    // rotation in radian
    pub rotation_factor: f32,
    // the scale factor of the groth_step in distance (1 for constant growing, -0.5 for smaller
    // growing on the end of the plant)

    // this factor will be added to the start_direction by growing of our plant
    pub scale_delta: f32,
    pub scale_start: f32,
    pub scale_min: f32,

    pub line_weight: f32,
    pub wind_power: Option<Point2>,
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
