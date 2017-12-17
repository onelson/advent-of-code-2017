//! --- Day 8: I Heard You Like Registers ---
//! You receive a signal directly from the CPU. Because of your recent assistance with
//! jump instructions, it would like you to compute the result of a series of unusual
//! register instructions.
//!
//! Each instruction consists of several parts: the register to modify, whether to increase
//! or decrease that register's value, the amount by which to increase or decrease it, and a
//! condition. If the condition fails, skip the instruction without modifying the register.
//! The registers all start at 0. The instructions look like this:
//!
//! b inc 5 if a > 1
//! a inc 1 if b < 5
//! c dec -10 if a >= 1
//! c inc -20 if c == 10
//!
//! These instructions would be processed as follows:
//!
//! Because a starts at 0, it is not greater than 1, and so b is not modified.
//! a is increased by 1 (to 1) because b is less than 5 (it is 0).
//! c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
//! c is increased by -20 (to -10) because c is equal to 10.
//! After this process, the largest value in any register is 1.
//!
//! You might also encounter <= (less than or equal to) or != (not equal to). However, the
//! CPU doesn't have the bandwidth to tell you what all the registers are named, and leaves
//! that to you to determine.
//!
//! What is the largest value in any register after completing the instructions in your puzzle
//! input?
//!

use std::collections::BTreeMap;

type Registers = BTreeMap<String, i32>;

#[derive(Debug)]
pub enum Cmp {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(Debug)]
pub enum Op {
    Inc,
    Dec
}

#[derive(Debug)]
pub struct Guard {
    pub key: String,
    pub cmp: Cmp,
    pub value: i32
}

impl Guard {
    pub fn new(key: String, cmp: String, value: i32) -> Guard {
        Guard {
            key,
            cmp: match cmp.as_ref() {
                "==" => Cmp::Eq,
                "!=" => Cmp::Neq,
                ">" => Cmp::Gt,
                "<" => Cmp::Lt,
                ">=" => Cmp::Gte,
                "<=" => Cmp::Lte,
                unknown => panic!("unknown cmp: `{}`", unknown),
            },
            value,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub key: String,
    pub op: Op,
    pub value: i32,
    pub guard: Guard
}

impl Instruction {
    pub fn from_string(instruction: &str) -> Instruction {
        // ex: "b inc 5 if a > 1"
        let parts: Vec<String> = instruction.split_whitespace().map(|x| x.to_owned()).collect();

        let key = parts[0].to_owned();
        let op = if &parts[1] == "inc" { Op::Inc } else { Op::Dec };
        let value: i32 = parts[2].parse().expect("instruction value");
        let guard = Guard::new(parts[4].to_owned(), parts[5].to_owned(), parts[6].parse().expect("guard value"));

        Instruction {
            key,
            op,
            value,
            guard,
        }
    }
}

fn check_guard(guard: &Guard, registers: &Registers) -> bool {
    let value = registers.get(&guard.key).unwrap_or(&0);
    match guard.cmp {
        Cmp::Eq => *value == guard.value,
        Cmp::Neq => *value != guard.value,
        Cmp::Lt => *value < guard.value,
        Cmp::Lte => *value <= guard.value,
        Cmp::Gt => *value > guard.value,
        Cmp::Gte => *value >= guard.value,
    }
}

fn update(instruction: &Instruction, registers: &mut Registers) {
    *registers.entry(instruction.key.clone()).or_insert(0) += match instruction.op {
        Op::Inc => instruction.value,
        Op::Dec => -instruction.value,
    };
}

pub fn execute(instructions: Vec<Instruction>) -> i32 {
    let mut registers = Registers::new();
//    println!();
    for instruction in &instructions {
//        println!("{:?}", instruction);
        if check_guard(&instruction.guard, &mut registers) {
            update(instruction, &mut registers);
        }
    }
//    println!("{:?}", registers);
    registers.values().max().expect("max value").to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn test_example() {
        let instructions = EXAMPLE.lines().map(|line| Instruction::from_string(line)).collect();
        assert_eq!(execute(instructions), 1);
    }
}
