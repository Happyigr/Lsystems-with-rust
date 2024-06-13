mod help_classes;
mod lsystem_builder;
mod lsystem_config;
mod lsystem_painter;
mod lsystem_tree;

// todo make the config with json
pub use help_classes::{Behaviour, Rule};
pub use lsystem_builder::LsystemBuilder;
pub use lsystem_config::LsystemConfig;
pub use lsystem_tree::LsystemTree;

// // todo make this private
// use lsystem_painter::LsystemPainter;
// use nannou::Draw;
//
// pub struct Lsystems {
//     builder: LsystemBuilder,
//     painter: LsystemPainter,
//     // we can delete it, this is for debugging
//     pub config: LsystemConfig,
// }
//
// impl Lsystems {
//     pub fn new(config: LsystemConfig) -> Lsystems {
//         Lsystems {
//             builder: LsystemBuilder::new(&config),
//             painter: LsystemPainter::new(&config),
//             config,
//         }
//     }
//
//     pub fn generate(&mut self, lvls: &Vec<usize>) -> Result<Vec<LsystemTree>, String> {
//         let mut res = vec![];
//         for lvl in lvls {
//             // todo error handling
//             res.push(self.builder.build_tree(lvl));
//         }
//
//         Ok(res)
//     }
//
//     pub fn draw_tree(&self, tree: &LsystemTree, draw: &Draw) {
//         self.painter.draw_tree(&tree, draw);
//     }
// }
