use nannou::{color::Rgb, geom::Point2, Draw};

use crate::misc::hex_to_rgb;

use super::{lsystem_tree::LsystemTree, LsystemConfig};

// This class is responsible for drawing the LsustemTrees on the Draw struct
pub struct LsystemPainter {
    line_weight: f32,
    color: Rgb,
    wind_power: Option<Point2>,
}

impl LsystemPainter {
    pub fn new(config: &LsystemConfig) -> LsystemPainter {
        LsystemPainter {
            line_weight: config.line_weight,
            color: hex_to_rgb(&config.main_color),
            wind_power: config.wind_power,
        }
    }

    pub fn draw_tree(&self, tree: &LsystemTree, draw: &Draw) {
        draw.polyline()
            .weight(self.line_weight)
            .points(tree.get_uncolored())
            .color(self.color);
    }

    pub fn draw_tree_windy(&mut self, tree: &LsystemTree) {
        unimplemented!()
    }
}
