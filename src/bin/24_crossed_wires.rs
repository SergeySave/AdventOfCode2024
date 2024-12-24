/*
--- Day 24: Crossed Wires ---

You and The Historians arrive at the edge of a large grove somewhere in the
jungle. After the last incident, the Elves installed a small device that
monitors the fruit. While The Historians search the grove, one of them asks
if you can take a look at the monitoring device; apparently, it's been
malfunctioning recently.

The device seems to be trying to produce a number through some boolean
logic gates. Each gate has two inputs and one output. The gates all operate
on values that are either true (1) or false (0).

- AND gates output 1 if both inputs are 1; if either input is 0, these
gates output 0.
- OR gates output 1 if one or both inputs is 1; if both inputs are 0,
these gates output 0.
- XOR gates output 1 if the inputs are different; if the inputs are the
same, these gates output 0.

Gates wait until both inputs are received before producing output; wires
can carry 0, 1 or no value at all. There are no loops; once a gate has
determined its output, the output will not change until the whole system is
reset. Each wire is connected to at most one gate output, but can be
connected to many gate inputs.

Rather than risk getting shocked while tinkering with the live system, you
write down all of the gate connections and initial wire values (your puzzle
input) so you can consider them in relative safety. For example:

x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02

Because gates wait for input, some wires need to start with a value (as
inputs to the entire system). The first section specifies these values. For
example, x00: 1 means that the wire named x00 starts with the value 1 (as
if a gate is already outputting that value onto that wire).

The second section lists all of the gates and the wires connected to them.
For example, x00 AND y00 -> z00 describes an instance of an AND gate which
has wires x00 and y00 connected to its inputs and which will write its
output to wire z00.

In this example, simulating these gates eventually causes 0 to appear on
wire z00, 0 to appear on wire z01, and 1 to appear on wire z02.

Ultimately, the system is trying to produce a number by combining the bits
on all wires starting with z. z00 is the least significant bit, then z01,
then z02, and so on.

In this example, the three output bits form the binary number 100 which is
equal to the decimal number 4.

Here's a larger example:

x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj

After waiting for values on all wires starting with z, the wires in this
system have the following values:

bfw: 1
bqk: 1
djm: 1
ffh: 0
fgs: 1
frj: 1
fst: 1
gnj: 1
hwm: 1
kjc: 0
kpj: 1
kwq: 0
mjb: 1
nrd: 1
ntg: 0
pbm: 1
psh: 1
qhw: 1
rvg: 0
tgd: 0
tnw: 1
vdt: 1
wpb: 0
z00: 0
z01: 0
z02: 0
z03: 1
z04: 0
z05: 1
z06: 1
z07: 1
z08: 1
z09: 1
z10: 1
z11: 0
z12: 0

Combining the bits from all wires starting with z produces the binary
number 0011111101000. Converting this number to decimal produces 2024.

Simulate the system of gates and wires. What decimal number does it output
on the wires starting with z?

--- Part Two ---

After inspecting the monitoring device more closely, you determine that the
system you're simulating is trying to add two binary numbers.

Specifically, it is treating the bits on wires starting with x as one
binary number, treating the bits on wires starting with y as a second
binary number, and then attempting to add those two numbers together. The
output of this operation is produced as a binary number on the wires
starting with z. (In all three cases, wire 00 is the least significant bit,
then 01, then 02, and so on.)

The initial values for the wires in your puzzle input represent just one
instance of a pair of numbers that sum to the wrong value. Ultimately, any
two binary numbers provided as input should be handled correctly. That is,
for any combination of bits on wires starting with x and wires starting
with y, the sum of the two numbers those bits represent should be produced
as a binary number on the wires starting with z.

For example, if you have an addition system with four x wires, four y
wires, and five z wires, you should be able to supply any four-bit number
on the x wires, any four-bit number on the y numbers, and eventually find
the sum of those two numbers as a five-bit number on the z wires. One of
the many ways you could provide numbers to such a system would be to pass
11 on the x wires (1011 in binary) and 13 on the y wires (1101 in binary):

x00: 1
x01: 1
x02: 0
x03: 1
y00: 1
y01: 0
y02: 1
y03: 1

If the system were working correctly, then after all gates are finished
processing, you should find 24 (11+13) on the z wires as the five-bit
binary number 11000:

z00: 0
z01: 0
z02: 0
z03: 1
z04: 1

Unfortunately, your actual system needs to add numbers with many more bits
and therefore has many more wires.

Based on forensic analysis of scuff marks and scratches on the device, you
can tell that there are exactly four pairs of gates whose output wires have
been swapped. (A gate can only be in at most one such pair; no gate's
output was swapped multiple times.)

For example, the system below is supposed to find the bitwise AND of the
six-bit number on x00 through x05 and the six-bit number on y00 through
y05 and then write the result as a six-bit number on z00 through z05:

x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00

However, in this example, two pairs of gates have had their output wires
swapped, causing the system to produce wrong answers. The first pair of
gates with swapped outputs is x00 AND y00 -> z05 and x05 AND y05 -> z00;
the second pair of gates is x01 AND y01 -> z02 and x02 AND y02 -> z01.
Correcting these two swaps results in this system that works as intended
for any set of initial values on wires that start with x or y:

x00 AND y00 -> z00
x01 AND y01 -> z01
x02 AND y02 -> z02
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z05

In this example, two pairs of gates have outputs that are involved in a
swap. By sorting their output wires' names and joining them with commas,
the list of wires involved in swaps is z00,z01,z02,z05.

Of course, your actual system is much more complex than this, and the gates
that need their outputs swapped could be anywhere, not just attached to a
wire starting with z. If you were to determine that you need to swap output
wires aaa with eee, ooo with z99, bbb with ccc, and aoc with z24, your
answer would be aaa,aoc,bbb,ccc,eee,ooo,z24,z99.

Your system of gates and wires has four pairs of gates which need their
output wires swapped - eight wires in total. Determine which four pairs of
gates need their outputs swapped so that your system correctly performs
addition; what do you get if you sort the names of the eight wires involved
in a swap and then join those names with commas?
 */
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::Shl;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use rayon::iter::ParallelIterator;

