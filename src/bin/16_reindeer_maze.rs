/*
--- Day 16: Reindeer Maze ---

It's time again for the Reindeer Olympics! This year, the big event is the
Reindeer Maze, where the Reindeer compete for the lowest score.

You and The Historians arrive to search for the Chief right as the event is
about to start. It wouldn't hurt to watch a little, right?

The Reindeer start on the Start Tile (marked S) facing East and need to
reach the End Tile (marked E). They can move forward one tile at a time
(increasing their score by 1 point), but never into a wall (#). They can
also rotate clockwise or counterclockwise 90 degrees at a time (increasing
their score by 1000 points).

To figure out the best place to sit, you start by grabbing a map (your
puzzle input) from a nearby kiosk. For example:

###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############

There are many paths through this maze, but taking any of the best paths
would incur a score of only 7036. This can be achieved by taking a total of
36 steps forward and turning 90 degrees a total of 7 times:


###############
#.......#....E#
#.#.###.#.###^#
#.....#.#...#^#
#.###.#####.#^#
#.#.#.......#^#
#.#.#####.###^#
#..>>>>>>>>v#^#
###^#.#####v#^#
#>>^#.....#v#^#
#^#.#.###.#v#^#
#^....#...#v#^#
#^###.#.#.#v#^#
#S..#.....#>>^#
###############

Here's a second example:

#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################

In this maze, the best paths cost 11048 points; following one such path
would look like this:

#################
#...#...#...#..E#
#.#.#.#.#.#.#.#^#
#.#.#.#...#...#^#
#.#.#.#.###.#.#^#
#>>v#.#.#.....#^#
#^#v#.#.#.#####^#
#^#v..#.#.#>>>>^#
#^#v#####.#^###.#
#^#v#..>>>>^#...#
#^#v###^#####.###
#^#v#>>^#.....#.#
#^#v#^#####.###.#
#^#v#^........#.#
#^#v#^#########.#
#S#>>^..........#
#################

Note that the path shown above includes one 90 degree turn as the very
first move, rotating the Reindeer from facing East to facing North.

Analyze your map carefully. What is the lowest score a Reindeer could
possibly get?

--- Part Two ---

Now that you know what the best paths look like, you can figure out the
best spot to sit.

Every non-wall tile (S, ., or E) is equipped with places to sit along the
edges of the tile. While determining which of these tiles would be the best
spot to sit depends on a whole bunch of factors (how comfortable the seats
are, how far away the bathrooms are, whether there's a pillar blocking your
view, etc.), the most important factor is whether the tile is on one of the
best paths through the maze. If you sit somewhere else, you'd miss all the
action!

So, you'll need to determine which tiles are part of any best path through
the maze, including the S and E tiles.

In the first example, there are 45 tiles (marked O) that are part of at
least one of the various best paths through the maze:

###############
#.......#....O#
#.#.###.#.###O#
#.....#.#...#O#
#.###.#####.#O#
#.#.#.......#O#
#.#.#####.###O#
#..OOOOOOOOO#O#
###O#O#####O#O#
#OOO#O....#O#O#
#O#O#O###.#O#O#
#OOOOO#...#O#O#
#O###.#.#.#O#O#
#O..#.....#OOO#
###############

In the second example, there are 64 tiles that are part of at least one of
the best paths:

#################
#...#...#...#..O#
#.#.#.#.#.#.#.#O#
#.#.#.#...#...#O#
#.#.#.#.###.#.#O#
#OOO#.#.#.....#O#
#O#O#.#.#.#####O#
#O#O..#.#.#OOOOO#
#O#O#####.#O###O#
#O#O#..OOOOO#OOO#
#O#O###O#####O###
#O#O#OOO#..OOO#.#
#O#O#O#####O###.#
#O#O#OOOOOOO..#.#
#O#O#O#########.#
#O#OOO..........#
#################

Analyze your map further. How many tiles are part of at least one of the
best paths through the maze?
 */
use std::cmp::{max, min, Ordering, PartialEq, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn main() {
    let input_file = fs::read_to_string("./inputs/16_reindeer_maze.txt").unwrap();
    println!("{}", get_lowest_score(&input_file));
    println!("{}", get_best_paths_tile_count(&input_file));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    End
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Empty),
            'E' => Ok(Tile::End),
            'S' => Ok(Tile::Empty),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn x(self) -> i64 {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 0,
            Direction::Left => -1,
        }
    }

    fn y(self) -> i64 {
        match self {
            Direction::Up => -1,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 0,
        }
    }

    fn clockwise(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn counterclockwise(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn value(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    x: i64,
    y: i64,
    direction: Direction
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    distance: Vec<usize>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: i64, y: i64) -> Tile {
        if x < 0 || y < 0 {
            return Tile::Wall;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return Tile::Wall;
        }
        self.tiles[y * self.width + x]
    }

    fn get_distance(&self, state: &State) -> usize {
        let x = state.x;
        let y = state.y;
        if x < 0 || y < 0 {
            return usize::MAX;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return usize::MAX;
        }
        self.distance[y * self.width * 4 + x * 4 + state.direction.value()]
    }

    fn set_distance(&mut self, state: &State, distance: usize) {
        let x = state.x;
        let y = state.y;
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }
        self.distance[y * self.width * 4 + x * 4 + state.direction.value()] = distance;
    }

    fn get_adjacent(&self, state: &State) -> Vec<(State, usize)> {
        let forward = state.direction;
        let mut options = vec![
            (
                State {
                    x: state.x,
                    y: state.y,
                    direction: forward.clockwise(),
                },
                1000,
            ),
            (
                State {
                    x: state.x,
                    y: state.y,
                    direction: forward.counterclockwise(),
                },
                1000,
            )
        ];

        let forward = (
            State {
                x: state.x + forward.x(),
                y: state.y + forward.y(),
                direction: forward,
            },
            1,
        );

        if self.get(forward.0.x, forward.0.y) != Tile::Wall {
            options.push(forward);
        }

        options
    }
}

