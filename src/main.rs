mod config;
mod constants;
mod lsystem;
mod misc;

use std::collections::VecDeque;

use config::AppConfig;
use constants::ANIMATE;
use lsystem::{LsystemBuilder, LsystemTree};
use misc::hex_to_rgb;
use nannou::{draw::primitive::Texture, prelude::*};

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
    // the tree has the tree info and the vector of branches, that should be animated and
    // queued branches
    trees: Vec<(LsystemTree, Vec<BranchInfo>, VecDeque<BranchInfo>)>,
    last_drawed_total_dot: Vec<Point2>,
    max_branches: usize,
    animate: bool,
}

fn model(_app: &App) -> Model {
    let deeps = vec![8];
    let app_config = AppConfig::new(deeps.clone());
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

    let trees = trees
        .into_iter()
        .map(|tree| return (tree, vec![BranchInfo::main_branch()], vec![].into()))
        .collect::<Vec<(LsystemTree, Vec<BranchInfo>, VecDeque<BranchInfo>)>>();

    Model {
        previous_i: 0,
        progress_i: 0,
        dots_pro_seconds: app_config.dots_pro_second,
        app_config,
        trees,
        last_drawed_total_dot: vec![],
        max_branches: 5000 / deeps.len(),
        animate: ANIMATE,
    }
}

// todo how to make the drawing not to disappear
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.previous_i = model.progress_i;
    model.progress_i += model.dots_pro_seconds;

    model
        .trees
        .iter_mut()
        .for_each(|(tree, branches_to_animate, queued_branches)| {
            let mut new_founded_branches = vec![];
            branches_to_animate.retain(|branch_info| {
                // updating the new opened branches to animate, if there are some
                if let Some(branch_dots) = tree.branches.get(&branch_info.id) {
                    // getting the index in dependence from the index on the start of drawing, but not
                    // bigger as the len of the dots itself
                    let to_index =
                        (model.progress_i - branch_info.i_on_start).min(branch_dots.len());
                    if to_index != branch_dots.len() {
                        // checking if in the new drawed dots are some connected branches
                        for dot in &branch_dots[to_index - model.dots_pro_seconds..to_index] {
                            model.last_drawed_total_dot.push(dot.pos);
                            for &branch_id in &dot.connected_branches_id {
                                // and push new branches in the branches to temp branches
                                new_founded_branches
                                    .push(BranchInfo::new(branch_id, model.progress_i));
                            }
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            // we render only some number of branches at the time (optimisation)
            while branches_to_animate.len() != model.max_branches {
                // we check if there are some queued branches, because they came earlier then the new ones
                if queued_branches.len() != 0 {
                    let mut temp = queued_branches.pop_front().unwrap();
                    // we change the start i of them
                    temp.i_on_start = model.progress_i;
                    branches_to_animate.push(temp);
                // if there are no queued branches, we push the new one
                } else if new_founded_branches.len() != 0 {
                    branches_to_animate.push(new_founded_branches.pop().unwrap());
                } else {
                    break;
                }
            }

            // if there are some new branches, we dont added to the animation, we put them in the queue
            while new_founded_branches.len() != 0 {
                queued_branches.push_back(new_founded_branches.pop().unwrap())
            }
        });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // let dots = model.trees[0]
    //     .0
    //     .dots_cutted
    //     .iter()
    //     .map(|b_dot| return b_dot.pos)
    //     .collect::<Vec<Point2>>();
    // draw.polyline()
    //     .weight(model.app_config.config.line_weight)
    //     .points(dots[..model.progress_i].iter().cloned())
    //     .color(hex_to_rgb(&model.app_config.config.main_color));

    for tree_info in model.trees.iter() {
        match model.animate {
            true => draw_branches_to_animate(&tree_info.0, &tree_info.1, &draw, model),
            false => draw_full_tree(&tree_info.0, &draw, model),
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_full_tree(tree: &LsystemTree, draw: &Draw, model: &Model) {
    draw.polyline()
        .weight(model.app_config.config.line_weight)
        .points(
            tree.dots_cutted
                .iter()
                .map(|branch_dot| return branch_dot.pos)
                .collect::<Vec<Point2>>()
                .iter()
                .cloned(),
        )
        .color(hex_to_rgb(&model.app_config.config.main_color));
}
fn draw_branches_to_animate(
    tree: &LsystemTree,
    branches_to_animate: &Vec<BranchInfo>,
    draw: &Draw,
    model: &Model,
) {
    for branch_info in branches_to_animate {
        if let Some(branch) = tree.branches.get(&branch_info.id) {
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
