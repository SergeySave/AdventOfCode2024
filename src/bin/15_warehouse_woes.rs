/*
--- Day 15: Warehouse Woes ---

You appear back inside your own mini submarine! Each Historian drives their
mini submarine in a different direction; maybe the Chief has his own
submarine down here somewhere as well?

You look up to see a vast school of lanternfish swimming past you. On
closer inspection, they seem quite anxious, so you drive your mini
submarine over to see if you can help.

Because lanternfish populations grow rapidly, they need a lot of food, and
that food needs to be stored somewhere. That's why these lanternfish have
built elaborate warehouse complexes operated by robots!

These lanternfish seem so anxious because they have lost control of the
robot that operates one of their most important warehouses! It is currently
running amok, pushing around boxes in the warehouse with no regard for
lanternfish logistics or lanternfish inventory management strategies.

Right now, none of the lanternfish are brave enough to swim up to an
unpredictable robot so they could shut it off. However, if you could
anticipate the robot's movements, maybe they could find a safe option.

The lanternfish already have a map of the warehouse and a list of movements
the robot will attempt to make (your puzzle input). The problem is that the
movements will sometimes fail as boxes are shifted around, making the
actual movements of the robot difficult to predict.

For example:

##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^

As the robot (@) attempts to move, if there are any boxes (O) in the way,
the robot will also attempt to push those boxes. However, if this action
would cause the robot or a box to move into a wall (#), nothing moves
instead, including the robot. The initial positions of these are shown on
the map at the top of the document the lanternfish gave you.

The rest of the document describes the moves (^ for up, v for down, < for
left, > for right) that the robot will attempt to make, in order. (The
moves form a single giant sequence; they are broken into multiple lines
just to make copy-pasting easier. Newlines within the move sequence should
be ignored.)

Here is a smaller example to get started:

########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<

Were the robot to attempt the given sequence of moves, it would push around
the boxes as follows:

Initial state:
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move <:
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move ^:
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move ^:
########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move >:
########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

Move v:
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########

Move >:
########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########

Move >:
########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########

Move v:
########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

Move <:
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

The larger example has many more moves; after the robot has finished those
moves, the warehouse would look like this:

##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########

The lanternfish use their own custom Goods Positioning System (GPS for
short) to track the locations of the boxes. The GPS coordinate of a box is
equal to 100 times its distance from the top edge of the map plus its
distance from the left edge of the map. (This process does not stop at wall
tiles; measure all the way to the edges of the map.)

So, the box shown below has a distance of 1 from the top edge of the map
and 4 from the left edge of the map, resulting in a GPS coordinate of
100 * 1 + 4 = 104.

#######
#...O..
#......

The lanternfish would like to know the sum of all boxes' GPS coordinates
after the robot finishes moving. In the larger example, the sum of all
boxes' GPS coordinates is 10092. In the smaller example, the sum is 2028.

Predict the motion of the robot and boxes in the warehouse. After the robot
is finished moving, what is the sum of all boxes' GPS coordinates?

--- Part Two ---

The lanternfish use your information to find a safe moment to swim in and
turn off the malfunctioning robot! Just as they start preparing a festival
in your honor, reports start coming in that a second warehouse's robot is
also malfunctioning.

This warehouse's layout is surprisingly similar to the one you just helped.
There is one key difference: everything except the robot is twice as wide!
The robot's list of movements doesn't change.

To get the wider warehouse's map, start with your original map and, for
each tile, make the following changes:

If the tile is #, the new map contains ## instead.
If the tile is O, the new map contains [] instead.
If the tile is ., the new map contains .. instead.
If the tile is @, the new map contains @. instead.

This will produce a new warehouse map which is twice as wide and with wide
boxes that are represented by []. (The robot does not change size.)

The larger example from before would now look like this:

####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################

Because boxes are now twice as wide but the robot is still the same size
and speed, boxes can be aligned such that they directly push two other
boxes at once. For example, consider this situation:

#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^

After appropriately resizing this map, the robot would push around these
boxes as follows:

Initial state:
##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

Move <:
##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############

Move v:
##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############

Move v:
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############

Move <:
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############

Move <:
##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############

Move ^:
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############

Move ^:
##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############

Move <:
##############
##......##..##
##...[][]...##
##....[]....##
##....@.....##
##..........##
##############

Move <:
##############
##......##..##
##...[][]...##
##....[]....##
##...@......##
##..........##
##############

Move ^:
##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############

Move ^:
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############

This warehouse also uses GPS to locate the boxes. For these larger boxes,
distances are measured from the edge of the map to the closest edge of the
box in question. So, the box shown below has a distance of 1 from the top
edge of the map and 5 from the left edge of the map, resulting in a GPS
coordinate of 100 * 1 + 5 = 105.

##########
##...[]...
##........

In the scaled-up version of the larger example from above, after the robot
has finished all of its moves, the warehouse would look like this:

####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################

The sum of these boxes' GPS coordinates is 9021.

Predict the motion of the robot and boxes in this new, scaled-up warehouse.
What is the sum of all boxes' final GPS coordinates?
 */