#[derive(Eq, PartialEq)]
struct Entry {
    state: State,
    cost: usize,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn preprocess(input: &str) -> (Map, (i64, i64)) {
    let mut input = input.lines();

    let mut tiles = vec![];
    let mut width = 0usize;
    let mut height = 0usize;
    let mut start = (0i64,0i64);

    while let Some(line) = input.next() {
        if line.is_empty() {
            break
        }

        let mut line_width = 0;

        line.char_indices().filter_map(|(x, c)| {
            if c == 'S' {
                start.0 = x as i64;
                start.1 = height as i64;
            }
            Tile::try_from(c).ok()
        }).for_each(|c| {
            tiles.push(c);
            line_width += 1;
        });

        width = max(width, line_width);
        height += 1;
    }

    let tiles_len = tiles.len();

    (
        Map {
            tiles,
            width,
            height,
            distance: vec![usize::MAX; tiles_len * 4]
        },
        start
    )
}

fn dijkstra(map: &mut Map, start: (i64, i64)) -> (usize, (i64, i64)) {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Entry {
        state: State {
            x: start.0,
            y: start.1,
            direction: Direction::Right,
        },
        cost: 0,
    }));

    let mut win_cost = usize::MAX;
    let mut win_pos = (-1, -1);

    while let Some(Reverse(Entry { state, cost })) = heap.pop() {
        if cost > win_cost {
            break;
        }
        if map.get(state.x, state.y) == Tile::End {
            win_cost = min(win_cost, cost);
            win_pos = (state.x, state.y);
        }
        if map.get_distance(&state) <= cost {
            continue;
        }

        map.set_distance(&state, cost);

        for (next_state, edge_cost) in map.get_adjacent(&state) {
            let new_cost = cost + edge_cost;
            if map.get_distance(&next_state) > new_cost {
                heap.push(Reverse(Entry {
                    state: next_state,
                    cost: new_cost,
                }));
            }
        }
    }

    (win_cost, win_pos)
}

fn get_lowest_score(input: &str) -> u64 {
    let (mut map, start) = preprocess(input);
    dijkstra(&mut map, start).0 as u64
}

fn dfs_find_all_paths(map: &Map, visited_positions: &mut HashSet<(i64, i64)>, state: State, cost: usize, start: &State) -> bool {
    if map.get_distance(&state) != cost {
        return false;
    }
    let is_in_path = if cost == 0 {
        &state == start
    } else {
        let is_in_path_a = dfs_find_all_paths(&map, visited_positions, State {
            x: state.x,
            y: state.y,
            direction: state.direction.counterclockwise(),
        }, cost - 1000, start);
        let is_in_path_b = dfs_find_all_paths(&map, visited_positions, State {
            x: state.x,
            y: state.y,
            direction: state.direction.clockwise(),
        }, cost - 1000, start);
        let is_in_path_c = dfs_find_all_paths(&map, visited_positions, State {
            x: state.x - state.direction.x(),
            y: state.y - state.direction.y(),
            direction: state.direction,
        }, cost - 1, start);

        is_in_path_a | is_in_path_b | is_in_path_c
    };

    if is_in_path {
        visited_positions.insert((state.x, state.y));
    }

    is_in_path
}

fn get_best_paths_tile_count(input: &str) -> u64 {
    let (mut map, start) = preprocess(input);
    let (win_cost, win_pos) = dijkstra(&mut map, start);

    // Dijkstra's algorithm has now updated the costs for all states which are cheaper to get to
    // that win_cost. I can now run a depth-first search backwards from the exit following the
    // state costs

    let start = State {
        x: start.0,
        y: start.1,
        direction: Direction::Right,
    };
    let mut visited_positions = HashSet::new();
    let _ = dfs_find_all_paths(&map, &mut visited_positions, State {
        x: win_pos.0,
        y: win_pos.1,
        direction: Direction::Up,
    }, win_cost, &start);
    let _ = dfs_find_all_paths(&map, &mut visited_positions, State {
        x: win_pos.0,
        y: win_pos.1,
        direction: Direction::Right,
    }, win_cost, &start);
    let _ = dfs_find_all_paths(&map, &mut visited_positions, State {
        x: win_pos.0,
        y: win_pos.1,
        direction: Direction::Down,
    }, win_cost, &start);
    let _ = dfs_find_all_paths(&map, &mut visited_positions, State {
        x: win_pos.0,
        y: win_pos.1,
        direction: Direction::Left,
    }, win_cost, &start);

    visited_positions.len() as u64
}

#[test]
fn test_part1() {
    assert_eq!(7036,
               get_lowest_score(
                   r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
               )
    );
    assert_eq!(11048,
               get_lowest_score(
                   r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!(45,
               get_best_paths_tile_count(
                   r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
               )
    );
    assert_eq!(64,
               get_best_paths_tile_count(
                   r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
               )
    );
}