/*
--- Day 14: Restroom Redoubt ---

One of The Historians needs to use the bathroom; fortunately, you know
there's a bathroom near an unvisited location on their list, and so you're
all quickly teleported directly to the lobby of Easter Bunny Headquarters.

Unfortunately, EBHQ seems to have "improved" bathroom security again after
your last visit. The area outside the bathroom is swarming with robots!

To get The Historian safely to the bathroom, you'll need a way to predict
where the robots will be in the future. Fortunately, they all seem to be
moving on the tile floor in predictable straight lines.

You make a list (your puzzle input) of all of the robots' current positions
(p) and velocities (v), one robot per line. For example:

p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3

Each robot's position is given as p=x,y where x represents the number of
tiles the robot is from the left wall and y represents the number of tiles
from the top wall (when viewed from above). So, a position of p=0,0 means
the robot is all the way in the top-left corner.

Each robot's velocity is given as v=x,y where x and y are given in tiles
per second. Positive x means the robot is moving to the right, and positive
y means the robot is moving down. So, a velocity of v=1,-2 means that each
second, the robot moves 1 tile to the right and 2 tiles up.

The robots outside the actual bathroom are in a space which is 101 tiles
wide and 103 tiles tall (when viewed from above). However, in this example,
the robots are in a space which is only 11 tiles wide and 7 tiles tall.

The robots are good at navigating over/under each other (due to a
combination of springs, extendable legs, and quadcopters), so they can
share the same tile and don't interact with each other. Visually, the
number of robots on each tile in this example looks like this:

1.12.......
...........
...........
......11.11
1.1........
.........1.
.......1...

These robots have a unique feature for maximum bathroom security: they can
teleport. When a robot would run into an edge of the space they're in, they
instead teleport to the other side, effectively wrapping around the edges.
Here is what robot p=2,4 v=2,-3 does for the first few seconds:

Initial state:
...........
...........
...........
...........
..1........
...........
...........

After 1 second:
...........
....1......
...........
...........
...........
...........
...........

After 2 seconds:
...........
...........
...........
...........
...........
......1....
...........

After 3 seconds:
...........
...........
........1..
...........
...........
...........
...........

After 4 seconds:
...........
...........
...........
...........
...........
...........
..........1

After 5 seconds:
...........
...........
...........
.1.........
...........
...........
...........

The Historian can't wait much longer, so you don't have to simulate the
robots for very long. Where will the robots be after 100 seconds?

In the above example, the number of robots on each tile after 100 seconds
has elapsed looks like this:

......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1....
To determine the safest area, count the number of robots in each quadrant
after 100 seconds. Robots that are exactly in the middle (horizontally or
vertically) don't count as being in any quadrant, so the only relevant
robots are:

..... 2..1.
..... .....
1.... .....

..... .....
...12 .....
.1... 1....

In this example, the quadrants contain 1, 3, 4, and 1 robot. Multiplying
these together gives a total safety factor of 12.

Predict the motion of the robots in your list within a space which is 101
tiles wide and 103 tiles tall. What will the safety factor be after exactly
100 seconds have elapsed?

--- Part Two ---

During the bathroom break, someone notices that these robots seem awfully
similar to ones built and used at the North Pole. If they're the same type
of robots, they should have a hard-coded Easter egg: very rarely, most of
the robots should arrange themselves into a picture of a Christmas tree.

What is the fewest number of seconds that must elapse for the robots to
display the Easter egg?
 */
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let input_file = fs::read_to_string("./inputs/14_restroom_redoubt.txt").unwrap();
    println!("{}", get_safety_factor(&input_file, 101, 103));
    // This value of 6446 was found by manual verification.
    // Since the two dimensions are 101 and 103, vertical motion is modulo 103 and horizontal motion
    // is modulo 101
    // Through dumb luck I noticed that 60 seconds into the problem a large number of robots appeared
    // to gather at around the same height value. 59 and 61 seconds both appeared nearly random.
    // So this told me that 60 (mod 103) was the right y value
    // I then stepped from 60 by 103 until I found a value where the x axis was correct (as I could
    // tell because there was clearly a tree structure).
    display_robots(&input_file, 6446, 101, 103);

    let mut seconds = 60;
    for _ in 0..101 {
        println!("{seconds}");
        display_robots(&input_file, seconds, 101, 103);
        seconds += 103;
        sleep(Duration::from_millis(2000));
    }
}

#[derive(Debug, Default)]
struct Vector2 {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Robot {
    pos: Vector2,
    vel: Vector2,
}

impl Robot {
    fn step(&mut self, seconds: i64, width: u64, height: u64) {
        self.pos.x += self.vel.x * seconds;
        self.pos.y += self.vel.y * seconds;

        self.pos.x = self.pos.x.rem_euclid(width as i64);
        self.pos.y = self.pos.y.rem_euclid(height as i64);
    }
}

fn preprocess(input: &str) -> Vec<Robot> {
    let robot_pattern = Regex::new(r"^p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)$").unwrap();
    input.lines()
        .map(|line| {
            let robot_capture = robot_pattern.captures(line).unwrap();
            Robot {
                pos: Vector2 {
                    x: robot_capture.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    y: robot_capture.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                },
                vel: Vector2 {
                    x: robot_capture.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                    y: robot_capture.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                }
            }
        })
        .collect_vec()
}

fn get_safety_factor(input: &str, width: u64, height: u64) -> u64 {
    let mut robots = preprocess(input);
    robots.iter_mut().for_each(|robot| robot.step(100, width, height));

    let x_div = (width / 2) as i64;
    let y_div = (height / 2) as i64;

    let mut quadrants = [0_u64; 4];
    robots.into_iter().for_each(|robot| {
        if robot.pos.x < x_div && robot.pos.y < y_div {
            quadrants[0] += 1;
        }
        if robot.pos.x > x_div && robot.pos.y < y_div {
            quadrants[1] += 1;
        }
        if robot.pos.x < x_div && robot.pos.y > y_div {
            quadrants[2] += 1;
        }
        if robot.pos.x > x_div && robot.pos.y > y_div {
            quadrants[3] += 1;
        }
    });

    quadrants.into_iter().product()
}

fn display_robots(input: &str, seconds: i64, width: u64, height: u64) {
    let mut robots = preprocess(input);
    robots.iter_mut().for_each(|robot| robot.step(seconds, width, height));

    let mut tiles = vec![vec![0_u64; width as usize]; height as usize];
    for robot in robots {
        tiles[robot.pos.y as usize][robot.pos.x as usize] += 1;
    }

    for row in tiles {
        for tile in row {
            if tile >= 10 {
                print!("X");
            } else if tile > 0 {
                print!("{tile}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[test]
fn test_part1() {
    assert_eq!(12,
               get_safety_factor(
                   r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3", 11, 7
               )
    );
}
