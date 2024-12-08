/*
--- Day 8: Resonant Collinearity ---

You find yourselves on the roof of a top-secret Easter Bunny installation.

While The Historians do their thing, you take a look at the familiar huge
antenna. Much to your surprise, it seems to have been reconfigured to emit
a signal that makes people 0.1% more likely to buy Easter Bunny brand
Imitation Mediocre Chocolate as a Christmas gift! Unthinkable!

Scanning across the city, you find that there are actually many such
antennas. Each antenna is tuned to a specific frequency indicated by a
single lowercase letter, uppercase letter, or digit. You create a map (your
puzzle input) of these antennas. For example:

............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............

The signal only applies its nefarious effect at specific antinodes based on
the resonant frequencies of the antennas. In particular, an antinode occurs
at any point that is perfectly in line with two antennas of the same
frequency - but only when one of the antennas is twice as far away as the
other. This means that for any pair of antennas with the same frequency,
there are two antinodes, one on either side of them.

So, for these two antennas with frequency a, they create the two antinodes
marked with #:

..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........

Adding a third antenna with the same frequency creates several more
antinodes. It would ideally add four antinodes, but two are off the right
side of the map, so instead it adds only two:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........

Antennas with different frequencies don't create antinodes; A and a count
as different frequencies. However, antinodes can occur at locations that
contain antennas. In this diagram, the lone antenna with frequency capital
A creates no antinodes but has a lowercase-a-frequency antinode at its
location:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........

The first example has antennas with two different frequencies, so the
antinodes they create look like this, plus an antinode overlapping the
topmost A-frequency antenna:

......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.

Because the topmost A-frequency antenna overlaps with a 0-frequency
antinode, there are 14 total unique locations that contain an antinode
within the bounds of the map.

Calculate the impact of the signal. How many unique locations within the
bounds of the map contain an antinode?

--- Part Two ---

Watching over your shoulder as you work, one of The Historians asks if you
took the effects of resonant harmonics into your calculations.

Whoops!

After updating your model, it turns out that an antinode occurs at any grid
position exactly in line with at least two antennas of the same frequency,
regardless of distance. This means that some of the new antinodes will
occur at the position of each antenna (unless that antenna is the only one
of its frequency).

So, these three T-frequency antennas now create many antinodes:

T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........

In fact, the three T-frequency antennas are all exactly in line with two
antennas, so they are all also antinodes! This brings the total number of
antinodes in the above example to 9.

The original example now has 34 antinodes, including the antinodes that
appear on every antenna:

##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##

Calculate the impact of the signal using this updated model. How many
unique locations within the bounds of the map contain an antinode?
 */
use std::collections::HashMap;
use itertools::Itertools;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("./inputs/08_resonant_collinearity.txt").unwrap();
    println!("{}", get_antinodes_count(&input_file, false));
    println!("{}", get_antinodes_count(&input_file, true));
}

struct Map {
    antennas: HashMap<char, Vec<(i64, i64)>>,
    has_antinode: Vec<bool>,
    width: usize,
    height: usize,
}

impl Map {
    fn in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && ((x as usize) < self.width) && ((y as usize) < self.height)
    }

    fn mark_antinode(&mut self, x: i64, y: i64) {
        if self.in_bounds(x, y) {
            self.has_antinode[self.width * (y as usize) + (x as usize)] = true;
        }
    }
}

fn preprocess(input: &str) -> Map {
    let lines = input.lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len(); // char == byte for this problem

    let mut antennas = HashMap::new();

    for (y, line) in lines.into_iter().enumerate() {
        let y = y as i64;
        for (x, c) in line.char_indices() {
            let x = x as i64;
            if c != '.' {
                if !antennas.contains_key(&c) {
                    antennas.insert(c, vec![]);
                }
                antennas.get_mut(&c).unwrap().push((x, y));
            }
        }
    }

    Map {
        antennas,
        has_antinode: vec![false; width * height],
        width,
        height,
    }
}

fn get_antinodes_count(input: &str, repeat: bool) -> u64 {
    let mut map = preprocess(input);
    let antennas = map.antennas.clone();

    for (_frequency, antennas) in antennas {
        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                if i != j {
                    // For all pairs of antennas
                    let i_to_j = (antennas[j].0 - antennas[i].0, antennas[j].1 - antennas[i].1);
                    let mut antinode = (antennas[j].0 + i_to_j.0, antennas[j].1 + i_to_j.1);
                    if repeat {
                        map.mark_antinode(antennas[j].0, antennas[j].1);
                    }
                    while map.in_bounds(antinode.0, antinode.1) {
                        map.mark_antinode(antinode.0, antinode.1);
                        antinode = (antinode.0 + i_to_j.0, antinode.1 + i_to_j.1);
                        if !repeat {
                            break
                        }
                    }
                }
            }
        }
    }

    map.has_antinode.into_iter().filter(|x| *x).count() as u64
}

#[test]
fn test_part1() {
    assert_eq!(14,
               get_antinodes_count(
                   r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
                   false
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!(34,
               get_antinodes_count(
                   r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
                   true
               )
    );
}
