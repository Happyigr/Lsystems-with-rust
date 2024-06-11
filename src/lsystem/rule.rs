use std::fmt::Display;

use super::Behaviour;

// a rule that includes the start string and the end string
// rule.0 -> rule.1
pub struct Rule(pub char, pub String, pub Behaviour);

impl Rule {
    pub fn new(from: char, to: &str, behaviour: Behaviour) -> Rule {
        Rule(from, to.to_string(), behaviour)
    }
}
impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}
