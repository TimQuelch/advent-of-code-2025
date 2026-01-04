use std::collections::HashSet;

use itertools::Itertools;

fn parse_positions(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let (xs, ys) = line.split_once(',').unwrap();
            (xs.parse().unwrap(), ys.parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let positions = parse_positions(input);

    positions
        .iter()
        .cartesian_product(positions.iter())
        .map(|(&(x1, y1), &(x2, y2))| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        .max()
        .unwrap()
        .try_into()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let positions = parse_positions(input);
    let positions_set: HashSet<_> = positions.iter().collect();

    // let vertical_edges: BTreeMap<_> = positions.iter().

    let point_in_polygon = |x: (u64, u64)| -> bool {
        if positions_set.contains(&x) {
            return true;
        }

        true
    };

    positions
        .iter()
        .cartesian_product(positions.iter())
        .filter_map(|(&(x1, y1), &(x2, y2))| Some((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)))
        .max()
        .unwrap()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 50);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 24);
    }
}