fn main() {
    let input_file = fs::read_to_string("./inputs/24_crossed_wires.txt").unwrap();
    println!("{}", get_decimal_z_output(&input_file));

    let mut labels = get_all_labels(&input_file);
    labels.sort();

    // There are four wrong outputs and four things that need to be swapped so I suspect each
    // swap corresponds to one of those outputs
    (0..labels.len()).into_par_iter().for_each(|a| {
        for b in (a + 1)..labels.len() {
            let swaps = [
                // (labels[2].clone(), labels[55].clone()),
                (labels[a].clone(), labels[b].clone())
            ];
            let mut wrong_count = 0;
            for i in 0..=44 {
                let y = 1 << i;
                let x = 0;
                let z = get_decimal_z_output_x_y(&input_file, x, y, &swaps);
                let expected_z = x + y;
                if z != expected_z {
                    wrong_count += 1;
                }
            }
            if wrong_count < 4 {
                println!("IMPROVEMENT: Wrong Count: {wrong_count} @ a={a} b={b}");
            } else {
                // println!("Wrong Count: {wrong_count} @ a={a} b={b}");
            }
        }
        println!("a={a}");
    });

    // Ok I found 18 potential improvements: lets try all combinations of 4 of them:
    const POTENTIAL_IMPROVEMENTS: &[(usize, usize)] = &[
        (13, 89),
        (13, 174),
        (2, 55),
        (174, 175),
        (174, 190),
        (30, 184),
        (153, 184),
        (65, 89),
        (8, 198),
        (165, 174),
        (47, 55),
        (65, 174),
        (47, 136),
        (47, 205),
        (69, 198),
        (40, 198),
        (89, 165),
        (89, 190),
    ];
    generate_all_four_swaps(POTENTIAL_IMPROVEMENTS).par_iter().for_each(|swaps| {
        // for b in (a + 1)..labels.len() {
            let swaps = [
                (labels[swaps[0].0].clone(), labels[swaps[0].1].clone()),
                (labels[swaps[1].0].clone(), labels[swaps[1].1].clone()),
                (labels[swaps[2].0].clone(), labels[swaps[2].1].clone()),
                (labels[swaps[3].0].clone(), labels[swaps[3].1].clone()),
            ];
            let mut wrong_count = 0;
            'outer_loop: for i in 0..=44 {
                let y = 1 << i;
                for j in 0..=44 {
                    let x = 1 << j;
                    let z = get_decimal_z_output_x_y(&input_file, x, y, &swaps);
                    let expected_z = x + y;
                    if z != expected_z {
                        wrong_count += 1;
                        break 'outer_loop;
                    }
                }
            }
            if wrong_count == 0 {
                println!("We found it!");
                println!("{} <-> {}", swaps[0].0, swaps[0].1);
                println!("{} <-> {}", swaps[1].0, swaps[1].1);
                println!("{} <-> {}", swaps[2].0, swaps[2].1);
                println!("{} <-> {}", swaps[3].0, swaps[3].1);
            }
    });

    // The following four passed the above check so lets see if this actually works

    // wrm <-> wss
    // thm <-> z08
    // gbs <-> grd
    // hwq <-> z22
    //
    // wrm <-> wss
    // thm <-> z08
    // gbs <-> grd
    // fjs <-> z22
    //
    // wrm <-> wss
    // thm <-> z08
    // gbs <-> z29
    // hwq <-> z22
    //
    // wrm <-> wss
    // thm <-> z08
    // gbs <-> z29
    // fjs <-> z22

    // Sort the results
    // This one passed!

    let mut results = vec![
        "wrm", "wss",
        "thm", "z08",
        "gbs", "z29",
        "hwq", "z22",
    ];
    results.sort();
    println!("{}", results.iter().join(","));
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
struct Label([char; 3]);

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        let chars = value.chars().collect_vec();
        Self([chars[0], chars[1], chars[2]])
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GateType {
    And,
    Or,
    Xor
}

