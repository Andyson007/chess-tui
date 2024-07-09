// FIX: This is actually horrible code. It doesn't fully comply with the specs etc. This should be
// rewritten

#[derive(Debug, PartialEq, Eq)]
pub struct Option {
    name: String,
    data: Data,
}

impl Option {
    pub fn parse(raw: &str) -> Self {
        let mut split = raw.split_whitespace();
        assert_eq!(split.next(), Some("option"), "Not parsing an option");
        assert_eq!(
            split.next(),
            Some("name"),
            "The second value wasn't name. This isn't handled yet"
        );
        let values = ["name", "type", "default", "min", "max", "var"];
        let mut name = String::new();
        for curr in split.by_ref() {
            if values.contains(&curr) {
                assert_eq!(curr, "type", "Not handling this yet either");
                break;
            }
            name.push_str(curr);
        }
        Self {
            name,
            data: Data::parse(std::iter::once("type").chain(split)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Data {
    Check { value: bool },
    Spin { value: i64, min: i64, max: i64 },
    Combo { value: String, options: Vec<String> },
    Button,
    String { value: String },
}

impl Data {
    pub fn parse<'a>(raw: impl IntoIterator<Item = &'a str>) -> Self {
        let mut raw = raw.into_iter();
        assert_eq!(Some("type"), raw.next());
        match raw.next().unwrap() {
            "check" => Self::parse_check(raw),
            "spin" => Self::parse_spin(raw),
            "button" => Self::Button,
            "string" => Self::Button,
            x => unimplemented!("{x}"),
        }
    }

    fn parse_check<'a>(rest: impl IntoIterator<Item = &'a str>) -> Self {
        Self::Check {
            value: rest.into_iter().nth(1).unwrap() == "true",
        }
    }

    fn parse_spin<'a>(rest: impl IntoIterator<Item = &'a str>) -> Self {
        let (value, min, max) = rest
            .into_iter()
            .collect::<Vec<&'a str>>()
            .chunks(2)
            .map(|x| (x[0], x[1]))
            .fold((None, None, None), |sum, (a, b)| match a {
                "default" => (Some(b.parse::<i64>().unwrap()), sum.1, sum.2),
                "min" => (sum.0, Some(b.parse::<i64>().unwrap()), sum.2),
                "max" => (sum.0, sum.1, Some(b.parse::<i64>().unwrap())),
                _ => unreachable!(),
            });
        Self::Spin {
            value: value.unwrap(),
            min: min.unwrap(),
            max: max.unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::options::{Data, Option};

    #[test]
    fn parse_options() {
        assert_eq!(
            Option::parse("option name nodestime type spin default 0 min 0 max 10000"),
            Option {
                name: String::from("nodestime"),
                data: Data::Spin {
                    value: 0,
                    min: 0,
                    max: 10000
                }
            }
        );
        // option name Debug Log File type string default
        // option name Threads type spin default 1 min 1 max 1024
        // option name Hash type spin default 16 min 1 max 33554432
        // option name Clear Hash type button
        // option name Ponder type check default false
        // option name MultiPV type spin default 1 min 1 max 500
        // option name Skill Level type spin default 20 min 0 max 20
        // option name Move Overhead type spin default 10 min 0 max 5000
        // option name Slow Mover type spin default 100 min 10 max 1000
        // option name nodestime type spin default 0 min 0 max 10000
        // option name UCI_Chess960 type check default false
        // option name UCI_AnalyseMode type check default false
        // option name UCI_LimitStrength type check default false
        // option name UCI_Elo type spin default 1320 min 1320 max 3190
        // option name UCI_ShowWDL type check default false
        // option name SyzygyPath type string default <empty>
        // option name SyzygyProbeDepth type spin default 1 min 1 max 100
        // option name Syzygy50MoveRule type check default true
        // option name SyzygyProbeLimit type spin default 7 min 0 max 7
        // option name Use NNUE type check default true
        // option name EvalFile type string default nn-5af11540bbfe.nnue
    }
}
