use cached::{Cached, UnboundCache, proc_macro::cached};
use std::collections::HashMap;

fn build_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (current, outputs) = line.split_once(": ").unwrap();
            (current, outputs.split_whitespace().collect())
        })
        .collect()
}

#[cached(
    ty = "UnboundCache<(String, String), u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (from.to_owned(), dest.to_owned()) }"#
)]
fn count_paths(graph: &HashMap<&str, Vec<&str>>, from: &str, dest: &str) -> u64 {
    if from == dest {
        1
    } else {
        graph.get(from).map_or(0, |nexts| {
            nexts
                .iter()
                .map(|&next| count_paths(graph, next, dest))
                .sum()
        })
    }
}

pub fn part1(input: &str) -> i64 {
    COUNT_PATHS.lock().unwrap().cache_clear(); // clear cache for fair timings

    let graph = build_graph(input);
    count_paths(&graph, "you", "out").try_into().unwrap()
}

pub fn part2(input: &str) -> i64 {
    COUNT_PATHS.lock().unwrap().cache_clear(); // clear cache for fair timings

    let graph = build_graph(input);

    let cp = |s, e| count_paths(&graph, s, e);

    (cp("svr", "dac") * cp("dac", "fft") * cp("fft", "out")
        + cp("svr", "fft") * cp("fft", "dac") * cp("dac", "out"))
    .try_into()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const EXAMPLE_2: &str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 5);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE_2.trim());
        assert_eq!(result, 2);
    }
}
