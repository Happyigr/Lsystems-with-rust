use nannou::geom::{pt2, Point2};

use crate::lsystem::{Behaviour, LsystemConfig, Rule};

const DEGREES_IN_RAD: f32 = 0.01745329;

pub struct AppConfig {
    pub bg_color: String,

    pub start_point: Option<Point2>,
    pub start_point_delta: Option<Point2>,

    pub deeps: Vec<usize>,
    pub config: LsystemConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let bg_color = "#F2F7F2".to_string();
        let fg_color = "#FF9FB2".to_string();
        // none for stat in bottom
        let start_point = None;
        let start_point_delta = None;
        let line_weight = 1.0;
        let wind_power = None;

        let deeps = vec![8, 8];

        let axiom = "X";
        // if you want not to delete thw symbol, you need to write itself in the Rule
        let rules = vec![
            Rule::new('X', "F+[[X]-X]-F[-FX]+X", Behaviour::DrawForward),
            Rule::new('F', "FF", Behaviour::DrawForward),
            Rule::new('+', "+", Behaviour::RotateLeft),
            Rule::new('-', "-", Behaviour::RotateRight),
            Rule::new('[', "[", Behaviour::Branch),
            Rule::new(']', "]", Behaviour::BranchStop),
        ];

        let start_direction = pt2(0.0, 1.0);
        let rotation_factor = 30.0 * DEGREES_IN_RAD;

        // for scaling in progression
        let scale_start = 1.0;
        let scale_delta = 0.00;
        let scale_min = 0.5;

        let config = LsystemConfig::new(
            axiom,
            rules,
            fg_color,
            start_direction,
            rotation_factor,
            scale_start,
            scale_delta,
            scale_min,
            line_weight,
            wind_power,
        );
        AppConfig {
            bg_color,
            start_point,
            start_point_delta,
            deeps,
            config,
        }
    }
}
