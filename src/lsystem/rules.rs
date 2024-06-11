use std::{collections::HashMap, fmt::Display};

use super::{rule::Rule, Behaviour};

pub struct Rules(HashMap<char, (String, Behaviour)>);

impl Rules {
    pub fn new(rules: Vec<Rule>) -> Rules {
        Rules(
            rules
                .into_iter()
                .map(|rule| (rule.0, (rule.1, rule.2)))
                .collect::<HashMap<char, (String, Behaviour)>>(),
        )
    }

    pub fn get_text(&self, ch: &char) -> Option<&String> {
        if let Some((text, _)) = self.0.get(&ch) {
            return Some(text);
        } else {
            return None;
        }
    }

    pub fn get_behaviour(&self, ch: &char) -> Option<&Behaviour> {
        if let Some((_, beh)) = self.0.get(&ch) {
            return Some(beh);
        } else {
            return None;
        }
    }
}

impl Display for Rules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rule in self.0.iter() {
            writeln!(f, "{} -> {}. Behaviour: {}", rule.0, rule.1 .0, rule.1 .1).unwrap()
        }
        Ok(())
    }
}