use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::fs;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./inputs/15_warehouse_woes.txt").unwrap();
    println!("{}", get_final_box_gps_sum(&input_file));
    println!("{}", get_final_box_gps_sum_double(&input_file));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
enum Tile {
    #[default]
    Wall,
    Empty,
    Box
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Empty),
            '@' => Ok(Tile::Empty),
            'O' => Ok(Tile::Box),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Empty => write!(f, "."),
            Tile::Box => write!(f, "O"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
enum TileDouble {
    #[default]
    Wall,
    Empty,
    BoxLeft,
    BoxRight
}

impl Display for TileDouble {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileDouble::Wall => write!(f, "#"),
            TileDouble::Empty => write!(f, "."),
            TileDouble::BoxLeft => write!(f, "["),
            TileDouble::BoxRight => write!(f, "]"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Up,
    Right,
    Down,
    Left
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Instruction::Up),
            '>' => Ok(Instruction::Right),
            'v' => Ok(Instruction::Down),
            '<' => Ok(Instruction::Left),
            _ => Err(()),
        }
    }
}

impl Instruction {
    fn get_direction(self) -> (i64, i64) {
        match self {
            Instruction::Up => (0, -1),
            Instruction::Right => (1, 0),
            Instruction::Down => (0, 1),
            Instruction::Left => (-1, 0),
        }
    }
}

#[derive(Clone)]
struct Map<T: Clone> {
    tiles: Vec<T>,
    width: usize,
    height: usize,
    robot: (i64, i64)
}

impl<T: Display + Copy + Default + Clone> Map<T> {
    fn get(&self, x: i64, y: i64) -> T {
        if x < 0 || y < 0 {
            return T::default();
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return T::default();
        }
        self.tiles[y * self.width + x]
    }
    fn set(&mut self, x: i64, y: i64, tile: T) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return;
        }
        self.tiles[y * self.width + x] = tile;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if x as i64 == self.robot.0 && y as i64 == self.robot.1 {
                    print!("@");
                } else {
                    print!("{}", self.get(x as i64, y as i64));
                }
            }
            println!();
        }
        println!();
    }
}

impl Map<Tile> {
    fn get_box_gps_sum(&self) -> u64 {
        let mut result = 0u64;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x as i64, y as i64) == Tile::Box {
                    result += (y as u64) * 100 + (x as u64);
                }
            }
        }
        result
    }
}

impl Map<TileDouble> {
    fn get_box_gps_sum(&self) -> u64 {
        let mut result = 0u64;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x as i64, y as i64) == TileDouble::BoxLeft {
                    result += (y as u64) * 100 + (x as u64);
                }
            }
        }
        result
    }
}

fn preprocess(input: &str) -> (Map<Tile>, Vec<Instruction>) {
    let mut input = input.lines();

    let mut tiles = vec![];
    let mut width = 0usize;
    let mut height = 0usize;
    let mut robot = (0i64,0i64);

    while let Some(line) = input.next() {
        if line.is_empty() {
            break
        }

        let mut line_width = 0;

        line.char_indices().filter_map(|(x, c)| {
            if c == '@' {
                robot.0 = x as i64;
                robot.1 = height as i64;
            }
            Tile::try_from(c).ok()
        }).for_each(|c| {
            tiles.push(c);
            line_width += 1;
        });

        width = max(width, line_width);
        height += 1;
    }

    let instructions = input
        .flat_map(|line| line.chars())
        .filter_map(|c| Instruction::try_from(c).ok())
        .collect_vec();

    let map = Map {
        tiles,
        width,
        height,
        robot,
    };

    (map, instructions)
}

fn preprocess_double(input: &str) -> (Map<TileDouble>, Vec<Instruction>) {
    let (map_single, instructions) = preprocess(input);

    let tiles = map_single.tiles.into_iter().flat_map(|tile| {
        match tile {
            Tile::Wall => vec![TileDouble::Wall, TileDouble::Wall],
            Tile::Empty => vec![TileDouble::Empty, TileDouble::Empty],
            Tile::Box => vec![TileDouble::BoxLeft, TileDouble::BoxRight],
        }
    }).collect_vec();

    (Map {
        tiles,
        width: map_single.width * 2,
        height: map_single.height,
        robot: (map_single.robot.0 * 2, map_single.robot.1),
    }, instructions)
}

