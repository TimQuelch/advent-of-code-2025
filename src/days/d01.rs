pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .fold((50_i16, 0_u16), |(dial, zero_count), line| {
            let (dir, count_str) = line.split_at(1);
            let count = count_str.parse::<i16>().expect("could not parse count");
            let dial_unwrapped = if dir.as_bytes()[0] == b'R' {
                dial + count
            } else {
                dial - count
            };

            let dial = dial_unwrapped.rem_euclid(100);

            (dial, zero_count + u16::from(dial == 0))
        })
        .1
        .into()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .fold((50_i16, 0_u16), |(dial, zero_count), line| {
            let (dir, count_str) = line.split_at(1);
            let count = count_str.parse::<i16>().expect("could not parse count");
            let dial_unwrapped = if dir.as_bytes()[0] == b'R' {
                dial + count
            } else {
                dial - count
            };

            let div = dial_unwrapped.div_euclid(100);
            let rem = dial_unwrapped.rem_euclid(100);
            let moving_left_to_zero = div <= 0 && rem == 0;
            let moving_left_from_zero = div < 0 && dial == 0;
            let inc =
                div.unsigned_abs() + u16::from(moving_left_to_zero) - u16::from(moving_left_from_zero);

            (rem, zero_count + inc)
        })
        .1
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 3);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 6);
    }

    #[test]
    fn wrapping_r() {
        let result = part2("R1000");
        assert_eq!(result, 10);
    }

    #[test]
    fn wrapping_l() {
        let result = part2("L1000");
        assert_eq!(result, 10);
    }

    #[test]
    fn landing_on_100() {
        let result = part2("R50");
        assert_eq!(result, 1);
    }

    #[test]
    fn landing_on_0() {
        let result = part2("L50");
        assert_eq!(result, 1);
    }

    #[test]
    fn landing_on_200() {
        let result = part2("R150");
        assert_eq!(result, 2);
    }

    #[test]
    fn landing_on_neg_100() {
        let result = part2("L150");
        assert_eq!(result, 2);
    }

    #[test]
    fn landing_on_neg_200() {
        let result = part2("L250");
        assert_eq!(result, 3);
    }

    #[test]
    fn to_zero_then_99() {
        let result = part2("L50\nL1");
        assert_eq!(result, 1);
    }
}
