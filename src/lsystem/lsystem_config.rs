use std::{fmt::Display, fs::File, io::BufReader};

use nannou::geom::Point2;
use serde::{Deserialize, Serialize};

use super::{help_classes::Rules, Rule};

#[derive(Serialize, Deserialize)]
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

impl LsystemConfig {
    pub fn new(
        axiom: &str,
        rules: Vec<Rule>,
        main_color: String,
        start_direction: Point2,
        rotation_factor: f32,
        scale_start: f32,
        scale_delta: f32,
        scale_min: f32,
        line_weight: f32,
        wind_power: Option<Point2>,
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
            line_weight,
            wind_power,
        }
    }

    pub fn read_from_json() -> LsystemConfig {
        let file =
            File::open("config.json").unwrap_or_else(|_| File::create("config.json").unwrap());
        let read_buf = BufReader::new(file);
        serde_json::from_reader(read_buf).unwrap()
    }

    pub fn write_to_json(&self) {
        let file = File::create("config.json").unwrap();
        serde_json::to_writer(file, &self).unwrap();
    }
}
