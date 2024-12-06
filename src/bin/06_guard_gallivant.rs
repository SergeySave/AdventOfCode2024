/*
--- Day 6: Guard Gallivant ---

The Historians use their fancy device again, this time to whisk you all
away to the North Pole prototype suit manufacturing lab... in the year
1518! It turns out that having direct access to history is very convenient
for a group of historians.

You still have to be careful of time paradoxes, and so it will be important
to avoid anyone from 1518 while The Historians search for the Chief.
Unfortunately, a single guard is patrolling this part of the lab.

Maybe you can work out where the guard will go ahead of time so that The
Historians can search safely?

You start by making a map (your puzzle input) of the situation. For
example:

....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

The map shows the current position of the guard with ^ (to indicate the
guard is currently facing up from the perspective of the map). Any
obstructions - crates, desks, alchemical reactors, etc. - are shown as #.

Lab guards in 1518 follow a very strict patrol protocol which involves
repeatedly following these steps:

If there is something directly in front of you, turn right 90 degrees.
Otherwise, take a step forward.
Following the above protocol, the guard moves up several times until she
reaches an obstacle (in this case, a pile of failed suit prototypes):

....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Because there is now an obstacle in front of the guard, she turns right
before continuing straight in her new facing direction:

....#.....
........>#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Reaching another obstacle (a spool of several very long polymers), she
turns right again and continues downward:

....#.....
.........#
..........
..#.......
.......#..
..........
.#......v.
........#.
#.........
......#...

This process continues for a while, but the guard eventually leaves the
mapped area (after walking past a tank of universal solvent):

....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#v..

By predicting the guard's route, you can determine which specific positions
in the lab will be in the patrol path. Including the guard's starting
position, the positions visited by the guard before leaving the area are
marked with an X:

....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..

In this example, the guard will visit 41 distinct positions on your map.

Predict the path of the guard. How many distinct positions will the guard
visit before leaving the mapped area?

--- Part Two ---

While The Historians begin working around the guard's patrol route, you
borrow their fancy device and step outside the lab. From the safety of a
supply closet, you time travel through the last few months and record the
nightly status of the lab's guard post on the walls of the closet.

Returning after what seems like only a few seconds to The Historians, they
explain that the guard's patrol area is simply too large for them to safely
search the lab without getting caught.

Fortunately, they are pretty sure that adding a single new obstruction
won't cause a time paradox. They'd like to place the new obstruction in
such a way that the guard will get stuck in a loop, making the rest of the
lab safe to search.

To have the lowest chance of creating a time paradox, The Historians would
like to know all of the possible positions for such an obstruction. The new
obstruction can't be placed at the guard's starting position - the guard is
there right now and would notice.

In the above example, there are only 6 different positions where a new
obstruction would cause the guard to get stuck in a loop. The diagrams of
these six situations use O to mark the new obstruction, | to show a
position where the guard moves up/down, - to show a position where the
guard moves left/right, and + to show a position where the guard moves both
up/down and left/right.

Option one, put a printing press next to the guard's starting position:

....#.....
....+---+#
....|...|.
..#.|...|.
....|..#|.
....|...|.
.#.O^---+.
........#.
#.........
......#...

Option two, put a stack of failed suit prototypes in the bottom right
quadrant of the mapped area:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
......O.#.
#.........
......#...

Option three, put a crate of chimney-squeeze prototype fabric next to the
standing desk in the bottom right quadrant:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----+O#.
#+----+...
......#...

Option four, put an alchemical retroencabulator near the bottom left
corner:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
..|...|.#.
#O+---+...
......#...

Option five, put the alchemical retroencabulator a bit to the right
instead:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
....|.|.#.
#..O+-+...
......#...

Option six, put a tank of sovereign glue right next to the tank of
universal solvent:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----++#.
#+----++..
......#O..

It doesn't really matter what you choose to use as an obstacle so long as
you and The Historians can put it into position without the guard noticing.
The important thing is having enough options that you can find one that
minimizes time paradoxes, and in this example, there are 6 different
positions you could choose.

You need to get the guard stuck in a loop by adding a single new
obstruction. How many different positions could you choose for this
obstruction?
 */

