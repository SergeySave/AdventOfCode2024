/*
--- Day 4: Ceres Search ---

"Looks like the Chief's not here. Next!" One of The Historians pulls out a
device and pushes the only button on it. After a brief flash, you recognize
the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station
tugs on your shirt; she'd like to know if you could help her with her word
search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written
backwards, or even overlapping other words. It's a little unusual, though,
as you don't merely need to find one instance of XMAS - you need to find
all of them. Here are a few ways XMAS might appear, where irrelevant
characters have been replaced with .:

..X...
.SAMX.
.A..A.
XMAS.S
.X....

The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX

In this word search, XMAS occurs a total of 18 times; here's the same word
search again, but where letters not involved in any XMAS have been replaced
with . :

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

Take a look at the little Elf's word search. How many times does XMAS
appear?

--- Part Two ---

The Elf looks quizzically at you. Did you misunderstand the assignment?

Looking for the instructions, you flip over the word search to find that
this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're
supposed to find two MAS in the shape of an X. One way to achieve that is
like this:

M.S
.A.
M.S

Irrelevant characters have again been replaced with . in the above diagram.
Within the X, each MAS can be written forwards or backwards.

Here's the same example from before, but this time all of the X-MASes have
been kept instead:

.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........

In this example, an X-MAS appears 9 times.

Flip the word search from the instructions back over to the word search
side and try again. How many times does an X-MAS appear?
 */

use std::fs;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./inputs/04_ceres_search.txt").unwrap();
    let input_lines = input_file.lines().collect_vec();
    println!("{}", get_xmas_count(input_lines.clone()));
    println!("{}", get_x_mas_count(input_lines));
}

const DIRECTIONS: [(i64, i64); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

// The four rotation matrices we will use
const ROTATIONS: [((i64, i64), (i64, i64)); 4] = [
    (
        (1, 0),
        (0, 1),
    ),
    (
      (0, -1),
      (1, 0),
    ),
    (
      (-1, 0),
      (0, -1),
    ),
    (
      (0, 1),
      (-1, 0),
    ),
];
// An "image" of the pattern
const X_MAS: [(i64, i64, char); 5] = [
    (0, 0, 'A'),
    (-1, -1, 'M'),
    (-1, 1, 'M'),
    (1, -1, 'S'),
    (1, 1, 'S'),
];

struct WordSearch<'a> {
    input: Vec<&'a str>
}

impl<'a> WordSearch<'a> {
    fn width(&self) -> i64 {
        self.input[0].len() as i64
    }

    fn height(&self) -> i64 {
        self.input.len() as i64
    }

    fn get(&self, x: i64, y: i64) -> char {
        if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
            return '.';
        }

        // I know that there are no multi-byte characters involved
        self.input[y as usize].as_bytes()[x as usize] as char
    }
}

fn get_xmas_count(input_lines: Vec<&str>) -> u64 {
    // This is basically just how I solve word searches when doing them manually
    let mut count = 0u64;

    let search = WordSearch {
        input: input_lines
    };

    for x in 0..search.width() {
        for y in 0..search.height() {
            'direction_loop: for d in DIRECTIONS {
                for i in 0..XMAS.len() {
                    let x = x + (i as i64) * d.0;
                    let y = y + (i as i64) * d.1;
                    let c = search.get(x, y);
                    if c != XMAS[i] {
                        continue 'direction_loop;
                    }
                }
                // Increment count if we found the whole word
                count += 1;
            }
        }
    }

    count
}

fn get_x_mas_count(input_lines: Vec<&str>) -> u64 {
    // This is like the previous solution but a more complex pattern
    // We're basically just going to try to apply the pattern everywhere we can
    let mut count = 0u64;

    let search = WordSearch {
        input: input_lines
    };

    for x in 0..search.width() {
        for y in 0..search.height() {
            'rotation_loop: for ((x_x, x_y), (y_x, y_y)) in ROTATIONS {
                for (offset_x, offset_y, expected) in X_MAS {
                    let x = x + x_x * offset_x + x_y * offset_y;
                    let y = y + y_x * offset_x + y_y * offset_y;
                    let c = search.get(x, y);
                    if c != expected {
                        continue 'rotation_loop;
                    }
                }
                // Increment count if we found the whole word
                count += 1;
            }
        }
    }

    count
}

#[test]
fn test_part1() {
    assert_eq!(18,
               get_xmas_count(
                   r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".lines().collect_vec()
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!(9,
               get_x_mas_count(
                   r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".lines().collect_vec()
               )
    );
}
