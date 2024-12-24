/*
--- Day 21: Keypad Conundrum ---

As you teleport onto Santa's Reindeer-class starship, The Historians begin
to panic: someone from their search party is missing. A quick life-form
scan by the ship's computer reveals that when the missing Historian
teleported, he arrived in another part of the ship.

The door to that area is locked, but the computer can't open it; it can
only be opened by physically typing the door codes (your puzzle input) on
the numeric keypad on the door.

The numeric keypad has four rows of buttons: 789, 456, 123, and finally an
empty gap followed by 0A. Visually, they are arranged like this:

+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

Unfortunately, the area outside the door is currently depressurized and
nobody can go near the door. A robot needs to be sent instead.

The robot has no problem navigating the ship and finding the numeric
keypad, but it's not designed for button pushing: it can't be told to push
a specific button directly. Instead, it has a robotic arm that can be
controlled remotely via a directional keypad.

The directional keypad has two rows of buttons: a gap / ^ (up) / A
(activate) on the first row and < (left) / v (down) / > (right) on the
second row. Visually, they are arranged like this:

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

When the robot arrives at the numeric keypad, its robotic arm is pointed at
the A button in the bottom right corner. After that, this directional
keypad remote control must be used to maneuver the robotic arm: the up /
down / left / right buttons cause it to move its arm one button in that
direction, and the A button causes the robot to briefly move forward,
pressing the button being aimed at by the robotic arm.

For example, to make the robot type 029A on the numeric keypad, one
sequence of inputs on the directional keypad you could use is:

- < to move the arm from A (its initial position) to 0.
- A to push the 0 button.
- ^A to move the arm to the 2 button and push it.
- >^^A to move the arm to the 9 button and push it.
- vvvA to move the arm to the A button and push it.

In total, there are three shortest possible sequences of button presses on
this directional keypad that would cause the robot to type 029A:
<A^A>^^AvvvA, <A^A^>^AvvvA, and <A^A^^>AvvvA.

Unfortunately, the area containing this directional keypad remote control
is currently experiencing high levels of radiation and nobody can go near
it. A robot needs to be sent instead.

When the robot arrives at the directional keypad, its robot arm is pointed
at the A button in the upper right corner. After that, a second, different
directional keypad remote control is used to control this robot (in the
same way as the first robot, except that this one is typing on a
directional keypad instead of a numeric keypad).

There are multiple shortest possible sequences of directional keypad button
presses that would cause this robot to tell the first robot to type 029A on
the door. One such sequence is v<<A>>^A<A>AvA<^AA>A<vAAA>^A.

Unfortunately, the area containing this second directional keypad remote
control is currently -40 degrees! Another robot will need to be sent to
type on that directional keypad, too.

There are many shortest possible sequences of directional keypad button
presses that would cause this robot to tell the second robot to tell the
first robot to eventually type 029A on the door. One such sequence is
<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A.

Unfortunately, the area containing this third directional keypad remote
control is currently full of Historians, so no robots can find a clear path
there. Instead, you will have to type this sequence yourself.

Were you to choose this sequence of button presses, here are all of the
buttons that would be pressed on your directional keypad, the two robots'
directional keypads, and the numeric keypad:

<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
v<<A>>^A<A>AvA<^AA>A<vAAA>^A
<A^A>^^AvvvA
029A

In summary, there are the following keypads:

- One directional keypad that you are using.
- Two directional keypads that robots are using.
- One numeric keypad (on a door) that a robot is using.

It is important to remember that these robots are not designed for button
pushing. In particular, if a robot arm is ever aimed at a gap where no
button is present on the keypad, even for an instant, the robot will panic
unrecoverably. So, don't do that. All robots will initially aim at the
keypad's A key, wherever it is.

To unlock the door, five codes will need to be typed on its numeric keypad.
For example:

029A
980A
179A
456A
379A

For each of these, here is a shortest sequence of button presses you could
type to cause the desired code to be typed on the numeric keypad:

029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

The Historians are getting nervous; the ship computer doesn't remember
whether the missing Historian is trapped in the area containing a giant
electromagnet or molten lava. You'll need to make sure that for each of the
five codes, you find the shortest sequence of button presses necessary.

The complexity of a single code (like 029A) is equal to the result of
multiplying these two values:

- The length of the shortest sequence of button presses you need to type
on your directional keypad in order to cause the code to be typed on
the numeric keypad; for 029A, this would be 68.
- The numeric part of the code (ignoring leading zeroes); for 029A,
this would be 29.

In the above example, complexity of the five codes can be found by
calculating 68 * 29, 60 * 980, 68 * 179, 64 * 456, and 64 * 379. Adding
these together produces 126384.

Find the fewest number of button presses you'll need to perform in order to
cause the robot in front of the door to type each code. What is the sum of
the complexities of the five codes on your list?

--- Part Two ---

Just as the missing Historian is released, The Historians realize that a
second member of their search party has also been missing this entire time!

A quick life-form scan reveals the Historian is also trapped in a locked
area of the ship. Due to a variety of hazards, robots are once again
dispatched, forming another chain of remote control keypads managing
robotic-arm-wielding robots.

This time, many more robots are involved. In summary, there are the
following keypads:

- One directional keypad that you are using.
- 25 directional keypads that robots are using.
- One numeric keypad (on a door) that a robot is using.

The keypads form a chain, just like before: your directional keypad
controls a robot which is typing on a directional keypad which controls a
robot which is typing on a directional keypad... and so on, ending with the
robot which is typing on the numeric keypad.

The door codes are the same this time around; only the number of robots and
directional keypads has changed.

Find the fewest number of button presses you'll need to perform in order to
cause the robot in front of the door to type each code. What is the sum of
the complexities of the five codes on your list?
 */
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./inputs/21_keypad_conundrum.txt").unwrap();
    println!("{}", get_complexity_sum_bfs::<2>(&input_file));
    println!("{}", get_complexity_sum_path_construction::<2>(&input_file));
    // BFS solution is too slow for the size 25 problem
    println!("{}", get_complexity_sum_path_construction::<25>(&input_file));
}

