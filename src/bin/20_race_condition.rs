/*
--- Day 20: Race Condition ---

The Historians are quite pixelated again. This time, a massive, black
building looms over you - you're right outside the CPU!

While The Historians get to work, a nearby program sees that you're idle
and challenges you to a race. Apparently, you've arrived just in time for
the frequently-held race condition festival!

The race takes place on a particularly long and twisting code path;
programs compete to see who can finish in the fewest picoseconds. The
winner even gets their very own mutex!

They hand you a map of the racetrack (your puzzle input). For example:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

The map consists of track (.) - including the start (S) and end (E)
positions (both of which also count as track) - and walls (#).

When a program runs through the racetrack, it starts at the start position.
Then, it is allowed to move up, down, left, or right; each such move takes
1 picosecond. The goal is to reach the end position as quickly as possible.
In this example racetrack, the fastest time is 84 picoseconds.

Because there is only a single path from the start to the end and the
programs all go the same speed, the races used to be pretty boring. To make
things more interesting, they introduced a new rule to the races: programs
are allowed to cheat.

The rules for cheating are very strict. Exactly once during a race, a
program may disable collision for up to 2 picoseconds. This allows the
program to pass through walls as if they were regular track. At the end of
the cheat, the program must be back on normal track again; otherwise, it
will receive a segmentation fault and get disqualified.

So, a program could complete the course in 72 picoseconds (saving 12
picoseconds) by cheating for the two moves marked 1 and 2:

###############
#...#...12....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

Or, a program could complete the course in 64 picoseconds (saving 20
picoseconds) by cheating for the two moves marked 1 and 2:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...12..#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

This cheat saves 38 picoseconds:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.####1##.###
#...###.2.#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

This cheat saves 64 picoseconds and takes the program directly to the end:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..21...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

Each cheat has a distinct start position (the position where the cheat is
activated, just before the first move that is allowed to go through walls)
and end position; cheats are uniquely identified by their start position
and end position.

In this example, the total number of cheats (grouped by the amount of time
they save) are as follows:

- There are 14 cheats that save 2 picoseconds.
- There are 14 cheats that save 4 picoseconds.
- There are 2 cheats that save 6 picoseconds.
- There are 4 cheats that save 8 picoseconds.
- There are 2 cheats that save 10 picoseconds.
- There are 3 cheats that save 12 picoseconds.
- There is one cheat that saves 20 picoseconds.
- There is one cheat that saves 36 picoseconds.
- There is one cheat that saves 38 picoseconds.
- There is one cheat that saves 40 picoseconds.
- There is one cheat that saves 64 picoseconds.

You aren't sure what the conditions of the racetrack will be like, so to
give yourself as many options as possible, you'll need a list of the best
cheats. How many cheats would save you at least 100 picoseconds?

--- Part Two ---

The programs seem perplexed by your list of cheats. Apparently, the two-
picosecond cheating rule was deprecated several milliseconds ago! The
latest version of the cheating rule permits a single cheat that instead
lasts at most 20 picoseconds.

Now, in addition to all the cheats that were possible in just two
picoseconds, many more cheats are possible. This six-picosecond cheat saves
76 picoseconds:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#1#####.#.#.###
#2#####.#.#...#
#3#####.#.###.#
#456.E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

Because this cheat has the same start and end positions as the one above,
it's the same cheat, even though the path taken during the cheat is
different:

###############
#...#...#.....#
#.#.#.#.#.###.#
#S12..#.#.#...#
###3###.#.#.###
###4###.#.#...#
###5###.#.###.#
###6.E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############

Cheats don't need to use all 20 picoseconds; cheats can last any amount of
time up to and including 20 picoseconds (but can still only end when the
program is on normal track). Any cheat time not used is lost; it can't be
saved for another cheat later.

You'll still need a list of the best cheats, but now there are even more to
choose between. Here are the quantities of cheats in this example that save
50 picoseconds or more:

- There are 32 cheats that save 50 picoseconds.
- There are 31 cheats that save 52 picoseconds.
- There are 29 cheats that save 54 picoseconds.
- There are 39 cheats that save 56 picoseconds.
- There are 25 cheats that save 58 picoseconds.
- There are 23 cheats that save 60 picoseconds.
- There are 20 cheats that save 62 picoseconds.
- There are 19 cheats that save 64 picoseconds.
- There are 12 cheats that save 66 picoseconds.
- There are 14 cheats that save 68 picoseconds.
- There are 12 cheats that save 70 picoseconds.
- There are 22 cheats that save 72 picoseconds.
- There are 4 cheats that save 74 picoseconds.
- There are 3 cheats that save 76 picoseconds.

Find the best cheats using the updated cheating rules. How many cheats
would save you at least 100 picoseconds?
 */
use std::cmp::max;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("./inputs/20_race_condition.txt").unwrap();
    println!("{}", get_cheat_count(&input_file, 100, 2));
    println!("{}", get_cheat_count(&input_file, 100, 20));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    tile_distance_start: Vec<usize>,
    ordered_tile_positions: Vec<(i64, i64)>,
}

impl Map {
    fn index(&self, x: i64, y: i64) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y * self.width + x)
    }

    fn get_tile(&self, x: i64, y: i64) -> Tile {
        match self.index(x, y) {
            None => Tile::Wall,
            Some(i) => self.tiles[i],
        }
    }

    fn get_distance(&self, x: i64, y: i64) -> usize {
        match self.index(x, y) {
            None => usize::MAX,
            Some(i) => self.tile_distance_start[i],
        }
    }

    fn set_distance(&mut self, x: i64, y: i64, distance: usize) {

        match self.index(x, y) {
            None => {},
            Some(i) => self.tile_distance_start[i] = distance,
        }
    }
}

