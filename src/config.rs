use nannou::{geom::Point2, glam::Vec2};

use crate::{
    constants::*,
    lsystem::{Behaviour, LsystemConfig, Rule, Rules},
};

const DEGREES_IN_RAD: f32 = 0.01745329;

#[derive(Clone)]
pub struct AppConfig {
    pub bg_color: String,
    pub dots_pro_second: usize,

    pub start_point: Option<Point2>,
    pub start_point_delta: Option<Point2>,

    pub deeps: Vec<usize>,
    pub config: LsystemConfig,
}

impl AppConfig {
    pub fn new(deeps: Vec<usize>) -> AppConfig {
        // todo put deeps and rules settings in config somehow
        // if you want not to delete thw symbol, you need to write itself in the Rule
        let rules = vec![
            Rule::new('X', "F+[[X]-X]-F[-FX]+X", Behaviour::DrawForward),
            Rule::new('F', "FF", Behaviour::DrawForward),
            Rule::new('+', "+", Behaviour::RotateLeft),
            Rule::new('-', "-", Behaviour::RotateRight),
            Rule::new('[', "[", Behaviour::Branch),
            Rule::new(']', "]", Behaviour::BranchStop),
        ];

        let wind_power = if let Some(temp) = WIND_POWER {
            Some(Vec2::from(temp))
        } else {
            None
        };

        let config = LsystemConfig {
            axiom: AXIOM.to_string(),
            rules: Rules::new(rules),

            main_color: FG_COLOR.to_string(),
            line_weight: LINE_WEIGHT,

            start_direction: Vec2::from(START_DIRECTION),
            rotation_factor: ROTATION_DEGREES * DEGREES_IN_RAD,

            wind_power,

            scale_delta: SCALE_DELTA,
            scale_start: SCALE_START,
            scale_min: SCALE_MIN,
        };

        let start_point = if let Some(temp) = START_POINT {
            Some(Vec2::from(temp))
        } else {
            None
        };
        let start_point_delta = if let Some(temp) = START_POINT_DELTA {
            Some(Vec2::from(temp))
        } else {
            None
        };

        AppConfig {
            bg_color: BG_COLOR.to_string(),
            dots_pro_second: DOTS_PRO_SECOND,
            start_point,
            start_point_delta,
            deeps,
            config,
        }
    }
}
