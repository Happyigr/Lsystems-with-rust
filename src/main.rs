mod config;
mod lsystem;
mod misc;

use config::AppConfig;
use lsystem::Lsystems;
use misc::{debug_info, hex_to_rgb};
use nannou::prelude::*;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let app_config = AppConfig::new();
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();

    draw.background().color(hex_to_rgb(&app_config.bg_color));

    let start_point = app_config
        .start_point
        .unwrap_or(pt2(win.pad(20.0).left(), win.bottom()));
    let delta = app_config.start_point_delta.unwrap_or(pt2(100.0, 0.0));

    // if len == 1, then the tree will be in the center bottom of the screen
    // else, the trees will be planted from left bottom to right bottom with 50 distance between them
    let deeps = vec![8, 8];
    let mut lsystem = Lsystems::new(app_config.config);
    debug_info(&draw, win, &lsystem.config);
    lsystem.generate(&deeps).unwrap();
    lsystem.draw_trees(&draw, start_point, delta).unwrap();

    draw.to_frame(app, &frame).unwrap();
}
