//! --- Day 7: Recursive Circus ---
//!
//! Wandering further through the circuits of the computer, you come upon a tower of programs that
//! have gotten themselves into a bit of trouble. A recursive algorithm has gotten out of hand, and
//! now they're balanced precariously in a large tower.
//!
//! One program at the bottom supports the entire tower. It's holding a large disc, and on the disc
//! are balanced several more sub-towers. At the bottom of these sub-towers, standing on the bottom
//! disc, are other programs, each holding their own disc, and so on. At the very tops of these
//! sub-sub-sub-...-towers, many programs stand simply keeping the disc below them balanced but with
//! no disc of their own.
//!
//! You offer to help, but first you need to understand the structure of these towers. You ask each
//! program to yell out their name, their weight, and (if they're holding a disc) the names of the
//! programs immediately above them balancing on that disc. You write this information down (your
//! puzzle input). Unfortunately, in their panic, they don't do this in an orderly fashion; by the
//! time you're done, you're not sure which program gave which information.
//!
//! For example, if your list is the following:
//!
//! pbga (66)
//! xhth (57)
//! ebii (61)
//! havc (66)
//! ktlj (57)
//! fwft (72) -> ktlj, cntj, xhth
//! qoyq (66)
//! padx (45) -> pbga, havc, qoyq
//! tknk (41) -> ugml, padx, fwft
//! jptl (61)
//! ugml (68) -> gyxo, ebii, jptl
//! gyxo (61)
//! cntj (57)
//!
//! ...then you would be able to recreate the structure of the towers that looks like this:
//!
//!              gyxo
//!              /
//!        ugml - ebii
//!       /      \
//!      |         jptl
//!      |
//!      |         pbga
//!      /        /
//! tknk --- padx - havc
//!      \        \
//!      |         qoyq
//!      |
//!      |         ktlj
//!       \      /
//!        fwft - cntj
//!              \
//!              xhth
//!
//!
//! In this example, tknk is at the bottom of the tower (the bottom program), and is holding up
//! ugml, padx, and fwft. Those programs are, in turn, holding up other programs; in this example,
//! none of those programs are holding up any other programs, and are all the tops of their own
//! towers. (The actual tower balancing in front of you is much larger.)
//!
//! Before you're ready to help them, you need to make sure your information is correct. What is
//! the name of the bottom program?
//!


#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    name: String,
    weight: u32,
}

impl Program {
    pub fn new(name: &str, weight: u32) -> Program {
        Program {
            name: name.to_owned(),
            weight,
        }
    }
}

pub fn find_parent(program_name: &str, nodes: &[(Program, Vec<String>)]) -> Option<Program> {
    // should this panic if the name isn't in the map at all? Seems like yes.
    let name = program_name.to_string();
    assert!(nodes.iter().find(|pair| pair.0.name == name).is_some(), "requested program not in tree");
    nodes.iter().find(|pair| pair.1.contains(&name)).map(|pair| pair.0.clone())
}

fn parse_line(line: &str) -> (Program, Vec<String>) {
    let splits: Vec<&str> = line.splitn(2, ")").collect();
    let left = splits.get(0).expect("left");

    let program = {
        let parts: Vec<&str> = left.splitn(2, " (").collect();
        let name = &parts[0].trim();

        // println!("{}: {:?}", name, parts);

        let weight = parts.get(1).expect("weight string").parse::<u32>().expect(
            "weight parse",
        );

        Program::new(name, weight)
    };

    let child_names: Vec<String> = {
        // println!("{:?}", splits);
        splits
            .get(1)
            .map(|right| {
                // barf. Revisit this later...
                right
                    .replace("->", "")
                    .trim()
                    .split(",")
                    .map(|s| s.trim().to_owned())
                    .filter(|s| s != "")
                    .collect()
            })
            .unwrap_or(vec![])
    };

    (program, child_names)
}

pub fn build_node_map(data: &str) -> Vec<(Program, Vec<String>)> {
    data.lines().map(parse_line).collect()
}

pub fn find_root(nodes: &[(Program, Vec<String>)]) -> Program {
    let mut node = nodes.first().cloned().unwrap().0;

    loop {
        if let Some(parent) = find_parent(&node.name, &nodes) {
            node = parent;
        } else { break; }
    }
    node
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &'static str = "C (3)
D (12) -> F
B (19) -> C, D
  F (1)
A (77) -> B   ";

    #[test]
    fn test_build_node_tree() {
        let expected: Vec<(Program, Vec<String>)> =
            vec![
                (Program::new("C", 3), vec![]),
                (Program::new("D", 12), vec![String::from("F")]),
                (
                    Program::new("B", 19),
                    vec![String::from("C"), String::from("D")]
                ),
                (Program::new("F", 1), vec![]),
                (Program::new("A", 77), vec![String::from("B")]),
            ];
        assert_eq!(build_node_map(&TEST_INPUT), expected)
    }

    #[test]
    fn test_find_parent() {
        let node_map = build_node_map(&TEST_INPUT);
        assert_eq!(find_parent("F", &node_map), Some(Program::new("D", 12)));
        assert_eq!(find_parent("B", &node_map), Some(Program::new("A", 77)));
        assert_eq!(find_parent("A", &node_map), None);
    }

    #[test]
    fn test_find_root() {
        let node_map = build_node_map(&TEST_INPUT);
        assert_eq!(find_root(&node_map), Program::new("A", 77));
    }

    #[test]
    #[should_panic]
    fn test_panics_if_program_not_in_tree() {
        let node_map = build_node_map(&TEST_INPUT);
        assert_eq!(find_parent("Z", &node_map), None);
    }

    #[test]
    #[should_panic]
    fn test_panics_if_program_not_in_tree2() {
        let node_map = build_node_map(&TEST_INPUT);
        assert_eq!(find_parent("F", &vec![]), None);
    }
}
