mod lsystem;
mod misc;

use std::mem::take;

use lsystem::{lsystems_to_dots, Behaviour, LsystemConfig, LsystemDots, Rule};
use misc::hex_to_rgb;
use nannou::{lyon::geom::euclid::point2, prelude::*};

const DEGREES_IN_RAD: f32 = 0.01745329;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

struct Model {
    bg_color: &'static str,
    // the deep of our lsystem
    start_point: Option<Point2>,
    start_point_delta: Option<f32>,
    line_weight: f32,
    lsystems_dots: Vec<LsystemDots>,
}

fn model(_app: &App) -> Model {
    let fg_color = "#957591";
    let bg_color = "#D1C6AD";
    let line_weight = 1.0;
    // none for stat in bottom
    // let start_point = Some(pt2(0.0, 0.0));
    let start_point = None;
    let start_point_delta = None;

    // if len == 1, then the tree will be in the center bottom of the screen
    // else, the trees will be planted from left bottom to right bottom with 50 distance between them
    let deeps = vec![8];
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
    let rotation_factor = 25.0 * DEGREES_IN_RAD;
    // for scaling in progression
    let scale_start = 1.0;
    let scale_delta = 0.0;
    let scale_min = 0.0;

    let lsystem_config = LsystemConfig::new(
        axiom,
        rules,
        fg_color,
        start_direction,
        rotation_factor,
        scale_start,
        scale_delta,
        scale_min,
    );

    let lsystems = deeps
        .iter()
        .map(|deep| return lsystem_config.generate(deep))
        .collect::<Vec<String>>();

    let lsystems_dots = lsystems_to_dots(&lsystems, &lsystem_config);

    Model {
        lsystems_dots,
        bg_color,
        line_weight,
        start_point,
        start_point_delta,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();

    draw.background().color(hex_to_rgb(model.bg_color));

    // debug_info(&draw, win, model);

    let mut start_point = model
        .start_point
        .unwrap_or(pt2(win.pad(20.0).left(), win.bottom()));
    if model.lsystems_dots.len() == 1 {
        start_point = pt2(0.0, win.bottom());
    }

    let delta = model.start_point_delta.unwrap_or(50.0);

    for (i, dots) in model.lsystems_dots.clone().iter_mut().enumerate() {
        dots.move_dots(start_point + pt2(delta * i as f32, 0.0));
        draw.polyline()
            .weight(model.line_weight)
            .points_colored(dots);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn debug_info(draw: &Draw, win: Rect, lsystem: LsystemConfig) {
    let pad = 6.0;
    draw.text(&format!("Lsystem config\n\n{}", lsystem))
        .h(win.pad(pad).h())
        .w(win.pad(pad).w())
        .line_spacing(pad)
        .font_size(14)
        .align_text_top()
        .left_justify();
}
