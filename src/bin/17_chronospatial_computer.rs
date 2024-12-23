/*
--- Day 17: Chronospatial Computer ---

The Historians push the button on their strange device, but this time, you
all just feel like you're falling.

"Situation critical", the device announces in a familiar voice.
"Bootstrapping process failed. Initializing debugger...."

The small handheld device suddenly unfolds into an entire computer! The
Historians look around nervously before one of them tosses it to you.

This seems to be a 3-bit computer: its program is a list of 3-bit numbers
(0 through 7), like 0,1,2,3. The computer also has three registers named A,
B, and C, but these registers aren't limited to 3 bits and can instead hold
any integer.

The computer knows eight instructions, each identified by a 3-bit number
(called the instruction's opcode). Each instruction also reads the 3-bit
number after it as an input; this is called its operand.

A number called the instruction pointer identifies the position in the
program from which the next opcode will be read; it starts at 0, pointing
at the first 3-bit number in the program. Except for jump instructions, the
instruction pointer increases by 2 after each instruction is processed (to
move past the instruction's opcode and its operand). If the computer tries
to read an opcode past the end of the program, it instead halts.

So, the program 0,1,2,3 would run the instruction whose opcode is 0 and
pass it the operand 1, then run the instruction having opcode 2 and pass it
the operand 3, then halt.

There are two types of operands; each instruction specifies the type of its
operand. The value of a literal operand is the operand itself. For example,
the value of the literal operand 7 is the number 7. The value of a combo
operand can be found as follows:

- Combo operands 0 through 3 represent literal values 0 through 3.
- Combo operand 4 represents the value of register A.
- Combo operand 5 represents the value of register B.
- Combo operand 6 represents the value of register C.
- Combo operand 7 is reserved and will not appear in valid programs.

The eight instructions are as follows:

The adv instruction (opcode 0) performs division. The numerator is the
value in the A register. The denominator is found by raising 2 to the power
of the instruction's combo operand. (So, an operand of 2 would divide A by
4 (2^2); an operand of 5 would divide A by 2^B.) The result of the
division operation is truncated to an integer and then written to the A
register.

The bxl instruction (opcode 1) calculates the bitwise XOR of register B and
the instruction's literal operand, then stores the result in register B.

The bst instruction (opcode 2) calculates the value of its combo operand
modulo 8 (thereby keeping only its lowest 3 bits), then writes that value
to the B register.

The jnz instruction (opcode 3) does nothing if the A register is 0.
However, if the A register is not zero, it jumps by setting the instruction
pointer to the value of its literal operand; if this instruction jumps, the
instruction pointer is not increased by 2 after this instruction.

The bxc instruction (opcode 4) calculates the bitwise XOR of register B and
register C, then stores the result in register B. (For legacy reasons, this
instruction reads an operand but ignores it.)

The out instruction (opcode 5) calculates the value of its combo operand
modulo 8, then outputs that value. (If a program outputs multiple values,
they are separated by commas.)

The bdv instruction (opcode 6) works exactly like the adv instruction
except that the result is stored in the B register. (The numerator is
still read from the A register.)

The cdv instruction (opcode 7) works exactly like the adv instruction
except that the result is stored in the C register. (The numerator is still
read from the A register.)

Here are some examples of instruction operation:

- If register C contains 9, the program 2,6 would set register B to 1.
- If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
- If register A contains 2024, the program 0,1,5,4,3,0 would output
4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
- If register B contains 29, the program 1,7 would set register B to 26.
- If register B contains 2024 and register C contains 43690, the program
4,0 would set register B to 44354.

The Historians' strange device has finished initializing its debugger and
is displaying some information about the program it is trying to run (your
puzzle input). For example:

Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0

Your first task is to determine what the program is trying to output. To do
this, initialize the registers to the given values, then run the given
program, collecting any output produced by out instructions. (Always join
the values produced by out instructions with commas.) After the above
program halts, its final output will be 4,6,3,5,6,3,5,2,1,0.

Using the information provided by the debugger, initialize the registers to
the given values, then run the program. Once it halts, what do you get if
you use commas to join the values it output into a single string?

--- Part Two ---

Digging deeper in the device's manual, you discover the problem: this
program is supposed to output another copy of the program! Unfortunately,
the value in register A seems to have been corrupted. You'll need to find a
new value to which you can initialize register A so that the program's
output instructions produce an exact copy of the program itself.

For example:

Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0

This program outputs a copy of itself if register A is instead initialized
to 117440. (The original initial value of register A, 2024, is ignored.)

What is the lowest positive initial value for register A that causes the
program to output a copy of itself?
 */
use itertools::Itertools;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{BitXor, Shr};

