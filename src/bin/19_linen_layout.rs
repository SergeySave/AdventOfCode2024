/*
--- Day 19: Linen Layout ---

Today, The Historians take you up to the hot springs on Gear Island! Very
suspiciously, absolutely nothing goes wrong as they begin their careful
search of the vast field of helixes.

Could this finally be your chance to visit the onsen next door? Only one
way to find out.

After a brief conversation with the reception staff at the onsen front
desk, you discover that you don't have the right kind of money to pay the
admission fee. However, before you can leave, the staff get your attention.
Apparently, they've heard about how you helped at the hot springs, and
they're willing to make a deal: if you can simply help them arrange their
towels, they'll let you in for free!

Every towel at this onsen is marked with a pattern of colored stripes.
There are only a few patterns, but for any particular pattern, the staff
can get you as many towels with that pattern as you need. Each stripe can
be white (w), blue (u), black (b), red (r), or green (g). So, a towel with
the pattern ggr would have a green stripe, a green stripe, and then a red
stripe, in that order. (You can't reverse a pattern by flipping a towel
upside-down, as that would cause the onsen logo to face the wrong way.)

The Official Onsen Branding Expert has produced a list of designs - each a
long sequence of stripe colors - that they would like to be able to
display. You can use any towels you want, but all of the towels' stripes
must exactly match the desired design. So, to display the design rgrgr, you
could use two rg towels and then an r towel, an rgr towel and then a gr
towel, or even a single massive rgrgr towel (assuming such towel patterns
were actually available).

To start, collect together all of the available towel patterns and the list
of desired designs (your puzzle input). For example:

r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb

The first line indicates the available towel patterns; in this example, the
onsen has unlimited towels with a single red stripe (r), unlimited towels
with a white stripe and then a red stripe (wr), and so on.

After the blank line, the remaining lines each describe a design the onsen
would like to be able to display. In this example, the first design (brwrr)
indicates that the onsen would like to be able to display a black stripe, a
red stripe, a white stripe, and then two red stripes, in that order.

Not all designs will be possible with the available towels. In the above
example, the designs are possible or impossible as follows:

- brwrr can be made with a br towel, then a wr towel, and then finally
an r towel.
- bggr can be made with a b towel, two g towels, and then an r towel.
- gbbr can be made with a gb towel and then a br towel.
- rrbgbr can be made with r, rb, g, and br.
- ubwu is impossible.
- bwurrg can be made with bwu, r, r, and g.
- brgr can be made with br, g, and r.
- bbrgwb is impossible.

In this example, 6 of the eight designs are possible with the available
towel patterns.

To get into the onsen as soon as possible, consult your list of towel
patterns and desired designs carefully. How many designs are possible?

--- Part Two ---

The staff don't really like some of the towel arrangements you came up
with. To avoid an endless cycle of towel rearrangement, maybe you should
just give them every possible option.

Here are all of the different ways the above example's designs can be made:

brwrr can be made in two different ways: b, r, wr, r or br, wr, r.

bggr can only be made with b, g, g, and r.

gbbr can be made 4 different ways:

- g, b, b, r
- g, b, br
- gb, b, r
- gb, br

rrbgbr can be made 6 different ways:

- r, r, b, g, b, r
- r, r, b, g, br
- r, r, b, gb, r
- r, rb, g, b, r
- r, rb, g, br
- r, rb, gb, r

bwurrg can only be made with bwu, r, r, and g.

brgr can be made in two different ways: b, r, g, r or br, g, r.

ubwu and bbrgwb are still impossible.

Adding up all of the ways the towels in this example could be arranged into
the desired designs yields 16 (2 + 1 + 4 + 6 + 1 + 2).

They'll let you into the onsen as soon as you have the list. What do you
get if you add up the number of different ways you could make each design?
 */
use std::fs;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input_file = fs::read_to_string("./inputs/19_linen_layout.txt").unwrap();
    println!("{}", get_possible_pattern_count(&input_file));
    println!("{}", get_total_arrangements(&input_file));
}

fn get_possible_pattern_count(input: &str) -> u64 {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(',').map(|section| section.trim()).collect_vec();
    let any_towel_regex = Regex::new(&towels.join("|")).unwrap();
    let valid_towel_pattern_regex = Regex::new(&format!("^({})+$", any_towel_regex.as_str())).unwrap();
    let _ = lines.next();
    lines.filter(|line| {
        valid_towel_pattern_regex.is_match(line)
    }).count() as u64
}

fn get_pattern_count(towels: &Vec<Vec<char>>, pattern: Vec<char>) -> u64 {
    // Each index in this is the number of achievable patterns that match the given prefix length
    // This is offset by 1 to account for the "null pattern" which is always achievable by applying
    // no towels
    let mut counts = vec![0u64; pattern.len() + 1];
    counts[0] = 1;

    for i in 1..=pattern.len() {
        // Let's try to apply all of the towels
        for towel in towels {
            let start = i.saturating_sub(towel.len());
            // If the towel matches the pattern
            if &towel[..] == &pattern[start..i] {
                counts[i] += counts[start];
            }
        }
    }

    counts[pattern.len()]
}

fn get_total_arrangements(input: &str) -> u64 {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(',').map(|section| section.trim().chars().collect_vec()).collect_vec();

    let _ = lines.next();

    lines
        .map(|line| get_pattern_count(&towels, line.chars().collect_vec()))
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(6,
               get_possible_pattern_count(
                   r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!(16,
               get_total_arrangements(
                   r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
               )
    );
}