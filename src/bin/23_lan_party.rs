/*
--- Day 23: LAN Party ---

As The Historians wander around a secure area at Easter Bunny HQ, you come
across posters for a LAN party scheduled for today! Maybe you can find it;
you connect to a nearby datalink port and download a map of the local
network (your puzzle input).

The network map provides a list of every connection between two computers.
For example:

kh-tc
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
td-yn

Each line of text in the network map represents a single connection; the
line kh-tc represents a connection between the computer named kh and the
computer named tc. Connections aren't directional; tc-kh would mean exactly
the same thing.

LAN parties typically involve multiplayer games, so maybe you can locate it
by finding groups of connected computers. Start by looking for sets of
three computers where each computer in the set is connected to the other
two computers.

In this example, there are 12 such sets of three inter-connected computers:

aq,cg,yn
aq,vc,wq
co,de,ka
co,de,ta
co,ka,ta
de,ka,ta
kh,qp,ub
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn
ub,vc,wq

If the Chief Historian is here, and he's at the LAN party, it would be best
to know that right away. You're pretty sure his computer's name starts with
t, so consider only sets of three computers where at least one computer's
name starts with t. That narrows the list down to 7 sets of three inter-
connected computers:

co,de,ta
co,ka,ta
de,ka,ta
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn

Find all the sets of three inter-connected computers. How many contain at
least one computer with a name that starts with t?

--- Part Two ---

There are still way too many results to go through them all. You'll have to
find the LAN party another way and go there yourself.

Since it doesn't seem like any employees are around, you figure they must
all be at the LAN party. If that's true, the LAN party will be the largest
set of computers that are all connected to each other. That is, for each
computer at the LAN party, that computer will have a connection to every
other computer at the LAN party.

In the above example, the largest set of computers that are all connected
to each other is made up of co, de, ka, and ta. Each computer in this set
has a connection to every other computer in the set:

ka-co
ta-co
de-co
ta-ka
de-ta
ka-de

The LAN party posters say that the password to get into the LAN party is
the name of every computer at the LAN party, sorted alphabetically, then
joined together with commas. (The people running the LAN party are clearly
a bunch of nerds.) In this example, the password would be co,de,ka,ta.

What is the password to get into the LAN party?
 */
use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./inputs/23_lan_party.txt").unwrap();
    println!("{}", get_count_3cliques_with_t_computer(&input_file));
    println!("{}", get_lan_password(&input_file));
}

#[derive(Clone)]
struct Graph {
    vertices: Vec<String>,
    edges: HashMap<usize, HashSet<usize>>,
}

fn preprocess(input: &str) -> Graph {
    let mut vertices = vec![];
    let mut edges = HashMap::new();
    let mut vertex_index_map = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        let left = left.to_string();
        let right = right.to_string();

        // Push the vertices
        if !vertex_index_map.contains_key(&left) {
            vertex_index_map.insert(left.clone(), vertices.len());
            vertices.push(left.clone())
        }
        if !vertex_index_map.contains_key(&right) {
            vertex_index_map.insert(right.clone(), vertices.len());
            vertices.push(right.clone())
        }

        let left = vertex_index_map.get(&left).unwrap().clone();
        let right = vertex_index_map.get(&right).unwrap().clone();

        if !edges.contains_key(&left) {
            edges.insert(left, HashSet::new());
        }
        if !edges.contains_key(&right) {
            edges.insert(right, HashSet::new());
        }

        edges.get_mut(&left).unwrap().insert(right);
        edges.get_mut(&right).unwrap().insert(left);
    }

    Graph {
        vertices,
        edges,
    }
}

fn get_count_3cliques_with_t_computer(input: &str) -> u64 {
    let empty_set = HashSet::new();
    let graph = preprocess(input);

    let mut count = 0;

    for (v1, _) in graph.vertices.iter().enumerate() {
        for v2 in graph.edges.get(&v1).unwrap_or(&empty_set).iter().cloned() {
            if v2 > v1 {
                for v3 in graph.edges.get(&v2).unwrap_or(&empty_set).iter().cloned() {
                    if v3 > v2 {
                        if graph.edges.get(&v3).unwrap_or(&empty_set).contains(&v1) {
                            // We have a 3 clique

                            if graph.vertices[v1].starts_with('t')
                                || graph.vertices[v2].starts_with('t')
                                || graph.vertices[v3].starts_with('t') {
                                // if one of the computers starts with a t
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn bron_kerbosch_without_pivoting(graph: &Graph, all: HashSet<usize>, mut some: HashSet<usize>, mut none: HashSet<usize>) -> Option<Vec<usize>> {
    // Implementing the Without Pivoting form https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm#Without_pivoting
    // This was fast enough so one of the more complex forms was not required

    if some.is_empty() && none.is_empty() {
        return Some(all.into_iter().collect_vec());
    }

    let mut best_result = None;

    for vertex in some.clone() {
        {
            let mut all = all.clone();
            all.insert(vertex);
            let some = some.intersection(graph.edges.get(&vertex).unwrap()).cloned();
            let none = none.intersection(graph.edges.get(&vertex).unwrap()).cloned();
            let result = bron_kerbosch_without_pivoting(graph, all, HashSet::from_iter(some), HashSet::from_iter(none));
            match result {
                None => {}
                Some(new_clique) => {
                    best_result = match best_result {
                        None => Some(new_clique),
                        Some(old_clique) => {
                            if old_clique.len() > new_clique.len() {
                                Some(old_clique)
                            } else {
                                Some(new_clique)
                            }
                        }
                    }
                }
            }
        }
        some.remove(&vertex);
        none.insert(vertex);
    }

    best_result
}

fn get_lan_password(input: &str) -> String {
    let graph = preprocess(input);
    let max_clique = bron_kerbosch_without_pivoting(&graph, HashSet::new(), HashSet::from_iter(0..graph.vertices.len()), HashSet::new()).unwrap();
    max_clique.into_iter()
        .map(|v| graph.vertices[v].clone())
        .sorted()
        .join(",")
}

#[test]
fn test_part1() {
    assert_eq!(
        7,
        get_count_3cliques_with_t_computer(r"kh-tc
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
td-yn"
        )
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        "co,de,ka,ta",
        get_lan_password(r"kh-tc
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
td-yn"
        )
    );
}