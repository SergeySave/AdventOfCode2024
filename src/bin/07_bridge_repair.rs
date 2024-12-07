/*
--- Day 7: Bridge Repair ---

The Historians take you to a familiar rope bridge over a river in the
middle of a jungle. The Chief isn't on this side of the bridge, though;
maybe he's on the other side?

When you go to cross the bridge, you notice a group of engineers trying to
repair it. (Apparently, it breaks pretty frequently.) You won't be able to
cross until it's fixed.

You ask how long it'll take; the engineers tell you that it only needs
final calibrations, but some young elephants were playing nearby and stole
all the operators from their calibration equations! They could finish the
calibrations if only someone could determine which test values could
possibly be produced by placing any combination of operators into their
calibration equations (your puzzle input).

For example:

190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20

Each line represents a single equation. The test value appears before the
colon on each line; it is your job to determine whether the remaining
numbers can be combined with operators to produce the test value.

Operators are always evaluated left-to-right, not according to precedence
rules. Furthermore, numbers in the equations cannot be rearranged. Glancing
into the jungle, you can see elephants holding two different types of
operators: add (+) and multiply (*).

Only three of the above equations can be made true by inserting operators:

- 190: 10 19 has only one position that accepts an operator: between 10
and 19. Choosing + would give 29, but choosing * would give the test
value (10 * 19 = 190).
- 3267: 81 40 27 has two positions for operators. Of the four possible
configurations of the operators, two cause the right side to match the
test value: 81 + 40 * 27 and 81 * 40 + 27 both equal 3267 (when
evaluated left-to-right)!
- 292: 11 6 16 20 can be solved in exactly one way: 11 + 6 * 16 + 20.

The engineers just need the total calibration result, which is the sum of
the test values from just the equations that could possibly be true. In the
above example, the sum of the test values for the three equations listed
above is 3749.

Determine which equations could possibly be true. What is their total
calibration result?

--- Part Two ---

The engineers seem concerned; the total calibration result you gave them is
nowhere close to being within safety tolerances. Just then, you spot your
mistake: some well-hidden elephants are holding a third type of operator.

The concatenation operator (||) combines the digits from its left and right
inputs into a single number. For example, 12 || 345 would become 12345. All
operators are still evaluated left-to-right.

Now, apart from the three equations that could be made true using only
addition and multiplication, the above example has three more equations
that can be made true by inserting operators:

- 156: 15 6 can be made true through a single concatenation: 15 || 6 = 156.
- 7290: 6 8 6 15 can be made true using 6 * 8 || 6 * 15.
- 192: 17 8 14 can be made true using 17 || 8 + 14.

Adding up all six test values (the three that could be made before using
only + and * plus the new three that can now be made by also using ||)
produces the new total calibration result of 11387.

Using your new knowledge of elephant hiding spots, determine which
equations could possibly be true. What is their total calibration result?
 */

use itertools::Itertools;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("./inputs/07_bridge_repair.txt").unwrap();
    println!("{}", get_sum_possibly_true_equations(&input_file, false));
    println!("{}", get_sum_possibly_true_equations(&input_file, true));
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    terms: Vec<u64>,
}

fn preprocess(input: &str) -> Vec<Equation> {
    input.lines().filter_map(|line| {
        let Some((test_value, terms)) = line.split_once(": ") else { return None; };

        let test_value = test_value.parse::<u64>().expect("Couldn't parse test value");
        let terms = terms.split_whitespace().map(|term| term.parse::<u64>().expect("Couldn't parse term")).collect_vec();

        Some(Equation {
            test_value,
            terms
        })
    }).collect_vec()
}

fn concat(a: u64, b: u64) -> u64 {
    let mut multiplier = 1u64;
    while multiplier <= b {
        multiplier *= 10;
    }

    a * multiplier + b
}

fn is_equation_possibly_true(equation: &Equation, allow_concat: bool) -> bool {
    // This solution relies on this assumption (i.e. the result of the equation is never decreasing)
    // Though it only uses this assumption as an optimization
    assert!(equation.terms.iter().all(|x| *x > 0));

    fn helper(target: u64, result: u64, remaining_terms: &[u64], allow_concat: bool) -> bool {
        if remaining_terms.is_empty() {
            return target == result;
        }
        if result > target {
            return false;
        }
        // Recurse trying both addition and multiplication
        // We try multiplication first as it's going to get us to bigger numbers quicker
        helper(target, result * remaining_terms[0], &remaining_terms[1..], allow_concat)
            || helper(target, result + remaining_terms[0], &remaining_terms[1..], allow_concat)
            || (allow_concat && helper(target, concat(result, remaining_terms[0]), &remaining_terms[1..], allow_concat))
    }

    helper(equation.test_value, 0, &equation.terms, allow_concat)
}

fn get_sum_possibly_true_equations(input: &str, allow_concat: bool) -> u64 {
    preprocess(input)
        .into_iter()
        .filter(|eq| is_equation_possibly_true(eq, allow_concat))
        .map(|x| x.test_value)
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(3749,
               get_sum_possibly_true_equations(
                   r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
                   false
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!(11387,
               get_sum_possibly_true_equations(
                   r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
                   true
               )
    );
}