trait Keypad {
    fn get_position(self) -> (u64, u64);
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
enum NumericKeypad {
    #[default]
    A,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
}

impl From<char> for NumericKeypad {
    fn from(value: char) -> Self {
        match value {
            'A' => NumericKeypad::A,
            '0' => NumericKeypad::Num0,
            '1' => NumericKeypad::Num1,
            '2' => NumericKeypad::Num2,
            '3' => NumericKeypad::Num3,
            '4' => NumericKeypad::Num4,
            '5' => NumericKeypad::Num5,
            '6' => NumericKeypad::Num6,
            '7' => NumericKeypad::Num7,
            '8' => NumericKeypad::Num8,
            '9' => NumericKeypad::Num9,
            _ => panic!("Encountered an unexpected numeric input")
        }
    }
}

impl Display for NumericKeypad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericKeypad::A => write!(f, "A"),
            NumericKeypad::Num0 => write!(f, "0"),
            NumericKeypad::Num1 => write!(f, "1"),
            NumericKeypad::Num2 => write!(f, "2"),
            NumericKeypad::Num3 => write!(f, "3"),
            NumericKeypad::Num4 => write!(f, "4"),
            NumericKeypad::Num5 => write!(f, "5"),
            NumericKeypad::Num6 => write!(f, "6"),
            NumericKeypad::Num7 => write!(f, "7"),
            NumericKeypad::Num8 => write!(f, "8"),
            NumericKeypad::Num9 => write!(f, "9"),
        }
    }
}

impl NumericKeypad {
    const ALL: &'static [NumericKeypad] = &[
        NumericKeypad::A,
        NumericKeypad::Num0,
        NumericKeypad::Num1,
        NumericKeypad::Num2,
        NumericKeypad::Num3,
        NumericKeypad::Num4,
        NumericKeypad::Num5,
        NumericKeypad::Num6,
        NumericKeypad::Num7,
        NumericKeypad::Num8,
        NumericKeypad::Num9,
    ];

    fn get_by_position(x: i64, y: i64) -> Option<NumericKeypad> {
        NumericKeypad::ALL
            .iter()
            .cloned()
            .filter(|d| (d.get_position().0 as i64 == x) && (d.get_position().1 as i64 == y))
            .next()
    }
}

