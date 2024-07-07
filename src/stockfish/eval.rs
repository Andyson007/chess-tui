#[derive(Debug, Copy, Clone)]
pub struct Eval {
    length: Option<usize>,
}

impl Default for Eval {
    fn default() -> Self {
        Self {
            length: Some(0),
        }
    }
}

impl Eval {
    pub fn parse(data: &str) -> Self {
        Self {
            length: Some(0),
        }
    }
}
