//! --- Day 3: Spiral Memory ---
//!
//! You come across an experimental new kind of memory stored on an infinite two-dimensional grid.
//!
//! Each square on the grid is allocated in a spiral pattern starting at a location marked 1 and
//! then counting up while spiraling outward. For example, the first few squares are allocated like
//! this:
//!
//! 17  16  15  14  13
//! 18   5   4   3  12
//! 19   6   1   2  11
//! 20   7   8   9  10
//! 21  22  23---> ...
//!
//! While this is very space-efficient (no squares are skipped), requested data must be carried
//! back to square 1 (the location of the only access port for this memory system) by programs that
//! can only move up, down, left, or right. They always take the shortest path: the Manhattan
//! Distance between the location of the data and square 1.
//!
//! For example:
//!
//! Data from square 1 is carried 0 steps, since it's at the access port.
//! Data from square 12 is carried 3 steps, such as: down, left, left.
//! Data from square 23 is carried only 2 steps: up twice.
//! Data from square 1024 must be carried 31 steps.
//!
//! How many steps are required to carry the data from the square identified in your puzzle input
//! all the way to the access port?
//!
//! --- Part Two ---
//!
//! As a stress test on the system, the programs here clear the grid and then store the value 1 in
//! square 1. Then, in the same allocation order as shown above, they store the sum of the values
//! in all adjacent squares, including diagonals.
//!
//! So, the first few squares' values are chosen as follows:
//!
//! Square 1 starts with the value 1.
//! Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
//! Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
//! Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their
//! values, 4.
//! Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.
//! Once a square is written, its value does not change. Therefore, the first few squares would
//! receive the following values:
//!
//! 147  142  133  122   59
//! 304    5    4    2   57
//! 330   10    1    1   54
//! 351   11   23   25   26
//! 362  747  806--->   ...
//! What is the first value written that is larger than your puzzle input?

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn zero() -> Pos {
        Pos { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct Walker {
    pos: Pos,
    value: u64,
    /// top left, bottom right
    inner_bounds: (Pos, Pos),
    heading: Direction,
}

impl Walker {
    fn new() -> Walker {
        Walker {
            pos: Pos::zero(),
            value: 1,
            inner_bounds: (Pos::zero(), Pos::zero()),
            heading: Direction::East,
        }
    }
}


impl Iterator for Walker {
    type Item = (u64, Pos);

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.heading {

            Direction::East => {
                if self.inner_bounds.1.x < self.pos.x {
                    self.heading = Direction::North;
                    self.inner_bounds.1 = self.pos;
                    self.pos.y += 1;
                } else {
                    self.pos.x += 1;
                }

                self.value += 1;

                (self.value, self.pos)
            }

            Direction::North => {
                if self.inner_bounds.0.y < self.pos.y {
                    self.heading = Direction::West;
                    self.pos.x -= 1;
                } else {
                    self.pos.y += 1;
                }

                self.value += 1;

                (self.value, self.pos)
            }

            Direction::West => {
                if self.inner_bounds.0.x > self.pos.x {
                    self.heading = Direction::South;
                    self.inner_bounds.0 = self.pos;
                    self.pos.y -= 1;
                } else {
                    self.pos.x -= 1;
                }

                self.value += 1;

                (self.value, self.pos)
            }

            Direction::South => {
                if self.inner_bounds.1.y > self.pos.y {
                    self.heading = Direction::East;
                    self.pos.x += 1;
                } else {
                    self.pos.y -= 1;
                }

                self.value += 1;

                (self.value, self.pos)
            }
        };
        println!("{:?}", self);
        Some(item)
    }
}

pub fn walk(n: u64) -> u64 {
    if n < 2 {
        0
    } else {
        let (_, pos) = Walker::new().find(|v| v.0 == n).unwrap();
        println!("{} = {:?}", n, pos);
        (pos.x.abs() + pos.y.abs()) as u64
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_1() {
        assert_eq!(walk(1), 0);
    }

    #[test]
    fn walk_2() {
        assert_eq!(walk(2), 1);
    }

    #[test]
    fn walk_3() {
        assert_eq!(walk(3), 2);
    }

    #[test]
    fn walk_4() {
        assert_eq!(walk(4), 1);
    }

    #[test]
    fn walk_5() {
        assert_eq!(walk(5), 2);
    }

    #[test]
    fn walk_6() {
        assert_eq!(walk(6), 1);
    }

    #[test]
    fn walk_7() {
        assert_eq!(walk(7), 2);
    }

    #[test]
    fn walk_8() {
        assert_eq!(walk(8), 1);
    }

    #[test]
    fn walk_9() {
        assert_eq!(walk(9), 2);
    }

    #[test]
    fn walk_10() {
        assert_eq!(walk(10), 3);
    }

    #[test]
    fn walk_12() {
        assert_eq!(walk(12), 3);
    }

    #[test]
    fn walk_23() {
        assert_eq!(walk(23), 2);
    }

    #[test]
    fn walk_24() {
        assert_eq!(walk(24), 3);
    }

    #[test]
    fn walk_1024() {
        assert_eq!(walk(1024), 31);
    }

}