use itertools::Itertools;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("./inputs/06_guard_gallivant.txt").unwrap();
    println!("{}", get_distinct_guard_positions(&input_file));
    println!("{}", get_potential_obstructions(&input_file));
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Free,
    Obstacle,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn unit_vector(self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
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

#[derive(Clone)]
struct GuardState {
    x: i64,
    y: i64,
    direction: Direction
}

impl GuardState {
    fn looking_at(&self) -> (i64, i64) {
        (
            self.x + self.direction.unit_vector().0,
            self.y + self.direction.unit_vector().1
        )
    }

    fn step(&mut self) {
        let (x, y) = self.looking_at();
        self.x = x;
        self.y = y;
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

struct Map {
    tiles: Vec<Tile>,
    visited: Vec<bool>,
    width: usize,
    height: usize,
}

impl Map {
    fn in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && ((x as usize) < self.width) && ((y as usize) < self.height)
    }

    fn get_tile(&self, x: i64, y: i64) -> Tile {
        if !self.in_bounds(x, y) {
            return Tile::Free;
        }
        self.tiles[self.width * (y as usize) + (x as usize)]
    }

    fn set_tile(&mut self, x: i64, y: i64, tile: Tile) {
        if self.in_bounds(x, y) {
            self.tiles[self.width * (y as usize) + (x as usize)] = tile;
        }
    }

    fn visit_tile(&mut self, x: i64, y: i64) {
        if self.in_bounds(x, y) {
            self.visited[self.width * (y as usize) + (x as usize)] = true;
        }
    }

    fn is_visited(&self, x: i64, y: i64) -> bool {
        if !self.in_bounds(x, y) {
            panic!("unimplemented");
        }

        self.visited[self.width * (y as usize) + (x as usize)]
    }
}

struct GuardStateSpace {
    states: Vec<bool>,
    width: usize,
}

impl GuardStateSpace {
    fn for_map(map: &Map) -> Self {
        Self {
            states: vec![false; map.tiles.len() * 4],
            width: map.width,
        }
    }

    fn reset(&mut self) {
        self.states.fill(false);
    }

    fn mark_seen(&mut self, guard: &GuardState) {
        self.states[4 * self.width * (guard.y as usize) + 4 * (guard.x as usize) + guard.direction.value()] = true;
    }

    fn was_seen(&self, guard: &GuardState) -> bool {
        self.states[4 * self.width * (guard.y as usize) + 4 * (guard.x as usize) + guard.direction.value()]
    }
}

fn preprocess(input: &str) -> (Map, GuardState) {
    let lines = input.lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len(); // char == byte for this problem
    let mut map = Map {
        tiles: vec![Tile::Free; width * height],
        visited: vec![false; width * height],
        width,
        height
    };
    let mut guard_x = -1i64;
    let mut guard_y = -1i64;
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '^' {
                guard_x = x as i64;
                guard_y = y as i64;
            }
            map.set_tile(x as i64, y as i64, if c == '#' {
                Tile::Obstacle
            } else {
                Tile::Free
            });
        }
    }
    if guard_x < 0 {
        panic!("Did not find guard position");
    }

    map.visit_tile(guard_x, guard_y);
    (map, GuardState {
        x: guard_x,
        y: guard_y,
        direction: Direction::Up,
    })
}

fn get_guard_expected_visit_map(input: &str) -> (Map, GuardState) {
    let (mut map, original_guard) = preprocess(input);

    let mut guard = original_guard.clone();
    while map.in_bounds(guard.x, guard.y) {
        let looking_at = guard.looking_at();
        let looking_at = map.get_tile(looking_at.0, looking_at.1);
        match looking_at {
            Tile::Free => guard.step(),
            Tile::Obstacle => guard.turn_right(),
        }
        map.visit_tile(guard.x, guard.y);
    }

    (map, original_guard)
}

fn get_distinct_guard_positions(input: &str) -> u64 {
    let (map, _) = get_guard_expected_visit_map(input);

    // Get the number of visited tiles
    map.visited.into_iter()
        .filter(|x| *x)
        .count() as u64
}

fn check_for_loop(
    map: &Map,
    state_space: &mut GuardStateSpace,
    mut guard: GuardState
) -> bool {
    while map.in_bounds(guard.x, guard.y) {
        // If we have returned to a previously seen state then we have a loop
        if state_space.was_seen(&guard) {
            return true;
        }
        // Mark the state as seen
        state_space.mark_seen(&guard);

        // Propagate the state forward
        let looking_at = guard.looking_at();
        let looking_at = map.get_tile(looking_at.0, looking_at.1);
        match looking_at {
            Tile::Free => guard.step(),
            Tile::Obstacle => guard.turn_right(),
        }
    }

    // We escaped the map: therefore no loop
    false
}

fn get_potential_obstructions(input: &str) -> u64 {
    let (mut map, original_guard_position) = get_guard_expected_visit_map(input);

    let mut valid_spots = 0;
    let mut guard_state_space = GuardStateSpace::for_map(&map);

    for x in 0..map.width {
        let x = x as i64;
        for y in 0..map.height {
            let y = y as i64;
            // Skip over any spots which we don't expect to visit: no point checking those
            // Also skip over the guard's initial position - we aren't allowed to place an obstacle there
            if map.is_visited(x, y) && !(x == original_guard_position.x && y == original_guard_position.y) {
                guard_state_space.reset(); // Start with a fresh state space

                // Temporarily add an obstacle
                map.set_tile(x, y, Tile::Obstacle);

                // If we have a loop now then increment our counter
                if check_for_loop(&map, &mut guard_state_space, original_guard_position.clone()) {
                    valid_spots += 1;
                }

                // Remove the obstacle
                map.set_tile(x, y, Tile::Free);
            }
        }
    }

    valid_spots
}


#[test]
fn test_part1() {
    assert_eq!(41,
               get_distinct_guard_positions(
                   r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!(6,
               get_potential_obstructions(
                   r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
               )
    );
}
