use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
