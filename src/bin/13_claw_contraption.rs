/*
--- Day 13: Claw Contraption ---

Next up: the lobby of a resort on a tropical island. The Historians take a
moment to admire the hexagonal floor tiles before spreading out.

Fortunately, it looks like the resort has a new arcade! Maybe you can win
some prizes from the claw machines?

The claw machines here are a little unusual. Instead of a joystick or
directional buttons to control the claw, these machines have two buttons
labeled A and B. Worse, you can't just put in a token and play; it costs 3
tokens to push the A button and 1 token to push the B button.

With a little experimentation, you figure out that each machine's buttons
are configured to move the claw a specific amount to the right (along the X
axis) and a specific amount forward (along the Y axis) each time that
button is pressed.

Each machine contains one prize; to win the prize, the claw must be
positioned exactly above the prize on both the X and Y axes.

You wonder: what is the smallest number of tokens you would have to spend
to win as many prizes as possible? You assemble a list of every machine's
button behavior and prize location (your puzzle input). For example:

Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279

This list describes the button configuration and prize location of four
different claw machines.

For now, consider just the first claw machine in the list:

- Pushing the machine's A button would move the claw 94 units along the
X axis and 34 units along the Y axis.
- Pushing the B button would move the claw 22 units along the X axis and
67 units along the Y axis.
- The prize is located at X=8400, Y=5400; this means that from the
claw's initial position, it would need to move exactly 8400 units
along the X axis and exactly 5400 units along the Y axis to be
perfectly aligned with the prize in this machine.

The cheapest way to win the prize is by pushing the A button 80 times and
the B button 40 times. This would line up the claw along the X axis
(because 80*94 + 40*22 = 8400) and along the Y axis (because
80*34 + 40*67 = 5400). Doing this would cost 80*3 tokens for the A presses
and 40*1 for the B presses, a total of 280 tokens.

For the second and fourth claw machines, there is no combination of A and B
presses that will ever win a prize.

For the third claw machine, the cheapest way to win the prize is by pushing
the A button 38 times and the B button 86 times. Doing this would cost a
total of 200 tokens.

So, the most prizes you could possibly win is two; the minimum tokens you
would have to spend to win all (two) prizes is 480.

You estimate that each button would need to be pressed no more than 100
times to win a prize. How else would someone be expected to play?

Figure out how to win as many prizes as possible. What is the fewest tokens
you would have to spend to win all possible prizes?

--- Part Two ---

As you go to win the first prize, you discover that the claw is nowhere
near where you expected it would be. Due to a unit conversion error in your
measurements, the position of every prize is actually 10000000000000 higher
on both the X and Y axis!

Add 10000000000000 to the X and Y position of every prize. After making
this change, the example above would now look like this:

Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279

Now, it is only possible to win a prize on the second and fourth claw
machines. Unfortunately, it will take many more than 100 presses to do so.

Using the corrected prize coordinates, figure out how to win as many prizes
as possible. What is the fewest tokens you would have to spend to win all
possible prizes?
 */
use std::fs;
use regex::Regex;

fn main() {
    let input_file = fs::read_to_string("./inputs/13_claw_contraption.txt").unwrap();
    println!("{}", get_minimum_tokens_to_win(&input_file, 0));
    println!("{}", get_minimum_tokens_to_win(&input_file, 10000000000000));
}

#[derive(Debug, Default)]
struct Vector2 {
    x: i64,
    y: i64
}

#[derive(Debug)]
struct ClawMachine {
    a_button: Vector2,
    b_button: Vector2,
    prize: Vector2,
}

fn preprocess(input: &str, prize_offset: i64) -> Vec<ClawMachine> {
    let a_button_regex = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
    let b_button_regex = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
    let prize_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
    let mut result = vec![];

    let mut a_button = Vector2::default();
    let mut b_button = Vector2::default();

    for line in input.lines() {
        if let Some(a_capture) = a_button_regex.captures(line) {
            a_button = Vector2 {
                x: a_capture.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                y: a_capture.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            };
        } else if let Some(b_capture) = b_button_regex.captures(line) {
            b_button = Vector2 {
                x: b_capture.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                y: b_capture.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            };
        } else if let Some(prize_capture) = prize_regex.captures(line) {
            let prize = Vector2 {
                x: prize_capture.get(1).unwrap().as_str().parse::<i64>().unwrap() + prize_offset,
                y: prize_capture.get(2).unwrap().as_str().parse::<i64>().unwrap() + prize_offset,
            };
            result.push(ClawMachine {
                a_button,
                b_button,
                prize,
            });
            a_button = Vector2::default();
            b_button = Vector2::default();
        }
    }

    result
}

fn solve_claw_machine(claw_machine: ClawMachine) -> u64 {
    // A = # of times A button needs to be pushed
    // B = # of times B button needs to be pushed

    // prize.x = A * a.x + B * b.x
    // prize.y = A * a.y + B * b.y

    // (prize.x - A * a.x) / b.x = B
    // prize.y = A * a.y + (prize.x - A * a.x) * b.y / b.x
    // prize.y = A * a.y + prize.x*b.y/b.x - A*a.x*b.y/b.x
    // prize.y - prize.x*b.y/b.x = A * a.y - A*a.x*b.y/b.x
    // prize.y - prize.x*b.y/b.x = A * (a.y - a.x*b.y/b.x)
    // (prize.y - prize.x*b.y/b.x) / (a.y - a.x*b.y/b.x) = A

    let prize_x = claw_machine.prize.x as f64;
    let prize_y = claw_machine.prize.y as f64;
    let a_x = claw_machine.a_button.x as f64;
    let a_y = claw_machine.a_button.y as f64;
    let b_x = claw_machine.b_button.x as f64;
    let b_y = claw_machine.b_button.y as f64;

    let a = (prize_y - prize_x*b_y/b_x) / (a_y - a_x*b_y/b_x);
    let b = (prize_x - a * a_x) / b_x;

    let a = a.round().abs() as i64; // Make sure they are positive - this could also break the solution
    let b = b.round().abs() as i64;

    // Check our work (i.e. make sure that the integral number of button pushes works)
    if (claw_machine.a_button.x * a + claw_machine.b_button.x * b == claw_machine.prize.x)
        && (claw_machine.a_button.y * a + claw_machine.b_button.y * b == claw_machine.prize.y) {
        (a as u64) * 3 + (b as u64)
    }
    else {
        0
    }
}

fn get_minimum_tokens_to_win(input: &str, prize_offset: i64) -> u64 {
    let claw_machines = preprocess(input, prize_offset);

    claw_machines.into_iter().map(solve_claw_machine).sum()
}


#[test]
fn test_part1() {
    assert_eq!(480,
               get_minimum_tokens_to_win(
                   r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
                   , 0
               )
    );
}
