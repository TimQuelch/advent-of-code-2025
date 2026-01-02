use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    let (fresh_ranges, available) = input.split_once("\n\n").unwrap();

    let fresh_ranges: Vec<_> = fresh_ranges
        .lines()
        .map(|l| {
            let (low, high) = l.split_once('-').unwrap();
            low.parse::<u64>().unwrap()..=high.parse::<u64>().unwrap()
        })
        .collect();

    available
        .lines()
        .filter(|&l| {
            let x: u64 = l.parse().unwrap();
            fresh_ranges.iter().any(|r| r.contains(&x))
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let (fresh_ranges, _) = input.split_once("\n\n").unwrap();

    let fresh_ranges: Vec<_> = fresh_ranges
        .lines()
        .map(|l| {
            let (low, high) = l.split_once('-').unwrap();
            (low.parse::<u64>().unwrap(), high.parse::<u64>().unwrap())
        })
        .sorted()
        .collect();

    let mut merged: Vec<_> = vec![fresh_ranges[0]];

    for &(l, h) in &fresh_ranges[1..] {
        let (_, last_h) = merged.last_mut().unwrap();
        if l <= *last_h {
            let new_h = h.max(*last_h);
            *last_h = new_h;
        } else {
            merged.push((l, h));
        }
    }

    merged
        .into_iter()
        .map(|(l, h)| h - l + 1)
        .sum::<u64>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 3);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 14);
    }
}
