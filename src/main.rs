mod config;
mod lsystem;
mod misc;

use config::AppConfig;
use lsystem::{LsystemBuilder, LsystemTree};
use misc::{debug_info, hex_to_rgb};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    pub index: usize,
}

fn model(_app: &App) -> Model {
    Model { index: 0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.index += 30;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // if len == 1, then the tree will be in the center bottom of the screen
    // else, the trees will be planted from left bottom to right bottom with 50 distance between them
    let deeps = vec![8, 8];
    let app_config = AppConfig::new();

    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();

    draw.background().color(hex_to_rgb(&app_config.bg_color));

    let mut start_point = app_config
        .start_point
        .unwrap_or(pt2(win.pad(100.0).left(), win.bottom()));
    if deeps.len() == 1 {
        start_point = pt2(0.0, win.bottom());
    }

    let delta = app_config.start_point_delta.unwrap_or(pt2(100.0, 0.0));

    let lsystem_builder = LsystemBuilder::new(&app_config.config.clone());
    let mut trees = vec![];

    deeps.iter().for_each(|deep| {
        trees.push(lsystem_builder.build_tree(&deep));
    });

    trees[0].move_tree(start_point);
    draw_branch(&trees[0].dots[..], &app_config, &draw);

    let tree = &mut trees[1];
    tree.move_tree(start_point + delta);
    // 0 is the main branch
    let mut picked_started_points: Vec<usize> = vec![0];
    // check if the dot in the branches start and then add it to tbranches to draw
    let anim_progress = model.index.min(tree.dots.len());
    for branch_start in picked_started_points {
        let branch_end = tree.branches.get(&branch_start).unwrap();
        let draw_dot = (branch_start + anim_progress).min(*branch_end);
        draw_branch(&tree.dots[branch_start..draw_dot], &app_config, &draw);
    }

    // for (i, tree) in trees.iter_mut().enumerate() {
    //     let mut branches_to_draw: Vec<usize> = vec![0];
    //     tree.move_tree(start_point + delta * i as f32);
    //     let anim_progress = model.index.min(tree.dots.len());
    //     draw_branch(&tree.dots[0..anim_progress], &app_config, &draw);
    // }

    // debug_info(&draw, win, &app_config.config);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_branch(branch: &[Point2], app_config: &AppConfig, draw: &Draw) {
    draw.polyline()
        .weight(app_config.config.line_weight)
        .points(branch.iter().cloned())
        .color(hex_to_rgb(&app_config.config.main_color));
}
