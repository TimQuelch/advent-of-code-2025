use std::collections::HashSet;

pub fn part1(input: &str) -> i64 {
    input
        .trim()
        .split(',')
        .flat_map(|r| {
            let (low_s, high_s) = r.split_once('-').unwrap();

            ((low_s.len())..=high_s.len())
                .filter(|l| l % 2 == 0)
                .map(|l| {
                    let seq_len = l / 2;

                    let seq_low = if l == low_s.len() {
                        let (upper_s, lower_s) = low_s.split_at(seq_len);
                        let upper = upper_s.parse().unwrap();
                        let lower: u64 = lower_s.parse().unwrap();
                        if lower > upper { upper + 1 } else { upper }
                    } else {
                        10u64.pow((seq_len - 1).try_into().unwrap())
                    };
                    let seq_high = if l == high_s.len() {
                        let (upper_s, lower_s) = high_s.split_at(seq_len);
                        let upper = upper_s.parse().unwrap();
                        let lower: u64 = lower_s.parse().unwrap();
                        if upper > lower { upper - 1 } else { upper }
                    } else {
                        10u64.pow(seq_len.try_into().unwrap()) - 1
                    };

                    if seq_high < seq_low {
                        0
                    } else {
                        let subseq_sum = (seq_high - seq_low + 1) * (seq_low + seq_high) / 2;
                        subseq_sum * (10u64.pow(seq_len.try_into().unwrap())) + subseq_sum
                    }
                })
        })
        .sum::<u64>()
        .try_into()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let mut set = HashSet::<u64>::new();

    input
        .trim()
        .split(',')
        .map(|range| {
            let (low_s, high_s) = range.split_once('-').unwrap();
            let low: u64 = low_s.parse().unwrap();
            let high: u64 = high_s.parse().unwrap();

            let iter = (low_s.len()..=high_s.len()).flat_map(|l| {
                (1..=(l / 2))
                    .filter(move |seq_len| l % seq_len == 0)
                    .flat_map(move |seq_len| {
                        let seq_low = if l == low_s.len() {
                            let upper = low_s[..seq_len].parse::<u64>().unwrap();

                            let upper_total: u64 = (0..(l / seq_len))
                                .map(|e| upper * (10u64.pow((e * seq_len).try_into().unwrap())))
                                .sum();

                            if upper_total >= low { upper } else { upper + 1 }
                        } else {
                            10u64.pow((seq_len - 1).try_into().unwrap())
                        };
                        let seq_high = if l == high_s.len() {
                            let upper = high_s[..seq_len].parse::<u64>().unwrap();

                            let upper_total: u64 = (0..(l / seq_len))
                                .map(|e| upper * (10u64.pow((e * seq_len).try_into().unwrap())))
                                .sum();

                            if upper_total <= high {
                                upper
                            } else {
                                upper - 1
                            }
                        } else {
                            10u64.pow((seq_len).try_into().unwrap()) - 1
                        };

                        (seq_low..=seq_high).map(move |seq| {
                            (0..(l / seq_len))
                                .map(|e| seq * (10u64.pow((e * seq_len).try_into().unwrap())))
                                .sum::<u64>()
                        })
                    })
            });

            set.clear();
            set.extend(iter);
            set.iter().sum::<u64>()
        })
        .sum::<u64>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn p1_11_22() {
        let result = part1("11-22");
        assert_eq!(result, 33);
    }

    #[test]
    fn p1_998_1012() {
        let result = part1("998-1012");
        assert_eq!(result, 1010);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn p2_998_1012() {
        let result = part2("998-1012");
        assert_eq!(result, 2009);
    }

    #[test]
    fn p2_565653_565659() {
        let result = part2("565653-565659");
        assert_eq!(result, 565656);
    }

    #[test]
    fn p2_222220_222224() {
        let result = part2("222220-222224");
        assert_eq!(result, 222222);
    }

    #[test]
    fn p2_201950_202021() {
        let result = part2("201950-202121");
        assert_eq!(result, 202020);
    }

    #[test]
    fn p2_446443_446449() {
        let result = part2("446443-446449");
        assert_eq!(result, 446446);
    }
}
