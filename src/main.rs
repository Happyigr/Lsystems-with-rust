mod config;
mod constants;
mod lsystem;
mod misc;

use std::collections::VecDeque;

use config::AppConfig;
use lsystem::{LsystemBuilder, LsystemTree};
use misc::hex_to_rgb;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct BranchInfo {
    id: usize,
    i_on_start: usize,
}

impl BranchInfo {
    fn new(id: usize, i_on_start: usize) -> Self {
        Self { id, i_on_start }
    }

    fn main_branch() -> Self {
        Self {
            id: 0,
            i_on_start: 0,
        }
    }
}

struct Model {
    previous_i: usize,
    progress_i: usize,
    dots_pro_seconds: usize,
    app_config: AppConfig,
    trees: Vec<LsystemTree>,
    branches_to_animate_current: Vec<BranchInfo>,
    drawed_dots: Vec<Point2>,
    // queued_branches with the info and the index on queue
    queued_branches: VecDeque<BranchInfo>,
}

fn model(_app: &App) -> Model {
    let app_config = AppConfig::new(vec![8]);
    let lsystem_builder = LsystemBuilder::new(&app_config.config.clone());
    let mut trees = app_config
        .deeps
        .iter()
        .map(|deep| lsystem_builder.build_tree(deep))
        .collect::<Vec<_>>();

    let delta = app_config.start_point_delta.unwrap_or(pt2(200.0, 0.0));
    let start_point = app_config.start_point.unwrap_or_else(|| {
        if app_config.deeps.len() == 1 {
            pt2(0.0, -200.0)
        } else {
            pt2(-100.0, -200.0)
        }
    });

    trees.iter_mut().enumerate().for_each(|(i, tree)| {
        tree.move_tree(start_point + delta * i as f32);
    });

    Model {
        previous_i: 0,
        progress_i: 0,
        dots_pro_seconds: app_config.dots_pro_second,
        app_config,
        trees,
        branches_to_animate_current: vec![BranchInfo::main_branch()],
        drawed_dots: vec![],
        queued_branches: vec![].into(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.previous_i = model.progress_i;
    model.progress_i += model.dots_pro_seconds;

    let mut new_founded_branches = vec![];

    model.branches_to_animate_current.retain(|branch_info| {
        if let Some(branch_dots) = model.trees[0].branches.get(&branch_info.id) {
            // getting the index in dependence from the index on the start of drawing, but not
            // bigger as the len of the dots itself
            let to_index = (model.progress_i - branch_info.i_on_start).min(branch_dots.len());
            if to_index != branch_dots.len() {
                // checking if in the new drawed dots are some connected branches
                for dot in &branch_dots[to_index - model.dots_pro_seconds..to_index] {
                    for &branch_id in &dot.connected_branches_id {
                        // and push new branches in the branches to temp branches
                        new_founded_branches.push(BranchInfo::new(branch_id, model.progress_i));
                    }
                }
                true
            } else {
                // if we rendered the full branch, we can remove it from current branches
                println!("Removed due to full drawing: {}", branch_info.id);
                false
            }
        } else {
            println!(
                "Removed due to not finding dots in the branch: {}",
                branch_info.id
            );
            false
        }
    });

    // we render only the 10 branches at the time
    while model.branches_to_animate_current.len() != 5000 {
        // we check if there are some queued branches, because they came earlier then the new ones
        if model.queued_branches.len() != 0 {
            let mut temp = model.queued_branches.pop_front().unwrap();
            // we change the start i of them
            temp.i_on_start = model.progress_i;
            model.branches_to_animate_current.push(temp);
        // if there are no queued branches, we push the new one
        } else if new_founded_branches.len() != 0 {
            model
                .branches_to_animate_current
                .push(new_founded_branches.pop().unwrap());
        } else {
            break;
        }
    }

    // if there are some new branches, we dont added to the animation, we put them in the queue
    while new_founded_branches.len() != 0 {
        model
            .queued_branches
            .push_back(new_founded_branches.pop().unwrap())
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background()
        .color(hex_to_rgb(&model.app_config.bg_color));

    draw_tree_animated(&model.trees[0], &draw, model);
    // draw.polyline()
    //     .weight(model.app_config.config.line_weight)
    //     .points(model.drawed_dots.iter().cloned())
    //     .color(hex_to_rgb(&model.app_config.config.main_color));
    draw.to_frame(app, &frame).unwrap();
}

fn draw_tree_animated(tree: &LsystemTree, draw: &Draw, model: &Model) {
    for branch_info in &model.branches_to_animate_current {
        if let Some(branch) = tree.branches.get(&branch_info.id) {
            println!(
                "id: {}, i: {}, i on start: {}, len: {}",
                branch_info.id,
                model.progress_i,
                branch_info.i_on_start,
                branch.len()
            );
            let to_index = (model.progress_i - branch_info.i_on_start).min(branch.len());
            if to_index > 0 {
                let dots = branch[..to_index].iter().map(|br_dot| br_dot.pos);
                draw.polyline()
                    .weight(model.app_config.config.line_weight)
                    .points(dots)
                    .color(hex_to_rgb(&model.app_config.config.main_color));
            }
        }
    }
}