impl Keypad for NumericKeypad {
    fn get_position(self) -> (u64, u64) {
        match self {
            NumericKeypad::A => (2, 3),
            NumericKeypad::Num0 => (1, 3),
            NumericKeypad::Num1 => (0, 2),
            NumericKeypad::Num2 => (1, 2),
            NumericKeypad::Num3 => (2, 2),
            NumericKeypad::Num4 => (0, 1),
            NumericKeypad::Num5 => (1, 1),
            NumericKeypad::Num6 => (2, 1),
            NumericKeypad::Num7 => (0, 0),
            NumericKeypad::Num8 => (1, 0),
            NumericKeypad::Num9 => (2, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
enum DirectionalKeypad {
    #[default]
    A,
    Up,
    Down,
    Right,
    Left
}

impl Display for DirectionalKeypad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionalKeypad::A => write!(f, "A"),
            DirectionalKeypad::Up => write!(f, "^"),
            DirectionalKeypad::Down => write!(f, "v"),
            DirectionalKeypad::Right => write!(f, ">"),
            DirectionalKeypad::Left => write!(f, "<"),
        }
    }
}

impl DirectionalKeypad {
    const ALL: &'static [DirectionalKeypad] = &[
        DirectionalKeypad::A,
        DirectionalKeypad::Up,
        DirectionalKeypad::Down,
        DirectionalKeypad::Right,
        DirectionalKeypad::Left,
    ];

    fn get_by_position(x: i64, y: i64) -> Option<DirectionalKeypad> {
        DirectionalKeypad::ALL
            .iter()
            .cloned()
            .filter(|d| (d.get_position().0 as i64 == x) && (d.get_position().1 as i64 == y))
            .next()
    }
}

impl Keypad for DirectionalKeypad {
    fn get_position(self) -> (u64, u64) {
        match self {
            DirectionalKeypad::A => (2, 0),
            DirectionalKeypad::Up => (1, 0),
            DirectionalKeypad::Down => (1, 1),
            DirectionalKeypad::Right => (2, 1),
            DirectionalKeypad::Left => (0, 1),
        }
    }
}

