use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Behaviour {
    DrawForward,
    RotateLeft,
    RotateRight,
    Branch,
    BranchStop,
}

impl Display for Behaviour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Behaviour::DrawForward => "Create",
            Behaviour::RotateLeft => "RotateLeft",
            Behaviour::RotateRight => "RotateRight",
            Behaviour::Branch => "Branch",
            Behaviour::BranchStop => "BranchStop",
        };

        write!(f, "{}", text)
    }
}
