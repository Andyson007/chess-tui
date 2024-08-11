#[derive(Debug, Copy, Clone)]
pub struct Eval {
    length: usize,
    evals: [f64; 3],
}

impl Default for Eval {
    fn default() -> Self {
        Self {
            length: 0,
            evals: [0.0; 3],
        }
    }
}

impl Eval {
    pub fn parse(data: &str) -> Self {
        eprintln!("{data}");
        Self {
            length: data.lines().count(),
            evals: data
                .lines()
                .rev()
                .take(4)
                .skip(1)
                .inspect(|x| eprintln!("{x}"))
                .map(|x| x.split_whitespace().nth(2))
                .map(|x| x.unwrap().parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