impl DirectionalKeypad {
    fn nudge(self) -> (i64, i64) {
        match self {
            DirectionalKeypad::A => unreachable!(),
            DirectionalKeypad::Up => (0, -1),
            DirectionalKeypad::Down => (0, 1),
            DirectionalKeypad::Right => (1, 0),
            DirectionalKeypad::Left => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct Input {
    sequence: Vec<NumericKeypad>,
    numeric: u64,
}

fn preprocess(input: &str) -> Vec<Input> {
    input.lines().map(|line| {
        let numeric = &line[..3];
        assert_eq!(4, line.len());
        assert_eq!("A", &line[3..]);
        let numeric = numeric.parse::<u64>().unwrap();
        Input {
            sequence: line.chars().map(NumericKeypad::from).collect_vec(),
            numeric,
        }
    }).collect_vec()
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct State<const ROBOTS: usize> {
    numeric_robot: NumericKeypad,
    keypad_robots: [DirectionalKeypad; ROBOTS],
}

struct Entry<const ROBOTS: usize> {
    state: State<ROBOTS>,
    cost: u64,
}

fn get_numeric_sequence_length<const ROBOTS: usize>(start: NumericKeypad, end: NumericKeypad) -> u64 {
    // Let's use BFS to find the fastest solution from start -> end
    let mut queue = VecDeque::new();
    queue.push_back(Entry {
        state: State {
            numeric_robot: start,
            keypad_robots: [DirectionalKeypad::default(); ROBOTS],
        },
        cost: 0,
    });

    let mut visited = HashSet::new();

    while let Some(Entry { state, cost }) = queue.pop_front() {
        if state.numeric_robot == end && state.keypad_robots.iter().all(|r| *r == DirectionalKeypad::A) {
            // Press A and win!
            return cost + 1;
        }
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());

        // Try our four movement actions (hitting our directional keypad on one of the directional keys)
        for action in [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ] {
            let start_position = state.keypad_robots[ROBOTS - 1].get_position();
            let start_position = (start_position.0 as i64, start_position.1 as i64);
            let end_position = (start_position.0 + action.0, start_position.1 + action.1);
            // Get the corresponding action we need to hit to get to the desired position
            // (this filters for invalid tiles)
            let Some(new_robot_position) = DirectionalKeypad::get_by_position(end_position.0, end_position.1) else { continue; };
            let mut new_state = state.clone();
            new_state.keypad_robots[ROBOTS - 1] = new_robot_position;
            queue.push_back(Entry {
                state: new_state,
                cost: cost + 1
            });
        }

        // Add the action to press 'A'
        for robot in (0..ROBOTS).rev() {
            if state.keypad_robots[robot] != DirectionalKeypad::A {
                let action = state.keypad_robots[robot].nudge();
                let mut new_state = state.clone();
                if robot > 0 {
                    let start_position = state.keypad_robots[robot - 1].get_position();
                    let start_position = (start_position.0 as i64, start_position.1 as i64);
                    let end_position = (start_position.0 + action.0, start_position.1 + action.1);
                    // Get the corresponding action we need to hit to get to the desired position
                    // (this filters for invalid tiles)
                    let Some(new_robot_position) = DirectionalKeypad::get_by_position(end_position.0, end_position.1) else { break; };
                    new_state.keypad_robots[robot - 1] = new_robot_position;
                } else {
                    let start_position = state.numeric_robot.get_position();
                    let start_position = (start_position.0 as i64, start_position.1 as i64);
                    let end_position = (start_position.0 + action.0, start_position.1 + action.1);
                    // Get the corresponding action we need to hit to get to the desired position
                    // (this filters for invalid tiles)
                    let Some(new_robot_position) = NumericKeypad::get_by_position(end_position.0, end_position.1) else { break; };
                    new_state.numeric_robot = new_robot_position;
                }
                queue.push_back(Entry {
                    state: new_state,
                    cost: cost + 1
                });
                break;
            }
        }
    }

    panic!("Failed to find solution");
}

// This BFS solution stops working for ROBOTS == 7 as it gets too slow
fn get_complexity_sum_bfs<const ROBOTS: usize>(input: &str) -> u64 {
    let inputs = preprocess(input);
    let mut result = 0;
    for input in inputs {
        // Unfortunately the easy/trivial solution doesn't produce an optimal enough solution
        // so lets use the insight that every letter of the input ends with a chain of 'A' inputs
        // on all of the layers
        // That means that we can individually solve for each number -> number input pair without
        // without needing to chain them all into a single search problem
        // As separate search problems they can be solved using any search algorithm - though as
        // a shortest solution is desired BFS is the chosen algorithm. Since all edges are of equal
        // weight, this is equivalent to dijkstra's algorithm. And I can't think of a trivial
        // heuristic to use so astar is also not an option. (Though if we wanted to take this a step
        // further we could do an action planning implementation of astar which would work).
        let mut length = 0;
        let mut last = NumericKeypad::A;
        for num in input.sequence {
            length += get_numeric_sequence_length::<ROBOTS>(last, num);
            last = num;
        }
        result += length * input.numeric;
    }
    result
}

// fn move_numeric_to_state<const ROBOTS: usize>(state: State<ROBOTS>)

// get_cost(A -> 0) + get_cost(0 -> 2) + get_cost(2 -> 9) + get_cost(9 -> A)
// get_cost(A -> <) + get_cost(< -> A) + get_cost(0 -> 2) + get_cost(2 -> 9) + get_cost(9 -> A)

enum Order {
    UpDownLeftRight,
    LeftRightUpDown,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct MemoKey {
    from: DirectionalKeypad,
    to: DirectionalKeypad,
    robots: usize,
}

fn get_cost_numeric(from: NumericKeypad, to: NumericKeypad, robots: usize, memo: &mut HashMap<MemoKey, u64>) -> u64 {
    let x = (to.get_position().0 as i64) - (from.get_position().0 as i64);
    let y = (to.get_position().1 as i64) - (from.get_position().1 as i64);

    let up = if y < 0 { (-y) as u64 } else { 0 };
    let down = if y > 0 { y as u64 } else { 0 };
    let left = if x < 0 { (-x) as u64 } else { 0 };
    let right = if x > 0 { x as u64 } else { 0 };

    // Rules for our update ordering
    let order = if from.get_position().0 == 0 && to.get_position().1 == 3 {
        // Apply x first then y
        Order::LeftRightUpDown
    } else if from.get_position().1 == 3 && to.get_position().0 == 0 {
        // Apply y first then x
        Order::UpDownLeftRight
    } else if left > 0 {
        Order::LeftRightUpDown
    } else if right > 0 {
        Order::UpDownLeftRight
    } else {
        // If we aren't going sideways order doesn't really matter anyways
        Order::UpDownLeftRight
    };

    let mut last_position = DirectionalKeypad::A;
    let mut apply = |count: u64, direction: DirectionalKeypad| {
        let mut cost = 0;
        for _ in 0..count {
            cost += get_cost_directional(last_position, direction, robots, memo);
            last_position = direction;
        }
        cost
    };

    (match order {
        Order::UpDownLeftRight => apply(up, DirectionalKeypad::Up) + apply(down, DirectionalKeypad::Down) + apply(left, DirectionalKeypad::Left) + apply(right, DirectionalKeypad::Right),
        Order::LeftRightUpDown => apply(left, DirectionalKeypad::Left) + apply(right, DirectionalKeypad::Right) + apply(up, DirectionalKeypad::Up) + apply(down, DirectionalKeypad::Down),
    }) + apply(1, DirectionalKeypad::A)
}

fn get_cost_directional(from: DirectionalKeypad, to: DirectionalKeypad, robots: usize, memo: &mut HashMap<MemoKey, u64>) -> u64 {
    if robots == 0 {
        // If there are no robots between us and the button then we can just push the button
        return 1;
    }
    if let Some(memo_result) = memo.get(&MemoKey {
        from,
        to,
        robots,
    }) {
        return *memo_result;
    }
    let x = (to.get_position().0 as i64) - (from.get_position().0 as i64);
    let y = (to.get_position().1 as i64) - (from.get_position().1 as i64);

    let up = if y < 0 { (-y) as u64 } else { 0 };
    let down = if y > 0 { y as u64 } else { 0 };
    let left = if x < 0 { (-x) as u64 } else { 0 };
    let right = if x > 0 { x as u64 } else { 0 };

    // Rules for our update ordering
    let order = if from == DirectionalKeypad::Left {
        // Apply x first then y
        Order::LeftRightUpDown
    } else if to == DirectionalKeypad::Left {
        // Apply y first then x
        Order::UpDownLeftRight
    } else if left > 0 {
        Order::LeftRightUpDown
    } else if right > 0 {
        Order::UpDownLeftRight
    } else {
        // If we aren't going sideways order doesn't really matter anyways
        Order::UpDownLeftRight
    };

    let mut last_position = DirectionalKeypad::A;
    let mut apply = |count: u64, direction: DirectionalKeypad| {
        let mut cost = 0;
        for _ in 0..count {
            cost += get_cost_directional(last_position, direction, robots - 1, memo);
            last_position = direction;
        }
        cost
    };

    let result = (match order {
        Order::UpDownLeftRight => apply(up, DirectionalKeypad::Up) + apply(down, DirectionalKeypad::Down) + apply(left, DirectionalKeypad::Left) + apply(right, DirectionalKeypad::Right),
        Order::LeftRightUpDown => apply(left, DirectionalKeypad::Left) + apply(right, DirectionalKeypad::Right) + apply(up, DirectionalKeypad::Up) + apply(down, DirectionalKeypad::Down),
    }) + apply(1, DirectionalKeypad::A);

    memo.insert(MemoKey {
        from,
        to,
        robots,
    }, result);

    result
}

fn get_complexity_sum_path_construction<const ROBOTS: usize>(input: &str) -> u64 {
    // Memoization was ABSOLUTELY necessary here: without it this takes forever
    // Though this also implies to me that there is probably a nice DP solution that I'm too lazy
    // to implement
    let mut memo = HashMap::new();
    let inputs = preprocess(input);
    let mut result = 0;
    for input in inputs {
        let mut length = 0;
        let mut last = NumericKeypad::A;
        for num in input.sequence {
            length += get_cost_numeric(last, num, ROBOTS, &mut memo);
            last = num;
        }
        result += length * input.numeric;
    }
    result
}

#[test]
fn test_part1() {
    assert_eq!(
        126384,
        get_complexity_sum_bfs::<2>(r"029A
980A
179A
456A
379A"
        )
    );
}

#[test]
fn test_part2() {
    let input = r"029A
980A
179A
456A
379A";
    assert_eq!(
        get_complexity_sum_bfs::<1>(input),
        get_complexity_sum_path_construction::<1>(input)
    );
    assert_eq!(
        get_complexity_sum_bfs::<2>(input),
        get_complexity_sum_path_construction::<2>(input)
    );
    assert_eq!(
        get_complexity_sum_bfs::<3>(input),
        get_complexity_sum_path_construction::<3>(input)
    );
    assert_eq!(
        get_complexity_sum_bfs::<4>(input),
        get_complexity_sum_path_construction::<4>(input)
    );
    // 5 and 6 are solvable via BFS but are commented out so that running all the tests isn't slow
    // assert_eq!(
    //     get_complexity_sum_bfs::<5>(input),
    //     get_complexity_sum_path_construction::<5>(input)
    // );
    // assert_eq!(
    //     get_complexity_sum_bfs::<6>(input),
    //     get_complexity_sum_path_construction::<6>(input)
    // );
}