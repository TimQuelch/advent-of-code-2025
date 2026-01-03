use std::{cmp::Reverse, collections::HashSet};

use ndarray::Array2;

#[derive(Debug, Clone, Copy)]
struct Edge {
    cost: f32,
    a: usize,
    b: usize,
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.total_cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn build_edge_list(input: &str) -> (Array2<u32>, Vec<Edge>) {
    let vals = {
        let vec: Vec<_> = input
            .lines()
            .flat_map(|line| line.split(','))
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Array2::from_shape_vec((vec.len() / 3, 3), vec).unwrap()
    };

    let mut edges: Vec<Edge> = Vec::with_capacity(vals.nrows().pow(2) / 2);

    for i in 0..(vals.nrows() - 1) {
        for j in (i + 1)..(vals.nrows()) {
            debug_assert_ne!(i, j);
            #[expect(clippy::cast_precision_loss)]
            let cost = (vals.row(i).mapv(|x| x as f32) - vals.row(j).mapv(|x| x as f32))
                .pow2()
                .sum()
                .sqrt();
            edges.push(Edge { cost, a: i, b: j });
        }
    }
    (vals, edges)
}

fn part1_impl(input: &str, n_connections: usize) -> i64 {
    let (_, mut all_edges) = build_edge_list(input);

    let (edges_slice, _, _) = all_edges.select_nth_unstable(n_connections);
    debug_assert_eq!(edges_slice.len(), n_connections);

    edges_slice.sort_unstable();

    let mut edges = edges_slice.to_vec();

    let mut nodes = vec![];
    let mut set = HashSet::new();
    let mut connected_sets = vec![];

    while let Some(edge) = edges.pop() {
        set.clear();
        nodes.clear();

        nodes.push(edge.a);
        nodes.push(edge.b);
        while let Some(node) = nodes.pop() {
            set.insert(node);
            let new_node_iter = edges
                .extract_if(.., |e| e.a == node || e.b == node)
                .map(|e| if e.a == node { e.b } else { e.a });
            for new_node in new_node_iter {
                if !set.contains(&new_node) {
                    nodes.push(new_node);
                }
            }
        }
        connected_sets.push(set.len());
    }

    let nth = connected_sets.len() - 4;
    let (_, _, top_three) = connected_sets.select_nth_unstable(nth);
    debug_assert_eq!(top_three.len(), 3);

    top_three.iter().product::<usize>().try_into().unwrap()
}

pub fn part1(input: &str) -> i64 {
    part1_impl(input, 1000)
}

pub fn part2(input: &str) -> i64 {
    let (vals, mut edges) = build_edge_list(input);

    // Reverse order so we are popping from the end
    edges.sort_unstable_by_key(|&x| Reverse(x));

    let mut connected_sets: Vec<Option<HashSet<usize>>> = vec![];

    let mut i = 0;
    let last_edge = loop {
        i += 1;
        if i % 100 == 0 {
            connected_sets.retain(std::option::Option::is_some);
        }

        let edge = edges.pop().unwrap();
        let mut matching_sets: Vec<_> = connected_sets
            .iter_mut()
            .filter(|so| {
                so.as_ref()
                    .is_some_and(|s| s.contains(&edge.a) || s.contains(&edge.b))
            })
            .collect();

        match matching_sets.as_mut_slice() {
            [] => connected_sets.push(Some(HashSet::from([edge.a, edge.b]))),
            [Some(set)] => {
                set.insert(edge.a);
                set.insert(edge.b);
            }
            [Some(set_a), opt_set_b] => {
                set_a.extend(opt_set_b.as_ref().unwrap());
                **opt_set_b = None;
            }
            _ => panic!("invalid connected set state"),
        }

        if connected_sets
            .iter()
            .any(|so| so.as_ref().is_some_and(|s| s.len() >= vals.nrows()))
        {
            break edge;
        }
    };

    (vals.row(last_edge.a)[0] * vals.row(last_edge.b)[0]).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn example_part1() {
        let result = part1_impl(EXAMPLE.trim(), 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 25272);
    }
}
