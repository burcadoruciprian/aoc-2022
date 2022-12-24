use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }

    fn neighbours(&self) -> Vec<Self> {
        vec![
            Pos::new(self.x - 1, self.y - 1),
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x - 1, self.y + 1),
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x, self.y + 1),
            Pos::new(self.x + 1, self.y - 1),
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x + 1, self.y + 1),
        ]
    }
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

impl std::ops::Add<Direction> for Pos {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        match other {
            Direction::North(p) => self + p,
            Direction::South(p) => self + p,
            Direction::East(p) => self + p,
            Direction::West(p) => self + p,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North(Pos),
    South(Pos),
    East(Pos),
    West(Pos),
}

fn parse_input(input: &str) -> HashSet<Pos> {
    let mut elfs = HashSet::new();
    input.trim().lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                elfs.insert(Pos::new(x as isize, y as isize));
            }
        })
    });
    elfs
}

fn can_move_to(direction: Direction, elf: &Pos, elfs: &HashSet<Pos>) -> bool {
    let nw: Pos = Pos::new(-1, -1);
    let n: Pos = Pos::new(0, -1);
    let ne: Pos = Pos::new(1, -1);
    let e: Pos = Pos::new(1, 0);
    let se: Pos = Pos::new(1, 1);
    let s: Pos = Pos::new(0, 1);
    let sw: Pos = Pos::new(-1, 1);
    let w: Pos = Pos::new(-1, 0);
    let dir_mapping: HashMap<Direction, Vec<Pos>> = HashMap::from([
        (Direction::North(Pos::new(0, -1)), vec![nw, n, ne]),
        (Direction::South(Pos::new(0, 1)), vec![sw, s, se]),
        (Direction::West(Pos::new(-1, 0)), vec![sw, w, nw]),
        (Direction::East(Pos::new(1, 0)), vec![se, e, ne]),
    ]);

    return dir_mapping
        .get(&direction)
        .unwrap()
        .iter()
        .all(|d| !elfs.contains(&(*elf + *d)));
}

fn run(elfs: &HashSet<Pos>, rounds: usize) -> (Option<isize>, Option<isize>) {
    let mut elfs = elfs.clone();

    let mut directions = vec![
        Direction::North(Pos::new(0, -1)),
        Direction::South(Pos::new(0, 1)),
        Direction::West(Pos::new(-1, 0)),
        Direction::East(Pos::new(1, 0)),
    ];

    for round in 0..rounds {
        let mut proposed_moves: HashMap<Pos, Vec<Pos>> = HashMap::new();
        let mut next_elfs = HashSet::new();
        elfs.iter().for_each(|elf| {
            if elf.neighbours().iter().all(|n| !elfs.contains(n)) {
                next_elfs.insert(*elf);
                return;
            } else {
                for d in directions.iter() {
                    if can_move_to(*d, elf, &elfs) {
                        proposed_moves
                            .entry(*elf + *d)
                            .and_modify(|v| v.push(*elf))
                            .or_insert(vec![*elf]);
                        return;
                    }
                }
            }
            next_elfs.insert(*elf);
        });

        
        if next_elfs.len() == elfs.len() {
            return (None, Some(round as isize + 1));
        }

        for (new, old) in proposed_moves.iter() {
            if old.len() == 1 {
                next_elfs.insert(*new);
            } else {
                for elf in old {
                    next_elfs.insert(*elf);
                }
            }
        }
        let last_dir = directions.remove(0);
        directions.push(last_dir);
        elfs = next_elfs;
    }
    let min_x = elfs.iter().map(|c| c.x).min().unwrap();
    let max_x = elfs.iter().map(|c| c.x).max().unwrap();
    let min_y = elfs.iter().map(|c| c.y).min().unwrap();
    let max_y = elfs.iter().map(|c| c.y).max().unwrap();

    (
        Some((max_x - min_x + 1) * (max_y - min_y + 1) - elfs.len() as isize),
        None,
    )
}

fn main() {
    let elfs = parse_input(include_str!("input"));
    println!("Part1: {}", run(&elfs, 10).0.unwrap());
    println!("Part2: {}", run(&elfs, usize::MAX).1.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_part1() {
        let elfs = parse_input(INPUT);
        assert_eq!(run(&elfs, 10).0, Some(110))
    }

    #[test]
    fn test_part2() {
        let elfs = parse_input(INPUT);
        assert_eq!(run(&elfs, usize::MAX).1, Some(20));
    }
}