fn apply_instruction(map: &mut Map<Tile>, instruction: Instruction) {
    let direction = instruction.get_direction();
    let next_tile = (map.robot.0 + direction.0, map.robot.1 + direction.1);
    let in_direction = map.get(next_tile.0, next_tile.1);
    match in_direction {
        Tile::Wall => {}, // Do Nothing
        Tile::Empty => {
            // Move the robot forward
            map.robot = next_tile;
        },
        Tile::Box => {
            // Try to move the box
            let mut steps = 2;
            while map.get(map.robot.0 + direction.0 * steps, map.robot.1 + direction.1 * steps) == Tile::Box {
                steps += 1;
            }
            let first_non_box_tile = (map.robot.0 + direction.0 * steps, map.robot.1 + direction.1 * steps);
            match map.get(first_non_box_tile.0, first_non_box_tile.1) {
                Tile::Wall => {}, // The stack of boxes is against a wall - we can't do anything
                Tile::Empty => {
                    // Move the stack of boxes
                    map.set(first_non_box_tile.0, first_non_box_tile.1, Tile::Box);
                    map.set(next_tile.0, next_tile.1, Tile::Empty);
                    // Move the robot forward
                    map.robot = next_tile;
                },
                Tile::Box => unreachable!("We shouldn't be here"),
            }
        }
    }
}

fn collision_resolution(map: &mut Map<TileDouble>, position: (i64, i64), direction: (i64, i64)) -> bool {
    let current_tile = map.get(position.0, position.1);
    let next_position = (position.0 + direction.0, position.1 + direction.1);
    let colliding_with = map.get(next_position.0, next_position.1);

    // Everything depends on what we are colliding with
    match colliding_with {
        TileDouble::Wall => false, // Can't do anything about a wall - so resolution fails
        TileDouble::Empty => {
            // Move the current tile into the empty tile
            map.set(next_position.0, next_position.1, current_tile);
            map.set(position.0, position.1, TileDouble::Empty);
            true
        },
        TileDouble::BoxLeft | TileDouble::BoxRight => {
            // We need to be smart when we are handling boxes
            let box_left_pos = if colliding_with == TileDouble::BoxLeft {
                next_position
            } else {
                (next_position.0 - 1, next_position.1)
            };
            let box_right_pos = (box_left_pos.0 + 1, box_left_pos.1);

            let is_vertical_motion = direction.1 != 0;

            let result = if is_vertical_motion {
                // We can resolve if we can move resolve both tiles individually
                collision_resolution(map, box_left_pos, direction) && collision_resolution(map, box_right_pos, direction)
            } else {
                // For horizontal motion we just need to resolve the two sides in the right order
                if direction.0 < 0 {
                    collision_resolution(map, box_left_pos, direction) && collision_resolution(map, box_right_pos, direction)
                } else {
                    collision_resolution(map, box_right_pos, direction) && collision_resolution(map, box_left_pos, direction)
                }
            };
            map.set(next_position.0, next_position.1, current_tile);
            map.set(position.0, position.1, TileDouble::Empty);
            result
        }
    }
}

fn apply_instruction_double(map: &mut Map<TileDouble>, instruction: Instruction) {
    let direction = instruction.get_direction();
    let next_tile = (map.robot.0 + direction.0, map.robot.1 + direction.1);
    let in_direction = map.get(next_tile.0, next_tile.1);
    match in_direction {
        TileDouble::Wall => {}, // Do Nothing
        TileDouble::Empty => {
            // Move the robot forward
            map.robot = next_tile;
        },
        TileDouble::BoxLeft | TileDouble::BoxRight => {
            let mut post_collision_map = map.clone();
            if collision_resolution(&mut post_collision_map, map.robot, direction) {
                *map = post_collision_map;
                // Move the robot forward
                map.robot = next_tile;
            }
        }
    }
}

fn get_final_box_gps_sum(input: &str) -> u64 {
    let (mut map, instructions) = preprocess(input);
    for instruction in instructions {
        apply_instruction(&mut map, instruction);
    }
    map.get_box_gps_sum()
}

fn get_final_box_gps_sum_double(input: &str) -> u64 {
    let (mut map, instructions) = preprocess_double(input);
    // map.print();
    for instruction in instructions {
        // dbg!(instruction);
        apply_instruction_double(&mut map, instruction);
        // map.print();
    }
    map.get_box_gps_sum()
}

#[test]
fn test_part1() {
    assert_eq!(2028,
               get_final_box_gps_sum(
                   r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
               )
    );
    assert_eq!(10092,
               get_final_box_gps_sum(
                   r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
               )
    );
}

#[test]
fn test_part2() {
//     assert_eq!(0,
//                get_final_box_gps_sum_double(
//                    r"#
// .
// O
// O
// @
// #
//
// ^^^^^"
//                )
//     );

    assert_eq!(9021,
               get_final_box_gps_sum_double(
                   r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
               )
    );
}