fn preprocess(input: &str) -> Map {
    let mut input = input.lines();
    let mut tiles = vec![];
    let mut width = 0;
    let mut height = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);

    while let Some(line) = input.next() {
        if line.is_empty() {
            break
        }

        let mut line_width = 0;

        line.char_indices().map(|(x, c)| {
            if c == '#' {
                Tile::Wall
            } else {
                if c == 'S' {
                    start.0 = x as i64;
                    start.1 = height as i64;
                } else if c == 'E' {
                    end.0 = x as i64;
                    end.1 = height as i64;
                }
                Tile::Empty
            }
        }).for_each(|c| {
            tiles.push(c);
            line_width += 1;
        });

        width = max(width, line_width);
        height += 1;
    }

    let tile_distance_start = vec![usize::MAX; tiles.len()];
    let ordered_tile_positions = vec![];

    let mut map = Map {
        width,
        height,
        tiles,
        tile_distance_start,
        ordered_tile_positions,
    };

    {
        let mut pos = start;
        loop {
            map.set_distance(pos.0, pos.1, map.ordered_tile_positions.len());
            map.ordered_tile_positions.push(pos);
            if pos == end {
                break;
            }

            'adjacent_loop: for adjacent in [
                (pos.0 + 1, pos.1 + 0),
                (pos.0 - 1, pos.1 + 0),
                (pos.0 + 0, pos.1 + 1),
                (pos.0 + 0, pos.1 - 1),
            ] {
                if map.get_tile(adjacent.0, adjacent.1) == Tile::Empty && map.get_distance(adjacent.0, adjacent.1) == usize::MAX {
                    pos = adjacent;
                    break 'adjacent_loop;
                }
            }
        }
    }

    map
}

fn get_cheat_count(input: &str, minimum_time_saved: u64, cheat_time: u64) -> u64 {
    let map = preprocess(input);
    let cheat_time_i64 = cheat_time as i64;

    let mut count = 0;

    for position in &map.ordered_tile_positions {
        let start_track_position = map.get_distance(position.0, position.1);
        for x in -cheat_time_i64..=cheat_time_i64 {
            for y in -cheat_time_i64..=cheat_time_i64 {
                let new_position = (position.0 + x, position.1 + y);
                // If it isn't a valid tile to jump to then skip it
                if map.get_tile(new_position.0, new_position.1) != Tile::Empty {
                    continue;
                }
                let manhattan_distance = position.0.abs_diff(new_position.0) + position.1.abs_diff(new_position.1);
                let legal_cheat = manhattan_distance <= cheat_time;
                // If it isn't a valid distance to jump then skip it
                if !legal_cheat {
                    continue;
                }

                let end_track_position = map.get_distance(new_position.0, new_position.1);

                // Make sure that we are cheating to get further along the track
                if end_track_position < start_track_position {
                    continue;
                }
                let track_pos_diff = end_track_position - start_track_position;
                let cheat_price = manhattan_distance as usize;

                // Make sure that our cheat saves us any time
                if track_pos_diff < cheat_price {
                    continue;
                }

                let time_saved = (track_pos_diff - cheat_price) as u64;
                // Make sure that we saved enough time
                if time_saved < minimum_time_saved {
                    continue;
                }

                count += 1;
            }
        }
    }

    count
}

#[test]
fn test_part1() {
    let input = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    assert_eq!(1, get_cheat_count(input, 64, 2));
    assert_eq!(2, get_cheat_count(input, 40, 2));
    assert_eq!(3, get_cheat_count(input, 38, 2));
    assert_eq!(4, get_cheat_count(input, 36, 2));
    assert_eq!(5, get_cheat_count(input, 20, 2));
    assert_eq!(8, get_cheat_count(input, 12, 2));
    assert_eq!(10, get_cheat_count(input, 10, 2));
    assert_eq!(14, get_cheat_count(input, 8, 2));
    assert_eq!(16, get_cheat_count(input, 6, 2));
    assert_eq!(30, get_cheat_count(input, 4, 2));
    assert_eq!(44, get_cheat_count(input, 2, 2));
}

#[test]
fn test_part2() {
    let input = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    assert_eq!(3, get_cheat_count(input, 76, 20));
    assert_eq!(7, get_cheat_count(input, 74, 20));
    assert_eq!(29, get_cheat_count(input, 72, 20));
    assert_eq!(41, get_cheat_count(input, 70, 20));
    assert_eq!(55, get_cheat_count(input, 68, 20));
    assert_eq!(67, get_cheat_count(input, 66, 20));
    assert_eq!(86, get_cheat_count(input, 64, 20));
    assert_eq!(106, get_cheat_count(input, 62, 20));
    assert_eq!(129, get_cheat_count(input, 60, 20));
    assert_eq!(154, get_cheat_count(input, 58, 20));
    assert_eq!(193, get_cheat_count(input, 56, 20));
    assert_eq!(222, get_cheat_count(input, 54, 20));
    assert_eq!(253, get_cheat_count(input, 52, 20));
    assert_eq!(285, get_cheat_count(input, 50, 20));
}