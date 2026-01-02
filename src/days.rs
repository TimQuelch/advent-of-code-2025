use std::{fs, sync::LazyLock};

pub mod utils;

type PartFn = fn(&str) -> i64;

pub struct Day {
    pub name: String,
    input: String,
    part1_impl: PartFn,
    part2_impl: PartFn,
}

impl Day {
    fn new(name: &str, filename: &str, part1: PartFn, part2: PartFn) -> Self {
        Day {
            name: name.to_string(),
            input: fs::read_to_string(filename).unwrap(),
            part1_impl: part1,
            part2_impl: part2,
        }
    }

    #[must_use]
    pub fn part1(&self) -> i64 {
        (self.part1_impl)(&self.input)
    }

    #[must_use]
    pub fn part2(&self) -> i64 {
        (self.part2_impl)(&self.input)
    }
}

macro_rules! declare_modules {
    ($($day:ident),*) => {
        $(
            #[expect(clippy::must_use_candidate)] // results may be discarded in benchmarks
            pub mod $day;
        )*
    };
}

macro_rules! make_days {
    ($($day:ident),*) => {
        {
            vec![
                $(
                    Day::new(stringify!($day), concat!("data/", stringify!($day), ".txt"), $day::part1, $day::part2),
                )*
            ]
        }
    };
}

declare_modules!(d01, d02, d03, d04, d05);
pub static DAYS: LazyLock<Vec<Day>> = LazyLock::new(|| make_days!(d01, d02, d03, d04, d05));
