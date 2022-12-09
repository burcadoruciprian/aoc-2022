use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn pos(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    fn advance(&mut self, direction: char) {
        match direction {
            'U' => {
                self.y += 1;
            }
            'D' => {
                self.y -= 1;
            }
            'L' => {
                self.x -= 1;
            }
            'R' => {
                self.x += 1;
            }
            _ => panic!("Invalid direction"),
        }
    }

    fn follow(&mut self, other: &Point) {
        //Check if head and tail touch or overlap
        if (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 {
            return;
        }
        self.x += isize::signum(other.x - self.x);
        self.y += isize::signum(other.y - self.y);
    }
}

fn parse_input(input: &str) -> Vec<(char, usize)> {
    input
        .trim()
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(d, c)| (d.chars().next().unwrap(), c.parse::<usize>().unwrap()))
        .collect()
}

fn run_instruction(
    knots: &mut Vec<Point>,
    trail: &mut HashSet<Point>,
    instruction: &(char, usize),
) {
    let (d, c) = *instruction;
    let knots_no = knots.len();

    for _ in 0..c {
        //Move the head
        knots.first_mut().unwrap().advance(d);
        //And the rest will follow
        for (i, _) in (0..knots_no).tuple_windows() {
            let mut iter = knots.iter_mut();
            let h = iter.nth(i).unwrap();
            let t = iter.next().unwrap();
            t.follow(h);
            if i == knots_no -2 {
                trail.insert(*t);
            }
        }
    }
}

fn solve(intructions: &Vec<(char, usize)>, knots_no: usize) -> usize {
    let mut trail = HashSet::new();
    let mut knots = vec![Point::new(0, 0); knots_no];
    trail.insert(*knots.first().unwrap());
    for i in intructions {
        run_instruction(&mut knots, &mut trail, i)
    }

    trail.len()
}

fn main() {
    let instructions = parse_input(include_str!("input"));
    println!("Part1: {}", solve(&instructions, 2));
    println!("Part2: {},", solve(&instructions, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        let instructions = parse_input(INPUT1);
        assert_eq!(solve(&instructions, 2), 13);
    }

    #[test]
    fn test_part2() {
        let instructions1 = parse_input(INPUT1);
        assert_eq!(solve(&instructions1, 10), 1);

        let instructions2 = parse_input(INPUT2);
        //assert_eq!(solve(&instructions2, 10), 36);
    }
}
