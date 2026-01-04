use itertools::Itertools;

fn parse_line(line: &str) -> (u16, Vec<u16>, Vec<u16>) {
    let (goal_s, rest) = line.split_once("] (").unwrap();
    let (buttons_s, costs_s) = rest.split_once(") {").unwrap();

    let light_goal: u16 =
        goal_s.bytes().skip(1).enumerate().fold(
            0u16,
            |acc, (i, b)| if b == b'#' { acc | (1 << i) } else { acc },
        );

    let buttons: Vec<_> = buttons_s
        .split(") (")
        .map(|button| {
            button
                .split(',')
                .fold(0u16, |acc, i| acc | (1 << i.parse::<u8>().unwrap()))
        })
        .collect();

    let costs: Vec<_> = costs_s
        .trim_end_matches('}')
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    (light_goal, buttons, costs)
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (goal, buttons, _) = parse_line(line);
            for n in 1..(buttons.len()) {
                // TODO: combinations allocates a new Vec on every iteration. I should write my own
                // which does not need to do this
                if buttons
                    .iter()
                    .combinations(n)
                    .any(|c| c.into_iter().copied().reduce(|acc, x| acc ^ x).unwrap() == goal)
                {
                    return n;
                }
            }
            panic!("no match found");
        })
        .sum::<usize>()
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
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 7);
    }

    #[test]
    fn p1_1() {
        let result = part1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(result, 2);
    }

    #[test]
    fn p1_2() {
        let result = part1("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(result, 3);
    }

    #[test]
    fn p1_3() {
        let result = part1("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(result, 2);
    }

    // #[test]
    // fn example_part2() {
    //     let result = part2(EXAMPLE.trim());
    //     assert_eq!(result, 33);
    // }

    // #[test]
    // fn p2_1() {
    //     let result = part1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    //     assert_eq!(result, 10);
    // }

    // #[test]
    // fn p2_2() {
    //     let result = part1("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
    //     assert_eq!(result, 12);
    // }

    // #[test]
    // fn p2_3() {
    //     let result = part1("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
    //     assert_eq!(result, 11);
    // }
}
