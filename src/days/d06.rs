use itertools::Itertools;
use ndarray::{Array2, Array3};
use regex::Regex;
use std::iter::{self, zip};

#[derive(Debug, Copy, Clone)]
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

impl From<u8> for Op {
    fn from(x: u8) -> Self {
        match x {
            b'+' => Op::Add,
            b'*' => Op::Mul,
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

pub fn part2(input: &str) -> i64 {
    let ops_str = input.trim_end_matches('\n').rsplit_once('\n').unwrap().1;
    let ops_bytes = ops_str.as_bytes();
    let ops: Vec<(Op, u8)> = Regex::new(r"[*+]\s+")
        .unwrap()
        .find_iter(ops_str)
        .map(|m| {
            // Need to subtract one off all values except the last one to account for the separator
            // between columns
            let len = if m.end() == ops_str.len() {
                m.len()
            } else {
                m.len() - 1
            };
            (ops_bytes[m.start()].into(), len.try_into().unwrap())
        })
        .collect();

    // Do some array indexing black magic to get the values in the correct order. Essentially we are
    // swapping around the order of the axes from (row, column, digit) to (column, row, digit). This
    // means that the digits of each number are contiguous
    let vals = {
        let ncols = ops.len();
        let nrows = input.trim().lines().count() - 1;
        let max_width = ops.iter().max_by_key(|&&(_, w)| w).unwrap().1 as usize;

        // Construct the vector in same order as input string
        let mut raw = Vec::with_capacity(ncols * nrows * max_width);
        for line in input.lines().take(nrows).map(str::as_bytes) {
            let mut offset: usize = 0;
            for w in ops.iter().map(|&(_, w)| w as usize) {
                raw.extend_from_slice(&line[offset..(offset + w)]);
                raw.extend(iter::repeat_n(b' ', max_width - w)); // Pad columns to the same width
                offset += w + 1; // Additional increment to skip space separator
            }
        }

        // Create array from raw vec in same order
        let raw_array = Array3::from_shape_vec((nrows, ncols, max_width), raw).unwrap();

        // Permute axes into desired order and create final array
        Array3::from_shape_vec(
            (ncols, max_width, nrows),
            raw_array.permuted_axes([1, 2, 0]).iter().copied().collect(),
        )
        .unwrap()
    };

    zip(vals.outer_iter(), ops.iter().map(|&(op, _)| op))
        .map(|(ns, op)| -> u64 {
            // Filter out values which error on parsing. These values are digit-columns full of
            // spaces where the columns have been padded out
            let n_iter = ns.outer_iter().filter_map(|n| {
                str::from_utf8(n.as_slice().unwrap())
                    .unwrap()
                    .trim()
                    .parse::<u64>()
                    .ok()
            });
            match op {
                Op::Add => n_iter.sum(),
                Op::Mul => n_iter.product(),
            }
        })
        .sum::<u64>()
        .try_into()
        .unwrap()
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
        let result = part1(EXAMPLE.trim_start());
        assert_eq!(result, 4277556);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim_start());
        assert_eq!(result, 3263827);
    }
}
