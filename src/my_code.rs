mod config;
mod lsystem;
mod misc;

use config::AppConfig;
use lsystem::{LsystemBuilder, LsystemTree};
use misc::hex_to_rgb;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct BranchInfo {
    pub id: usize,
    pub i_on_start: usize,
}

impl BranchInfo {
    fn new(id: usize, i_on_start: usize) -> BranchInfo {
        BranchInfo { id, i_on_start }
    }
}

struct Model {
    pub previous_index: usize,
    pub progress_i: usize,
    pub groth_speed: usize,
    pub app_config: AppConfig,
    pub trees: Vec<LsystemTree>,
    pub branches_to_animate_current: Vec<BranchInfo>,
}

fn model(_app: &App) -> Model {
    let deeps = vec![8, 8];
    let app_config = AppConfig::new();
    let lsystem_builder = LsystemBuilder::new(&app_config.config.clone());
    let mut trees = vec![];
    deeps.iter().for_each(|deep| {
        trees.push(lsystem_builder.build_tree(&deep));
    });

    let groth_speed = 10;

    let delta = app_config.start_point_delta.unwrap_or(pt2(200.0, 0.0));
    let start_point = app_config.start_point.unwrap_or_else(|| {
        if deeps.len() == 1 {
            pt2(0.0, -200.0)
        } else {
            pt2(-100.0, -200.0)
        }
    });

    for (i, tree) in trees.iter_mut().enumerate() {
        tree.move_tree(start_point + delta * i as f32);
    }

    let branches_to_animate_current = vec![
        BranchInfo::new(0, 0),
        BranchInfo::new(1, 0),
        BranchInfo::new(2, 0),
    ];
    Model {
        previous_index: 0,
        progress_i: 0,
        groth_speed,
        branches_to_animate_current,
        app_config,
        trees,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.previous_index = model.progress_i;
    model.progress_i += model.groth_speed;

    let tree = model.trees[1].clone();
    let mut new_branches = vec![];

    for branch_info in model.branches_to_animate_current.iter() {
        if let Some(branch_dots) = tree.branches.get(&branch_info.id) {
            let to_index = (model.progress_i - branch_info.i_on_start).min(branch_dots.len());

            for dot in branch_dots[..to_index].iter() {
                if !dot.connected_branches_id.is_empty() {
                    for branch_id in dot.connected_branches_id.iter() {
                        new_branches.push(BranchInfo::new(*branch_id, model.progress_i));
                    }
                }
            }
        }
    }

    model.branches_to_animate_current.extend(new_branches);

    println!(
        "index:{}, prev_i:{}    -   {:?}",
        model.progress_i, model.previous_index, model.branches_to_animate_current
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background()
        .color(hex_to_rgb(&model.app_config.bg_color));

    // Draw the main tree
    draw.polyline()
        .weight(model.app_config.config.line_weight)
        .points(model.trees[0].dots.iter().cloned())
        .color(hex_to_rgb(&model.app_config.config.main_color));

    // Draw the animated branches
    for branch_info in model.branches_to_animate_current.iter() {
        if let Some(branch) = model.trees[1].branches.get(&branch_info.id) {
            let to_index = (model.progress_i - branch_info.i_on_start).min(branch.len());
            let dots = branch[..to_index]
                .iter()
                .map(|br_dot| br_dot.pos)
                .collect::<Vec<Point2>>();

            draw.polyline()
                .weight(model.app_config.config.line_weight)
                .points(dots.iter().cloned())
                .color(hex_to_rgb(&model.app_config.config.main_color));
        }
    }

    draw.to_frame(app, &frame).unwrap();
    println!("-2");
}
