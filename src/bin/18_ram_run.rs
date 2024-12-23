/*
--- Day 18: RAM Run ---

You and The Historians look a lot more pixelated than you remember. You're
inside a computer at the North Pole!

Just as you're about to check out your surroundings, a program runs up to
you. "This region of memory isn't safe! The User misunderstood what a
pushdown automaton is and their algorithm is pushing whole bytes down on
top of us! Run!"

The algorithm is fast - it's going to cause a byte to fall into your memory
space once every nanosecond! Fortunately, you're faster, and by quickly
scanning the algorithm, you create a list of which bytes will fall (your
puzzle input) in the order they'll land in your memory space.

Your memory space is a two-dimensional grid with coordinates that range
from 0 to 70 both horizontally and vertically. However, for the sake of
example, suppose you're on a smaller grid with coordinates that range from
0 to 6 and the following list of incoming byte positions:

5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0

Each byte position is given as an X,Y coordinate, where X is the distance
from the left edge of your memory space and Y is the distance from the top
edge of your memory space.

You and The Historians are currently in the top left corner of the memory
space (at 0,0) and need to reach the exit in the bottom right corner (at
70,70 in your memory space, but at 6,6 in this example). You'll need to
simulate the falling bytes to plan out where it will be safe to run; for
now, simulate just the first few bytes falling into your memory space.

As bytes fall into your memory space, they make that coordinate corrupted.
Corrupted memory coordinates cannot be entered by you or The Historians, so
you'll need to plan your route carefully. You also cannot leave the
boundaries of the memory space; your only hope is to reach the exit.

In the above example, if you were to draw the memory space after the first
12 bytes have fallen (using . for safe and # for corrupted), it would look
like this:

...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....

You can take steps up, down, left, or right. After just 12 bytes have
corrupted locations in your memory space, the shortest path from the top
left corner to the exit would take 22 steps. Here (marked with O) is one
such path:

OO.#OOO
.O#OO#O
.OOO#OO
...#OO#
..#OO#.
.#.O#..
#.#OOOO

Simulate the first kilobyte (1024 bytes) falling onto your memory space.
Afterward, what is the minimum number of steps needed to reach the exit?

--- Part Two ---

The Historians aren't as used to moving around in this pixelated universe
as you are. You're afraid they're not going to be fast enough to make it to
the exit before the path is completely blocked.

To determine how fast everyone needs to go, you need to determine the first
byte that will cut off the path to the exit.

In the above example, after the byte at 1,1 falls, there is still a path to
the exit:

O..#OOO
O##OO#O
O#OO#OO
OOO#OO#
###OO##
.##O###
#.#OOOO

However, after adding the very next byte (at 6,1), there is no longer a
path to the exit:

...#...
.##..##
.#..#..
...#..#
###..##
.##.###
#.#....

So, in this example, the coordinates of the first byte that prevents the
exit from being reachable are 6,1.

Simulate more of the bytes that are about to corrupt your memory space.
What are the coordinates of the first byte that will prevent the exit from
being reachable from your starting position? (Provide the answer as two
integers separated by a comma with no other characters.)
 */
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fs;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./inputs/18_ram_run.txt").unwrap();
    println!("{}", get_minimum_steps_to_exit(&input_file, 71, 1024));
    let coordinates_cutoff = get_coordinates_cutoff(&input_file, 71);
    println!("{},{}", coordinates_cutoff.0, coordinates_cutoff.1);
}

struct Map {
    corrupted: Vec<bool>,
    visited: Vec<bool>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(size: usize) -> Self {
        Self {
            corrupted: vec![false; size * size],
            visited: vec![false; size * size],
            width: size,
            height: size,
        }
    }

    fn is_corrupted(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 {
            return true;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return true;
        }

        self.corrupted[y * self.width + x]
    }

    fn set_corrupted(&mut self, x: i64, y: i64, value: bool) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }

        self.corrupted[y * self.width + x] = value
    }

    fn is_visited(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 {
            return true;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return true;
        }

        self.visited[y * self.width + x]
    }

    fn set_visited(&mut self, x: i64, y: i64, value: bool) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }

        self.visited[y * self.width + x] = value
    }
}

fn generate_corrupted_map(input: &str, size: usize, bytes: usize) -> Map {
    let mut map = Map::new(size);

    let mut lines = input.lines();

    for _ in 0..bytes {
        let Some(line) = lines.next() else { break; };
        let Some((left, right)) = line.split_once(',') else { panic!("Bad Format") };
        let left = left.parse::<i64>().unwrap();
        let right = right.parse::<i64>().unwrap();

        map.set_corrupted(left, right, true);
    }

    map
}

#[derive(Eq, PartialEq)]
struct Entry {
    state: (i64, i64),
    cost: usize,
    heuristic: usize,
    value: usize,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn astar(map: &mut Map, start: (i64, i64), end: (i64, i64)) -> Option<u64> {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Entry {
        state: start,
        cost: 0,
        heuristic: 0,
        value: 0,
    }));

    while let Some(Reverse(Entry { state, cost, ..})) = queue.pop() {
        if state == end {
            return Some(cost as u64);
        }
        if map.is_visited(state.0, state.1) {
            continue;
        }
        if map.is_corrupted(state.0, state.1) {
            continue;
        }

        map.set_visited(state.0, state.1, true);

        for next_state in [
            (state.0 + 1, state.1 + 0),
            (state.0 - 1, state.1 + 0),
            (state.0 + 0, state.1 + 1),
            (state.0 + 0, state.1 - 1),
        ] {
            let new_cost = cost + 1;
            let heuristic = (state.0.abs_diff(next_state.0) + state.1.abs_diff(next_state.1)) as usize;
            let value = new_cost + heuristic;
            queue.push(Reverse(Entry {
                state: next_state,
                cost: new_cost,
                heuristic,
                value,
            }));
        }
    }

    None
}

fn get_minimum_steps_to_exit(input: &str, size: usize, bytes: usize) -> u64 {
    let mut map = generate_corrupted_map(input, size, bytes);
    astar(&mut map, (0, 0), (size as i64 - 1, size as i64 - 1)).unwrap()
}

fn get_coordinates_cutoff(input: &str, size: usize) -> (i64, i64) {
    let lines = input.lines().collect_vec();
    // We're going to do binary search to find the cutoff
    let mut min_bytes = 0usize;
    let mut max_bytes = lines.len();

    // Make sure that there is at least one value in between the min and max
    while min_bytes + 1 < max_bytes {
        let bytes = (min_bytes + max_bytes) / 2;
        let mut map = generate_corrupted_map(input, size, bytes + 1);
        match astar(&mut map, (0, 0), (size as i64 - 1, size as i64 - 1)) {
            Some(_) => min_bytes = bytes,
            None => max_bytes = bytes,
        }
    }

    let (x, y) = lines[max_bytes].split_once(',').unwrap();
    (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
}

#[test]
fn test_part1() {
    assert_eq!(22,
               get_minimum_steps_to_exit(
                   r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0", 7, 12
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!((6, 1),
               get_coordinates_cutoff(
                   r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0", 7
               )
    );
}