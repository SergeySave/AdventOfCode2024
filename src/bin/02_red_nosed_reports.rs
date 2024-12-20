/*
--- Day 2: Red-Nosed Reports ---

Fortunately, the first location The Historians want to search isn't a long
walk from the Chief Historian's office.

While the Red-Nosed Reindeer nuclear fusion/fission plant appears to
contain no sign of the Chief Historian, the engineers there run up to you
as soon as they see you. Apparently, they still talk about the time Rudolph
was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really
appreciate your help analyzing some unusual data from the Red-Nosed
reactor. You turn to check if The Historians are waiting for you, but they
seem to have already divided into groups that are currently searching every
corner of the facility. You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many reports, one report
per line. Each report is a list of numbers called levels that are separated
by spaces. For example:

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9

This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-
Nosed reactor safety systems can only tolerate levels that are either
gradually increasing or gradually decreasing. So, a report only counts as
safe if both of the following are true:
- The levels are either all increasing or all decreasing.
- Any two adjacent levels differ by at least one and at most three.
In the example above, the reports can be found safe or unsafe by checking
those rules:

7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?

--- Part Two ---

The engineers are surprised by the low number of safe reports until they
realize they forgot to tell you about the Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor
safety systems tolerate a single bad level in what would otherwise be a
safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from
an unsafe report would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

7 6 4 2 1: Safe without removing any level.
1 2 7 8 9: Unsafe regardless of which level is removed.
9 7 6 2 1: Unsafe regardless of which level is removed.
1 3 2 4 5: Safe by removing the second level, 3.
8 6 4 4 1: Safe by removing the third level, 4.
1 3 6 7 9: Safe without removing any level.

Thanks to the Problem Dampener, 4 reports are actually safe!

Update your analysis by handling situations where the Problem Dampener can
remove a single level from unsafe reports. How many reports are now safe?
 */

use std::fs;
use std::str::FromStr;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./inputs/02_red_nosed_reports.txt").unwrap();
    let input_lines = input_file.lines();
    println!("{}", get_safe_report_count(input_lines.clone(), is_report_safe));
    println!("{}", get_safe_report_count(input_lines.clone(), is_report_safe_problem_dampener_brute_force));
}

/// A tri-state enum to represent the direction of a sequence
#[derive(Eq, PartialEq, Copy, Clone)]
enum ChangeDirection {
    Increasing,
    Decreasing,
    Unknown
}

/// A simple solution which only works for part 1
fn is_report_safe(report: &Vec<u64>) -> bool {
    let mut direction = ChangeDirection::Unknown;

    for adjacent_pair in report.windows(2) {
        let first = adjacent_pair[0];
        let second = adjacent_pair[1];
        let difference = first.abs_diff(second);

        // Any two adjacent levels differ by at least one and at most three
        if !(difference >= 1 && difference <= 3) {
            return false;
        }

        assert_ne!(first, second, "Since there is a difference these cannot be equal");
        let expected_direction = if first < second { ChangeDirection::Increasing } else { ChangeDirection::Decreasing };

        if direction != ChangeDirection::Unknown && expected_direction != direction {
            return false;
        }
        direction = expected_direction;
    }

    true
}

/// An extension to the simple solution that allows for part 2 to be solved (by brute force)
fn is_report_safe_problem_dampener_brute_force(report: &Vec<u64>) -> bool {
    if is_report_safe(&report) {
        return true;
    }

    let mut test_report = Vec::<u64>::with_capacity(report.len() - 1);
    for i in 0..report.len() {
        test_report.clear();
        test_report.extend_from_slice(&report[..i]);
        test_report.extend_from_slice(&report[(i+1)..]);
        if is_report_safe(&test_report) {
            return true;
        }
    }

    false
}

fn get_safe_report_count<'a>(input_lines: impl Iterator<Item=&'a str>, report_safe_fn: impl Fn(&Vec<u64>)->bool) -> u64 {
    let reports = input_lines.into_iter()
        .map(|report|
            report.split_whitespace()
                .map(|level| u64::from_str(level).unwrap())
                .collect_vec()
        ).collect_vec();

    reports.into_iter()
        .filter(|report| report_safe_fn(&report))
        .count() as u64
}

#[test]
fn test_part1() {
    assert_eq!(2,
               get_safe_report_count(
                   r"7 6 4 2 1
                    1 2 7 8 9
                    9 7 6 2 1
                    1 3 2 4 5
                    8 6 4 4 1
                    1 3 6 7 9".lines(),
                   is_report_safe
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!(4,
               get_safe_report_count(
                   r"7 6 4 2 1
                    1 2 7 8 9
                    9 7 6 2 1
                    1 3 2 4 5
                    8 6 4 4 1
                    1 3 6 7 9".lines(),
                   is_report_safe_problem_dampener_brute_force
               )
    );
}
