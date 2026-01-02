use std::iter::zip;

use ndarray::{Array2, Zip, s};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Val {
    Roll,
    Open,
}

use Val::{Open, Roll};

fn build_grid(input: &str) -> (Array2<Val>, [usize; 2]) {
    let ncols = input.lines().next().unwrap().len();
    let nrows = ((input.len() + 1) / ncols) - 1;

    let mut grid = Array2::from_elem((nrows + 2, ncols + 2), Open);

    for (x, elem) in zip(
        grid.slice_mut(s![1..(1 + nrows), 1..(1 + ncols)]),
        input.chars().filter_map(|c| match c {
            '@' => Some(Roll),
            '.' => Some(Open),
            _ => None,
        }),
    ) {
        *x = elem;
    }

    (grid, [nrows, ncols])
}

pub fn part1(input: &str) -> i64 {
    build_grid(input)
        .0
        .windows((3, 3))
        .into_iter()
        .filter(|&w| w[(1, 1)] == Roll && w.into_iter().filter(|&&x| x == Roll).count() <= 4)
        .count()
        .try_into()
        .unwrap()
}

// TODO: I'm unhappy with performance here. Many passes of large arrays. Possible refactor:
// - Values stored as enum of { Empty, Roll(count) }
// - Iterative passes remove rolls that have counts less than 4, and when removed also decrement the
//   counts of the neighbours
// - This will most likely reduce the number of iterations required.
// - It may be challenging to enforce count invariant. Not quite sure how initialisation would go,
//   maybe with an intermediate array of bools?
// - Possibly could done in a single pass if we recursively remove the rolls. i.e. when a count is
//   reduced below the threshold as part of the removal of another roll, then it also triggers
//   reduction of its neighbours. This may be more complex to implement though
pub fn part2(input: &str) -> i64 {
    let (mut grid, inner_shape) = build_grid(input);
    let mut to_be_removed: Array2<bool> = Array2::default(inner_shape);
    let mut count = 0;
    loop {
        to_be_removed.fill(false);
        Zip::from(grid.windows((3, 3)))
            .and(&mut to_be_removed)
            .for_each(|w, to_remove| {
                if w[(1, 1)] == Roll && w.into_iter().filter(|&&x| x == Roll).count() <= 4 {
                    *to_remove = true;
                }
            });

        let inc_count = to_be_removed
            .iter()
            .filter_map(|&x| x.then_some(()))
            .count();

        count += inc_count;

        if inc_count == 0 {
            return count.try_into().unwrap();
        }

        #[expect(clippy::reversed_empty_ranges)]
        Zip::from(grid.slice_mut(s![1..-1, 1..-1]))
            .and(&to_be_removed)
            .for_each(|v, &to_remove| {
                if to_remove {
                    *v = Open;
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 13);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 43);
    }
}
