mod config;
mod constants;
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
    fn main_branch() -> BranchInfo {
        BranchInfo {
            id: 0,
            i_on_start: 0,
        }
    }
}

struct Model {
    pub previous_i: usize,
    pub progress_i: usize,
    pub dots_pro_seconds: usize,
    pub app_config: AppConfig,
    pub trees: Vec<LsystemTree>,
    pub branches_to_animate_current: Vec<BranchInfo>,
}

fn model(_app: &App) -> Model {
    // creating the default configs
    let app_config = AppConfig::new(vec![8]);
    let lsystem_builder = LsystemBuilder::new(&app_config.config.clone());

    // building the trees
    let mut trees = vec![];
    app_config.deeps.iter().for_each(|deep| {
        trees.push(lsystem_builder.build_tree(&deep));
    });

    // finding the values for moving the trees on the canvas (for rendering)
    let delta = app_config.start_point_delta.unwrap_or(pt2(200.0, 0.0));
    let start_point = app_config.start_point.unwrap_or_else(|| {
        if app_config.deeps.len() == 1 {
            pt2(0.0, -200.0)
        } else {
            pt2(-100.0, -200.0)
        }
    });

    // moving the trees to right positions
    for (i, tree) in trees.iter_mut().enumerate() {
        tree.move_tree(start_point + delta * i as f32);
    }

    Model {
        branches_to_animate_current: vec![BranchInfo::main_branch()],
        dots_pro_seconds: app_config.dots_pro_second,
        previous_i: 0,
        progress_i: 0,
        app_config,
        trees,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // chenging the indexes of the animation progress
    model.previous_i = model.progress_i;
    model.progress_i += model.dots_pro_seconds;

    let mut new_branches = vec![];

    // we are searching for a new branch in all branches that are on the way
    for branch_info in model.branches_to_animate_current.iter() {
        if let Some(branch_dots) = model.trees[0].branches.get(&branch_info.id) {
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
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background()
        .color(hex_to_rgb(&model.app_config.bg_color));

    // draw_tree(&model.trees[0], &draw, model);
    draw_tree_animated(&model.trees[0], &draw, model);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_tree(tree: &LsystemTree, draw: &Draw, model: &Model) {
    // Draw the main tree
    draw.polyline()
        .weight(model.app_config.config.line_weight)
        .points(tree.dots.iter().cloned())
        .color(hex_to_rgb(&model.app_config.config.main_color));
}

fn draw_tree_animated(tree: &LsystemTree, draw: &Draw, model: &Model) {
    // Draw the animated branches
    for branch_info in model.branches_to_animate_current.iter() {
        if let Some(branch) = tree.branches.get(&branch_info.id) {
            let to_index = (model.progress_i - branch_info.i_on_start).min(branch.len());
            if to_index != 0 {
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
    }
}
