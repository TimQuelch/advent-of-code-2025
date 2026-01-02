use std::collections::BinaryHeap;

use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|bank| {
            let bytes = bank.as_bytes();
            let first_rev_pos = bytes[..bytes.len() - 1]
                .iter()
                .rev()
                .position_max()
                .unwrap()
                + 1;
            let first_pos = bytes.len() - first_rev_pos - 1;
            let first = bytes[first_pos];
            let second = bytes[first_pos + 1..].iter().max().unwrap();
            10 * u32::from(first - b'0') + u32::from(second - b'0')
        })
        .sum::<u32>()
        .into()
}

pub fn part2(input: &str) -> i64 {
    const N: usize = 12;
    let mut heap = BinaryHeap::new();
    let mut tail = Vec::with_capacity(N);
    input
        .lines()
        .map(|bank_str| {
            heap.clear();
            tail.clear();

            let costs = bank_str
                .bytes()
                .rev()
                .enumerate()
                .map(|(i, x)| (x - b'0', i));

            tail.extend(costs.clone().take(N));
            heap.extend(costs.skip(N));

            (0..N)
                .map(|i| {
                    heap.push(tail.pop().unwrap());
                    let (val, offset) = heap.pop().unwrap();
                    heap.retain(|&(_, i)| i < offset);
                    10u64.pow(u32::try_from(N - i).unwrap() - 1) * u64::from(val)
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 357);
    }

    #[test]
    fn p2_1() {
        let result = part2("987654321111111");
        assert_eq!(result, 987654321111);
    }

    #[test]
    fn p2_2() {
        let result = part2("811111111111119");
        assert_eq!(result, 811111111119);
    }

    #[test]
    fn p2_3() {
        let result = part2("234234234234278");
        assert_eq!(result, 434234234278);
    }

    #[test]
    fn p2_4() {
        let result = part2("818181911112111");
        assert_eq!(result, 888911112111);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 3121910778619);
    }
}
