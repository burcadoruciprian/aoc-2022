use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Blizzard {
    pos: Pos,
    dir: char,
}

impl std::ops::Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Blizzard {
    fn new(pos: Pos, dir: char) -> Self {
        Self { pos, dir }
    }

    fn mv(&mut self, top_bound: isize, right_bound: isize) {
        match self.dir {
            '>' => self.pos.x = (self.pos.x + 1 - 1).rem_euclid(right_bound - 2) + 1,
            '<' => self.pos.x = (self.pos.x - 1 - 1).rem_euclid(right_bound - 2) + 1,
            '^' => self.pos.y = (self.pos.y - 1 - 1).rem_euclid(top_bound - 2) + 1,
            'v' => self.pos.y = (self.pos.y + 1 - 1).rem_euclid(top_bound - 2) + 1,
            _ => panic!("unknown blizzard direction"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Blizzard>, isize, isize) {
    let top_pound = input.lines().count() as isize;
    let right_bound = input.lines().next().unwrap().len() as isize;

    (
        input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| {
                    ['>', '<', '^', 'v']
                        .contains(&c)
                        .then_some(Blizzard::new(Pos::new(x as isize, y as isize), c))
                })
            })
            .collect(),
        top_pound,
        right_bound,
    )
}

fn next_possible(
    crt: Pos,
    top_bound: isize,
    right_bound: isize,
    blizzards: &HashSet<Pos>,
    start: &Pos,
    end: &Pos,
) -> Vec<Pos> {
    vec![(1_isize, 0_isize), (0, 1), (0, 0), (-1, 0), (0, -1)]
        .iter()
        .filter_map(|(dx, dy)| {
            let n = crt + Pos::new(*dx, *dy);
            if ((1..right_bound - 1).contains(&n.x)
                && (1..top_bound - 1).contains(&n.y)
                && !blizzards.contains(&n))
                || n == *start
                || n == *end
            {
                Some(n)
            } else {
                None
            }
        })
        .collect()
}

fn run(blizzards: &mut Vec<Blizzard>, bounds: (isize, isize), start: Pos, end: Pos) -> usize {
    let (top_bound, right_bound) = bounds;
    let mut minute = 0_usize;

    let mut positions = HashSet::from([start]);
    loop {
        blizzards
            .iter_mut()
            .for_each(|b| b.mv(top_bound, right_bound));

        let blizzards_pos = blizzards.iter().map(|b| b.pos).collect::<HashSet<_>>();
        let mut next_pozitions: HashSet<Pos> = HashSet::new();
        positions.iter().for_each(|e| {
            next_pozitions.extend(
                next_possible(*e, top_bound, right_bound, &blizzards_pos, &start, &end).iter(),
            )
        });

        if next_pozitions.contains(&end) {
            return minute + 1;
        }

        positions = next_pozitions;
        minute += 1;

        if positions.is_empty() {
            return 0;
        }
    }
}

fn part1(blizzards: &Vec<Blizzard>, bounds: (isize, isize)) -> usize {
    let mut blizzards = blizzards.clone();
    let start = Pos::new(1, 0);
    let end = Pos::new(bounds.1 - 2, bounds.0 - 1);
    run(&mut blizzards, bounds, start, end)
}

fn part2(blizzards: &Vec<Blizzard>, bounds: (isize, isize)) -> usize {
    let mut blizzards = blizzards.clone();
    let start = Pos::new(1, 0);
    let end = Pos::new(bounds.1 - 2, bounds.0 - 1);
    run(&mut blizzards, bounds, start, end)
        + run(&mut blizzards, bounds, end, start)
        + run(&mut blizzards, bounds, start, end)
}

fn main() {
    let (blizzards, top, right) = parse_input(include_str!("input"));
    println!("Part1: {}", part1(&blizzards, (top, right)));
    println!("Part2: {}", part2(&blizzards, (top, right)));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part1() {
        let (blizzards, top, right) = parse_input(INPUT);
        assert_eq!(part1(&blizzards, (top, right)), 18);
    }

    #[test]
    fn test_part2() {
        let (blizzards, top, right) = parse_input(INPUT);
        assert_eq!(part2(&blizzards, (top, right)), 54);
    }
}
