use itertools::Itertools;
use ndarray::Array2;
use std::iter::zip;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl From<&str> for Op {
    fn from(x: &str) -> Self {
        match x {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("invalid op {x}"),
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let ncols = input.lines().next().unwrap().split_whitespace().count();
    let nrows = input.lines().count() - 1;

    let ops: Vec<Op> = input
        .trim_end()
        .rsplit_once('\n')
        .unwrap()
        .1
        .split_whitespace()
        .map_into()
        .collect();

    let vals = Array2::from_shape_vec(
        (nrows, ncols),
        input
            .lines()
            .take(nrows)
            .flat_map(|l| l.split_whitespace().map(|x| x.parse::<u64>().unwrap()))
            .collect(),
    )
    .unwrap();

    assert_eq!(ops.len(), vals.ncols());

    zip(ops, vals.columns())
        .map(|(op, vals)| -> u64 {
            match op {
                Op::Add => vals.into_iter().sum(),
                Op::Mul => vals.into_iter().product(),
            }
        })
        .sum::<u64>()
        .try_into()
        .unwrap()
}

pub fn part2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 4277556);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 0);
    }
}
