//! --- Day 5: A Maze of Twisty Trampolines, All Alike ---
//!
//! An urgent interrupt arrives from the CPU: it's trapped in a maze of jump instructions, and it
//! would like assistance from any programs with spare cycles to help find the exit.
//!
//! The message includes a list of the offsets for each jump. Jumps are relative: -1 moves to the
//! previous instruction, and 2 skips the next one. Start at the first instruction in the list.
//! The goal is to follow the jumps until one leads outside the list.
//!
//! In addition, these instructions are a little strange; after each jump, the offset of that
//! instruction increases by 1. So, if you come across an offset of 3, you would move three
//! instructions forward, but change it to a 4 for the next time it is encountered.
//!
//! For example, consider the following list of jump offsets:
//!
//! 0
//! 3
//! 0
//! 1
//! -3
//!
//! Positive jumps ("forward") move downward; negative jumps move upward. For legibility in this
//! example, these offset values will be written all on one line, with the current instruction
//! marked in parentheses. The following steps would be taken before an exit is found:
//!
//! (0) 3  0  1  -3  - before we have taken any steps.
//! (1) 3  0  1  -3  - jump with offset 0 (that is, don't jump at all). Fortunately, the
//!                    instruction is then incremented to 1.
//! 2 (3) 0  1  -3   - step forward because of the instruction we just modified. The first
//!                    instruction is incremented again, now to 2.
//! 2  4  0  1 (-3)  - jump all the way to the end; leave a 4 behind.
//! 2 (4) 0  1  -2   - go back to where we just were; increment -3 to -2.
//! 2  5  0  1  -2   - jump 4 steps forward, escaping the maze.
//! In this example, the exit is reached in 5 steps.
//!
//! How many steps does it take to reach the exit?
//!

pub fn parse_instructions(data: &str) -> Vec<i64> {
    data.lines().map(|x| x.parse::<i64>().unwrap()).collect()
}


/// updates the value of the cell at `idx` returns the updated position
fn jump(idx: usize, xs: &mut [i64]) -> Option<usize> {
    // calculate where we will land first
    let dest = (xs[idx] + idx as i64) as usize;
    xs[idx] += 1;
    // check to see if the dest is out of bounds or not
    if dest < xs.len() { Some(dest) } else { None }
}

pub fn execute(instructions: &mut [i64]) -> u64 {
    let mut destination = jump(0, instructions);

    // only increment the moves if we have somewhere to go?
    let mut moves = if destination.is_some() { 1 } else { 0 };

    while let Some(idx) = destination {
        moves += 1;
        destination = jump(idx, instructions);
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_by_jump() {
        let mut instructions = vec![0, 3, 0, 1, -3];
        let idx = Some(0);
        let idx = jump(idx.unwrap(), &mut instructions);
        assert_eq!(idx, Some(0));
        assert_eq!(instructions, vec![1, 3, 0, 1, -3]);

        let idx = jump(idx.unwrap(), &mut instructions);
        assert_eq!(idx, Some(1));
        assert_eq!(instructions, vec![2, 3, 0, 1, -3]);

        let idx = jump(idx.unwrap(), &mut instructions);
        assert_eq!(idx, Some(4));
        assert_eq!(instructions, vec![2, 4, 0, 1, -3]);


        let idx = jump(idx.unwrap(), &mut instructions);
        assert_eq!(idx, Some(1));
        assert_eq!(instructions, vec![2, 4, 0, 1, -2]);


        let idx = jump(idx.unwrap(), &mut instructions);
        assert_eq!(idx, None);
        assert_eq!(instructions, vec![2, 5, 0, 1, -2]);
    }

    #[test]
    fn test_example_by_execute() {
        let mut instructions = vec![0, 3, 0, 1, -3];
        assert_eq!(execute(&mut instructions), 5);
    }

}
