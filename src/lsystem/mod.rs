mod help_classes;
mod lsystem_builder;
mod lsystem_config;
mod lsystem_painter;
mod lsystem_tree;

use lsystem_builder::LsystemBuilder;
// todo make the config with json
pub use lsystem_config::LsystemConfig;
use lsystem_painter::LsystemPainter;

// todo make this private
pub use help_classes::{Behaviour, Rule};
use lsystem_tree::LsystemTree;
use nannou::{
    geom::{pt2, Point2},
    Draw,
};

pub struct Lsystems {
    builder: LsystemBuilder,
    painter: LsystemPainter,
    trees: Option<Vec<LsystemTree>>,
    // we can delete it, this is for debugging
    pub config: LsystemConfig,
}

impl Lsystems {
    pub fn new(config: LsystemConfig) -> Lsystems {
        Lsystems {
            builder: LsystemBuilder::new(&config),
            painter: LsystemPainter::new(&config),
            trees: None,
            config,
        }
    }

    pub fn generate(&mut self, lvls: &Vec<usize>) -> Result<(), String> {
        let mut res = vec![];
        for lvl in lvls {
            // todo error handling
            res.push(self.builder.build_tree(lvl));
        }

        self.trees = Some(res);

        Ok(())
    }

    pub fn draw_trees(
        &mut self,
        draw: &Draw,
        start_point: Point2,
        delta: Point2,
    ) -> Result<(), &str> {
        if let Some(trees) = self.trees.as_mut() {
            for (i, tree) in trees.iter_mut().enumerate() {
                tree.move_tree(start_point + pt2(delta.x * i as f32, delta.y * i as f32));
                self.painter.draw_tree(&tree, draw);
            }
        } else {
            return Err("The trees wasn't generated".into());
        }

        Ok(())
    }
}
