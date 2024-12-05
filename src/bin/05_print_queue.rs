/*
--- Day 5: Print Queue ---

Satisfied with their search on Ceres, the squadron of scholars suggests
subsequently scanning the stationery stacks of sub-basement 17.

The North Pole printing department is busier than ever this close to
Christmas, and while The Historians continue their search of this
historically significant facility, an Elf operating a very familiar printer
beckons you over.

The Elf must recognize you, because they waste no time explaining that the
new sleigh launch safety manual updates won't print correctly. Failure to
update the safety manuals would be dire indeed, so you offer your services.

Safety protocols clearly indicate that new pages for the safety manuals
must be printed in a very specific order. The notation X|Y means that if
both page number X and page number Y are to be produced as part of an
update, page number X must be printed at some point before page number Y.

The Elf has for you both the page ordering rules and the pages to produce
in each update (your puzzle input), but can't figure out whether each
update has the pages in the right order.

For example:

47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47

The first section specifies the page ordering rules, one per line. The
first rule, 47|53, means that if an update includes both page number 47 and
page number 53, then page number 47 must be printed at some point before
page number 53. (47 doesn't necessarily need to be immediately before 53;
other pages are allowed to be between them.)

The second section specifies the page numbers of each update. Because most
safety manuals are different, the pages needed in the updates are different
too. The first update, 75,47,61,53,29, means that the update consists of
page numbers 75, 47, 61, 53, and 29.

To get the printers going as soon as possible, start by identifying which
updates are already in the right order.

In the above example, the first update (75,47,61,53,29) is in the right
order:

- 75 is correctly first because there are rules that put each other page
after it: 75|47, 75|61, 75|53, and 75|29.
- 47 is correctly second because 75 must be before it (75|47) and every
other page must be after it according to 47|61, 47|53, and 47|29.
- 61 is correctly in the middle because 75 and 47 are before it (75|61
and 47|61) and 53 and 29 are after it (61|53 and 61|29).
- 53 is correctly fourth because it is before page number 29 (53|29).
- 29 is the only page left and so is correctly last.

Because the first update does not include some page numbers, the ordering
rules involving those missing page numbers are ignored.

The second and third updates are also in the correct order according to the
rules. Like the first update, they also do not include every page number,
and so only some of the ordering rules apply - within each update, the
ordering rules that involve missing page numbers are not used.

The fourth update, 75,97,47,61,53, is not in the correct order: it would
print 75 before 97, which violates the rule 97|75.

The fifth update, 61,13,29, is also not in the correct order, since it
breaks the rule 29|13.

The last update, 97,13,75,29,47, is not in the correct order due to
breaking several rules.

For some reason, the Elves also need to know the middle page number of each
update being printed. Because you are currently only printing the
correctly-ordered updates, you will need to find the middle page number of
each correctly-ordered update. In the above example, the correctly-ordered
updates are:

75,47,61,53,29
97,61,53,29,13
75,29,13

These have middle page numbers of 61, 53, and 29 respectively. Adding
these page numbers together gives 143.

Of course, you'll need to be careful: the actual list of page ordering
rules is bigger and more complicated than the above example.

Determine which updates are already in the correct order. What do you get
if you add up the middle page number from those correctly-ordered updates?

--- Part Two ---

While the Elves get to work printing the correctly-ordered updates, you
have a little time to fix the rest of them.

For each of the incorrectly-ordered updates, use the page ordering rules to
put the page numbers in the right order. For the above example, here are
the three incorrectly-ordered updates and their correct orderings:

75,97,47,61,53 becomes 97,75,47,61,53.
61,13,29 becomes 61,29,13.
97,13,75,29,47 becomes 97,75,47,29,13.

After taking only the incorrectly-ordered updates and ordering them
correctly, their middle page numbers are 47, 29, and 47. Adding these
together produces 123.

Find the updates which are not in the correct order. What do you get if you
add up the middle page numbers after correctly ordering just those updates?
 */
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./inputs/05_print_queue.txt").unwrap();
    println!("{}", get_sum_correct_middle_page_numbers(&input_file));
    println!("{}", get_sum_incorrect_middle_page_numbers(&input_file));
}

/// A helper that parses the page ordering rules into a 1:many mapping of before -> after
/// This then runs through all the page updates and determines if they are correct or incorrect
/// It then applies mapping functions to determine the value of the correct and incorrect mappings
/// And finally sums up the values of all updates
fn eval_on_pages(
    input: &str,
    on_incorrect: impl Fn(&HashMap<u64, HashSet<u64>>, Vec<u64>)->u64,
    on_correct: impl Fn(&HashMap<u64, HashSet<u64>>, Vec<u64>)->u64
) -> u64 {
    let mut before_mapping = HashMap::<u64, HashSet<u64>>::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let Some((left, right)) = line.split_once('|') else { break; };
        let before = left.parse::<u64>().unwrap();
        let after = right.parse::<u64>().unwrap();
        if !before_mapping.contains_key(&before) {
            before_mapping.insert(before, HashSet::new());
        }
        before_mapping.get_mut(&before).unwrap().insert(after);
    }

    lines.map(|line| {
        let pages = line.split(',').map(|page| page.parse::<u64>().unwrap()).collect_vec();
        let mut previous_pages = HashSet::new();
        for page in &pages {
            // If there is nothing that needs to appear after this then no problem
            if let Some(after_set) = before_mapping.get(&page) {
                // If one of the previous pages needs to be after this page then this isn't valid
                if !after_set.is_disjoint(&previous_pages) {
                    return on_incorrect(&before_mapping, pages);
                }
            }
            previous_pages.insert(*page);
        }
        on_correct(&before_mapping, pages)
    }).sum()
}

/// Solve part 1
fn get_sum_correct_middle_page_numbers(input: &str) -> u64 {
    eval_on_pages(input, |_, _| 0, |_, pages| pages[pages.len() / 2])
}

/// Solve part 2
fn get_sum_incorrect_middle_page_numbers(input: &str) -> u64 {
    eval_on_pages(input, |before_mapping, pages| {
        // Using DFS to perform a topological sorting
        let mut sorted_pages = Vec::with_capacity(pages.len());
        let mut unvisited_pages = BTreeSet::from_iter(pages.iter().cloned());
        let mut in_progress_pages = HashSet::new();

        fn visit(page: u64, before_mapping: &HashMap<u64, HashSet<u64>>, unvisited_pages: &mut BTreeSet<u64>, in_progress_pages: &mut HashSet<u64>, sorted_pages: &mut Vec<u64>) {
            if !unvisited_pages.contains(&page) {
                return
            }
            if in_progress_pages.contains(&page) {
                panic!("cycle detected!"); // This shouldn't happen due to the structure of this problem
            }

            in_progress_pages.insert(page);

            // If there is nothing that needs to appear after this then no problem
            if let Some(after_set) = before_mapping.get(&page) {
                for next_page in after_set {
                    visit(*next_page, before_mapping, unvisited_pages, in_progress_pages, sorted_pages);
                }
            }

            in_progress_pages.remove(&page);
            unvisited_pages.remove(&page);
            sorted_pages.insert(0, page);
        }

        while let Some(page) = unvisited_pages.first() {
            visit(*page, before_mapping, &mut unvisited_pages, &mut in_progress_pages, &mut sorted_pages);
        }

        sorted_pages[sorted_pages.len() / 2]
    }, |_, _| 0)
}

#[test]
fn test_part1() {
    assert_eq!(143,
               get_sum_correct_middle_page_numbers(
                   r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!(123,
               get_sum_incorrect_middle_page_numbers(
                   r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
               )
    );
}
