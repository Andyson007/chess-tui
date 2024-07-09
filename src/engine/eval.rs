#[derive(Debug, Copy, Clone)]
pub struct Eval {
    #[allow(unused)]
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
        eprintln!("{}", data.lines().nth(1).unwrap());
        Self {
            length: Some(data.lines().count()),
        }
    }
}