#[derive(Debug, Clone)]
struct Gate {
    gate_type: GateType,
    operand1: Label,
    operand2: Label,
}

#[derive(Debug, Clone)]
struct Wires {
    assertions: HashMap<Label, bool>,
    gates: HashMap<Label, Gate>,
    outputs: Vec<Label>,
    broken: bool
}

fn preprocess(input: &str) -> Wires {
    let mut assertions = HashMap::new();
    let mut gates = HashMap::new();
    let mut outputs = vec![];

    let mut input = input.lines();

    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }
        let line = line.chars().collect_vec();
        let name = Label([line[0], line[1], line[2]]);
        let value = line[5] == '1';
        assertions.insert(name, value);
    }

    let gate_regex = Regex::new(r"^(\w{3}) (AND|OR|XOR) (\w{3}) -> (\w{3})$").unwrap();

    while let Some(line) = input.next() {
        let capture = gate_regex.captures(line).unwrap();
        let operand1 = capture.get(1).unwrap().as_str().into();
        let operand2 = capture.get(3).unwrap().as_str().into();
        let output: Label = capture.get(4).unwrap().as_str().into();
        let gate_type = match capture.get(2).unwrap().as_str() {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => panic!("Unknown Gate Type"),
        };
        if output.0[0] == 'z' {
            outputs.push(output.clone());
        }
        gates.insert(output, Gate {
            gate_type,
            operand1,
            operand2,
        });
    }

    outputs.sort();
    Wires {
        assertions,
        gates,
        outputs,
        broken: false
    }
}

fn clear_assertions(wires: &mut Wires, label: Label, loop_prevention: &mut HashSet<Label>) {
    if loop_prevention.contains(&label) {
        return;
    }
    loop_prevention.insert(label.clone());
    if wires.assertions.remove(&label).is_none() {
        return;
    }
    for (output, gate) in wires.gates.clone() {
        if gate.operand1 == label || gate.operand2 == label {
            clear_assertions(wires, output.clone(), loop_prevention);
        }
    }
}

