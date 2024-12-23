use std::iter::once;

use itertools::Itertools;
use petgraph::{algo::toposort, prelude::*};
use winnow::{
    ascii::{alpha1, line_ending},
    combinator::{separated, separated_pair},
    PResult, Parser as _,
};

use crate::days::Day;

pub type HashMap<K, T> = std::collections::HashMap<K, T, ahash::RandomState>;

pub struct Day23;

fn parse_pair<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str)> {
    separated_pair(alpha1, '-', alpha1).parse_next(input)
}

fn parse_pairs<'a>(input: &mut &'a str) -> PResult<Vec<(&'a str, &'a str)>> {
    separated(1.., parse_pair, line_ending).parse_next(input)
}

#[derive(Debug)]
pub struct Puzzle {
    graph: UnGraph<String, ()>,
    nodes: HashMap<String, NodeIndex>,
}

impl Day for Day23 {
    type Input = Puzzle;

    fn parser(input: &mut &str) -> PResult<Self::Input> {
        let edges = parse_pairs.parse_next(input)?;
        let mut graph = UnGraph::new_undirected();
        let mut nodes = HashMap::default();
        for edge in edges {
            nodes
                .entry(edge.0.to_string())
                .or_insert(graph.add_node(edge.0.to_string()));
            nodes
                .entry(edge.1.to_string())
                .or_insert(graph.add_node(edge.1.to_string()));
            graph.add_edge(*nodes.get(edge.0).unwrap(), *nodes.get(edge.1).unwrap(), ());
        }
        Ok(Puzzle { graph, nodes })
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        // println!(
        //     "{:?}",
        //     Dot::with_config(&input.graph, &[Config::EdgeNoLabel])
        // );
        input
            .nodes
            .values()
            .combinations(3)
            .filter_map(|n| {
                if n.iter()
                    .tuple_combinations()
                    .all(|(a, b)| input.graph.contains_edge(**a, **b))
                    && n.iter()
                        .any(|idx| input.graph.node_weight(**idx).unwrap().starts_with("t"))
                {
                    return Some(n.into_iter().copied().collect_vec());
                }
                None
            })
            .count()
    }

    type Output2 = String;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut largest_group = Vec::new();
        for idx in input.nodes.values() {
            for group in input
                .graph
                .neighbors(*idx)
                .chain(once(*idx))
                .powerset()
                .filter(|set| {
                    set.len() > 1
                        && set
                            .iter()
                            .tuple_combinations()
                            .all(|(a, b)| input.graph.contains_edge(*a, *b))
                })
            {
                if group.len() > largest_group.len() {
                    largest_group = group;
                }
            }
        }
        let mut nodes = largest_group
            .into_iter()
            .map(|idx| input.graph.node_weight(idx).unwrap())
            .cloned()
            .collect_vec();
        nodes.sort_unstable();
        nodes.join(",")
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        let parsed = Day23::parser(&mut INPUT).unwrap();
        assert_eq!(Day23::part_1(&parsed), 7);
    }

    #[test]
    fn test_part2() {
        let parsed = Day23::parser(&mut INPUT).unwrap();
        assert_eq!(Day23::part_2(&parsed), "co,de,ka,ta".to_string());
    }
}
