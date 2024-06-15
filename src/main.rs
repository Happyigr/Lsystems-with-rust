mod config;
mod lsystem;
mod misc;

use std::collections::HashSet;

use config::AppConfig;
use lsystem::{HashDot, LsystemBuilder, LsystemTree};
use misc::hex_to_rgb;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct BranchInfo {
    pub start_dot: HashDot,
    pub i_on_start: usize,
}

impl BranchInfo {
    fn new(start_dot: HashDot, i_on_start: usize) -> BranchInfo {
        BranchInfo {
            start_dot,
            i_on_start,
        }
    }
}

struct Model {
    pub previous_index: usize,
    pub progress_i: usize,
    pub groth_speed: usize,
    pub app_config: AppConfig,
    pub trees: Vec<LsystemTree>,
    pub branches_to_animate: HashSet<BranchInfo>,
}

fn model(_app: &App) -> Model {
    let deeps = vec![8, 8];
    let app_config = AppConfig::new();
    let lsystem_builder = LsystemBuilder::new(&app_config.config.clone());
    // if len == 1, then the tree will be in the center bottom of the screen
    // else, the trees will be planted from left bottom to right bottom with 50 distance between them
    let mut trees = vec![];
    deeps.iter().for_each(|deep| {
        trees.push(lsystem_builder.build_tree(&deep));
    });

    let groth_speed = 10;

    let delta = app_config.start_point_delta.unwrap_or(pt2(200.0, 0.0));
    let start_point = app_config.start_point.unwrap_or_else(|| {
        if deeps.len() == 1 {
            return pt2(0.0, -200.0);
        } else {
            return pt2(-100.0, -200.0);
        }
    });

    for (i, tree) in trees.iter_mut().enumerate() {
        tree.move_tree(start_point + delta * i as f32);
    }

    // 0,0 is the main branch
    let branches_to_animate = HashSet::new();
    Model {
        previous_index: 0,
        progress_i: 0,
        groth_speed,
        branches_to_animate,
        app_config,
        trees,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.previous_index = model.progress_i;
    model.progress_i += model.groth_speed;

    let activated_dots = &model.trees[1].dots[model.previous_index..model.progress_i];

    for dot in activated_dots {
        if let Some(_) = model.trees[1].start_point_to_branch.get(&HashDot(*dot)) {
            model
                .branches_to_animate
                .insert(BranchInfo::new(HashDot(*dot), model.progress_i));
        }
    }
    println!(
        "index:{}, prev_i:{}\n{:?}",
        model.progress_i, model.previous_index, model.branches_to_animate
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background()
        .color(hex_to_rgb(&model.app_config.bg_color));

    draw.polyline()
        .weight(model.app_config.config.line_weight)
        .points(
            model.trees[0].dots[..model.trees[0].dots.len()]
                .iter()
                .cloned(),
        )
        .color(hex_to_rgb(&model.app_config.config.main_color));

    // check if the dot in the branches start and then add it to tbranches to draw
    for branch_info in model.branches_to_animate.iter() {
        let branch = model.trees[1]
            .start_point_to_branch
            .get(&branch_info.start_dot)
            .unwrap();
        let to_index = (model.progress_i - branch_info.i_on_start).min(branch.len());
        draw.polyline()
            .weight(model.app_config.config.line_weight)
            .points(branch[..to_index].iter().cloned())
            .color(hex_to_rgb(&model.app_config.config.main_color));
    }

    // debug_info(&draw, win, &app_config.config);

    draw.to_frame(app, &frame).unwrap();
}