fn main() {
    let input_file = fs::read_to_string("./inputs/17_chronospatial_computer.txt").unwrap();
    println!("{}", get_output_string(&input_file));
    println!("{}", get_reg_a_value(&input_file));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<Number> for Opcode {
    fn from(value: Number) -> Self {
        match value.0 {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!("All values considered"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Number(u8);

impl Number {
    fn new(value: u8) -> Self {
        Self(value % 8)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::new(value.rem_euclid(8) as u8)
    }
}

impl Into<i64> for Number {
    fn into(self) -> i64 {
        (self.0 % 8) as i64
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
         write!(f, "{}", self.0)
    }
}

struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
}

impl Computer {
    fn combo_operand(&self, operand: Number) -> i64 {
        match operand.0 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => unreachable!("Reserved and will not appear in valid programs"),
            _ => unreachable!("All values considered"),
        }
    }
}

fn preprocess(input: &str) -> (Computer, Vec<Number>) {
    let mut lines = input.lines();

    let reg_a = (&lines.next().unwrap()[12..]).parse::<i64>().unwrap();
    let reg_b = (&lines.next().unwrap()[12..]).parse::<i64>().unwrap();
    let reg_c = (&lines.next().unwrap()[12..]).parse::<i64>().unwrap();
    let _ = lines.next();
    let program = (&lines.next().unwrap()[9..])
        .split(',')
        .map(|num| Number::new(num.parse::<u8>().unwrap()))
        .collect_vec();

    (
        Computer {
            reg_a,
            reg_b,
            reg_c,
        },
        program
    )
}

fn run_program(computer: &mut Computer, program: &Vec<Number>) -> Vec<Number> {
    let mut instruction_pointer = 0usize;
    let mut output = vec![];

    while instruction_pointer < program.len() {
        let opcode: Opcode = program[instruction_pointer].into();
        assert!((instruction_pointer + 1) < program.len(), "Operand is always in-bounds");
        let operand: Number = program[instruction_pointer + 1];

        instruction_pointer += 2;

        match opcode {
            Opcode::Adv => computer.reg_a = computer.reg_a.shr(computer.combo_operand(operand)),
            Opcode::Bxl => computer.reg_b = computer.reg_b.bitxor(&operand.into()),
            Opcode::Bst => computer.reg_b = computer.combo_operand(operand) % 8,
            Opcode::Jnz => {
                if computer.reg_a != 0 {
                    instruction_pointer = <Number as Into<i64>>::into(operand) as usize
                }
            }
            Opcode::Bxc => computer.reg_b = computer.reg_b.bitxor(computer.reg_c),
            Opcode::Out => output.push(computer.combo_operand(operand).into()),
            Opcode::Bdv => computer.reg_b = computer.reg_a.shr(computer.combo_operand(operand)),
            Opcode::Cdv => computer.reg_c = computer.reg_a.shr(computer.combo_operand(operand)),
        }
    }

    output
}

fn get_output_string(input: &str) -> String {
    let (mut computer, program) = preprocess(input);
    let output = run_program(&mut computer, &program);
    output.into_iter().join(",")
}

fn find_reg_a(program: &Vec<Number>, digit: usize, reg_a: i64, reg_b: i64, reg_c: i64) -> i64 {
    let min = 1i64 << (digit * 3);
    let max = 1i64 << ((digit + 1) * 3);
    let min_reg_a = 1i64 << ((program.len() - 1) * 3);

    for i in (0..max).step_by(min as usize) {
        let reg_a = i + reg_a;
        if reg_a < min_reg_a {
            continue;
        }
        let mut computer = Computer {
            reg_a,
            reg_b,
            reg_c,
        };

        let output = run_program(&mut computer, program);
        if output[digit] == program[digit] {
            if digit == 0 {
                return reg_a;
            }
            let reg_a = find_reg_a(program, digit - 1, reg_a, reg_b, reg_c);
            // If we got a result
            if reg_a > 0 {
                // Make sure it is correct just in case
                let mut computer = Computer {
                    reg_a,
                    reg_b,
                    reg_c,
                };
                let output = run_program(&mut computer, program);
                if output[digit] == program[digit] {
                    return reg_a;
                }
            }
        }
    }

    -1
}

fn get_reg_a_value(input: &str) -> i64 {
    let (computer, program) = preprocess(input);

    // For both the test input and real input it seems the number of digits in the octal representation
    // of register a is the same as the number of values in the output

    // Additionally each output value seems to change only when that digit changes
    // when indexing output left to right and input right to left

    // So if I want to change the right most output of the program I need to step forward by 1 on
    // the left most digit of the input
    find_reg_a(&program, program.len() - 1, 0, computer.reg_b, computer.reg_c)
}

#[test]
fn test_part1() {
    assert_eq!("4,6,3,5,6,3,5,2,1,0",
               get_output_string(
                   r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
               )
    );
}

#[test]
fn test_part2() {
    assert_eq!("0,3,5,4,3,0",
               get_output_string(
                   r"Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
               )
    );
    assert_eq!(117440,
               get_reg_a_value(
                   r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
               )
    );
}