fn apply_swaps(wires: &mut Wires, swaps: &[(Label, Label)]) {
    let mut loop_prevention = HashSet::new();
    for (a, b) in swaps {
        clear_assertions(wires, a.clone(), &mut loop_prevention);
        clear_assertions(wires, b.clone(), &mut loop_prevention);
    }
    for (a, b) in swaps {
        let a_gate = wires.gates.get(a).unwrap().clone();
        let b_gate = wires.gates.get(b).unwrap().clone();
        wires.gates.insert(a.clone(), b_gate);
        wires.gates.insert(b.clone(), a_gate);
    }
}

fn get_wire(wires: &mut Wires, label: Label, loop_prevention: &mut HashSet<Label>) -> bool {
    if wires.broken {
        return false;
    }
    // If we are already asserting this wire
    if let Some(assertion) = wires.assertions.get(&label) {
        return *assertion;
    }

    if loop_prevention.contains(&label) {
        wires.broken = true;
        return false;
    }
    loop_prevention.insert(label.clone());

    // Otherwise we need to figure out what this wire is asserting
    let gate = wires.gates.get(&label).expect("Missing Gate").clone();

    // Slight efficiency gain by lazy evaluation - if the first branch of an AND gate
    // fails (or the first of an OR succeeds no need to check the second)
    let operand1 = |wires, loop_prevention| get_wire(wires, gate.operand1.clone(), loop_prevention);
    let operand2 = |wires, loop_prevention| get_wire(wires, gate.operand2.clone(), loop_prevention);

    let result = match gate.gate_type {
        GateType::And => operand1(wires, loop_prevention) && operand2(wires, loop_prevention),
        GateType::Or => operand1(wires, loop_prevention) || operand2(wires, loop_prevention),
        GateType::Xor => operand1(wires, loop_prevention) ^ operand2(wires, loop_prevention),
    };

    wires.assertions.insert(label, result);

    result
}

fn get_z_output(mut wires: Wires) -> u64 {
    let mut result = 0;
    let mut loop_prevention = HashSet::new();

    for output in wires.outputs.clone() {
        let place_value = 1u64.shl(output.0[1].to_digit(10).unwrap()*10 + output.0[2].to_digit(10).unwrap());
        if get_wire(&mut wires, output, &mut loop_prevention) {
            result |= place_value;
        }
    }

    if wires.broken {
        return u64::MAX;
    }

    result

}

fn get_decimal_z_output(input: &str) -> u64 {
    get_z_output(preprocess(input))
}

fn override_assertions(wires: &mut Wires, x: u64, y: u64) {
    for (label, _) in wires.assertions.clone() {
        let is_x = label.0[0] == 'x';
        let place_value = 1u64.shl(label.0[1].to_digit(10).unwrap()*10 + label.0[2].to_digit(10).unwrap());
        let value = if is_x { x } else { y };
        let value = (value & place_value) != 0;
        wires.assertions.insert(label, value);
    }
}

fn get_decimal_z_output_x_y(input: &str, x: u64, y: u64, swaps: &[(Label, Label)]) -> u64 {
    let mut wires = preprocess(input);
    apply_swaps(&mut wires, swaps);
    override_assertions(&mut wires, x, y);
    get_z_output(wires)
}

fn get_all_labels(input: &str) -> Vec<Label> {
    preprocess(input).gates.keys().cloned().collect_vec()
}

fn generate_all_four_swaps(potential_improvements: &[(usize, usize)]) -> Vec<[(usize, usize); 4]> {
    (0..potential_improvements.len()).combinations(4).filter_map(|items| {
        let mut result = HashSet::new();
        for i in 0..4 {
            result.insert(potential_improvements[items[i]].0);
            result.insert(potential_improvements[items[i]].1);
        }
        if result.len() == 8 {
            Some([
                (potential_improvements[items[0]].0, potential_improvements[items[0]].1),
                (potential_improvements[items[1]].0, potential_improvements[items[1]].1),
                (potential_improvements[items[2]].0, potential_improvements[items[2]].1),
                (potential_improvements[items[3]].0, potential_improvements[items[3]].1),
            ])
        } else {
            None
        }
    }).collect_vec()
}

#[test]
fn test_part1() {
    assert_eq!(
        4,
        get_decimal_z_output(r"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
        )
    );
    assert_eq!(
        2024,
        get_decimal_z_output(r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
        )
    );
}

#[test]
fn test_part2() {
}