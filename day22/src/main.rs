use std::{collections::HashMap, ops};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl ops::Add<Pos> for Pos {
    type Output = Self;

    fn add(self, _rhs: Pos) -> Pos {
        Pos::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl ops::AddAssign<Pos> for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug)]
struct Caret {
    dir: char,
    pos: Pos,
}

impl Caret {
    fn new(dir: char, pos: Pos) -> Self {
        Caret { dir, pos }
    }

    fn mv_to(&mut self, pos: Pos) {
        self.pos = pos
    }

    fn turn(&mut self, dir: char) {
        let states = vec!['^', '>', 'v', '<'];
        let i = states.iter().position(|c| *c == self.dir).unwrap() as i32;
        match dir {
            'L' => self.dir = states[(i - 1).rem_euclid(4) as usize],
            'R' => self.dir = states[(i + 1).rem_euclid(4) as usize],
            _ => panic!("unknown turn direction"),
        }
    }

    fn score(&self) -> isize {
        match self.dir {
            '>' => (1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + 0) as isize,
            'v' => (1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + 1) as isize,
            '<' => (1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + 2) as isize,
            '^' => (1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + 3) as isize,
            _ => panic!("unkwown orientation"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<Pos, char>,
    caret: Caret,
}

impl Grid {
    fn new(grid: HashMap<Pos, char>) -> Self {
        Grid {
            grid,
            caret: Caret::new('o', Pos::new(-1, -1)),
        }
    }

    fn col(&self, x: i32) -> Vec<Pos> {
        self.grid
            .keys()
            .filter_map(|pos| if pos.x == x { Some(*pos) } else { None })
            .sorted_unstable_by_key(|p| p.y)
            .collect_vec()
    }

    fn row(&self, y: i32) -> Vec<Pos> {
        self.grid
            .keys()
            .filter_map(|pos| if pos.y == y { Some(*pos) } else { None })
            .sorted_unstable_by_key(|p| p.x)
            .collect_vec()
    }

    fn next_position_flat(&self) -> Pos {
        let s = match self.caret.dir {
            '>' => Pos::new(1, 0),
            'v' => Pos::new(0, 1),
            '<' => Pos::new(-1, 0),
            '^' => Pos::new(0, -1),
            _ => panic!("unkwown orientation"),
        };

        let next = self.caret.pos + s;
        if self.grid.contains_key(&next) {
            return next;
        }

        //Ovwer the edge, wrap
        match self.caret.dir {
            '<' | '>' => {
                let t = self.row(self.caret.pos.y);
                if self.caret.dir == '>' {
                    *t.first().unwrap()
                } else {
                    *t.last().unwrap()
                }
            }
            '^' | 'v' => {
                let t = self.col(self.caret.pos.x);
                if self.caret.dir == 'v' {
                    *t.first().unwrap()
                } else {
                    *t.last().unwrap()
                }
            }
            _ => panic!(),
        }
    }

    fn next_position_cube(&self) -> Pos {
        Pos::new(0, 0)
    }

    fn execute(&mut self, inst_set: &Vec<(i32, Option<char>)>, on_cube: bool) {
        //self.print();
        inst_set.iter().for_each(|i| {
            let (steps, dir) = *i;

            //Move
            for _ in 0..steps {
                let next = if on_cube {
                    self.next_position_cube()
                } else {
                    self.next_position_flat()
                };
                let tile = self.grid.get(&next);
                match tile {
                    Some('.') => {
                        //empty tile
                        self.caret.mv_to(next);
                        continue;
                    }
                    Some('#') => {
                        //wall tile
                        break;
                    }
                    _ => panic!("invalid tile"),
                }
            }

            //Turn
            if let Some(d) = dir {
                self.caret.turn(d);
            }
        });
    }

    fn print(&self) {
        let max_y = self.grid.keys().max_by_key(|t| t.y).unwrap().y;
        let max_x = self.grid.keys().max_by_key(|t| t.x).unwrap().x;

        for y in 0..max_y {
            for x in 0..max_x {
                if self.caret.pos == Pos::new(x, y) {
                    print!("{}", self.caret.dir);
                    continue;
                }

                if let Some(v) = self.grid.get(&Pos::new(x, y)) {
                    print!("{}", *v);
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        println!("\n");
    }
}

fn parse_input(input: &str) -> (HashMap<Pos, char>, Vec<(i32, Option<char>)>) {
    let (grid_str, inst_str) = input.trim_end().split_once("\n\n").unwrap();

    let mut grid = HashMap::new();
    grid_str.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if !c.is_whitespace() {
                grid.insert(Pos::new(x as i32, y as i32), c);
            }
        });
    });

    let ins = inst_str
        .split_inclusive(&['L', 'R'][..])
        .map(|c| match ['L', 'R'].contains(&c.chars().last().unwrap()) {
            true => {
                let (l, r) = c.split_at(c.len() - 1);
                (l.parse::<i32>().unwrap(), Some(r.chars().next().unwrap()))
            }
            false => (c.parse::<i32>().unwrap(), None),
        })
        .collect();

    (grid, ins)
}

fn solve(grid: &HashMap<Pos, char>, ins_set: &Vec<(i32, Option<char>)>, on_cube: bool) -> isize {
    let mut grid = Grid::new(grid.clone());

    //compute the starting pos
    let start = *grid
        .row(0)
        .iter()
        .find(|p| grid.grid.get(p) == Some(&'.'))
        .unwrap();

    grid.caret = Caret::new('>', start);
    grid.execute(ins_set, on_cube);

    grid.caret.score()
}

fn main() {
    let (grid, ins) = parse_input(include_str!("input"));
    println!("Part1: {}", solve(&grid, &ins, false));
    //println!("Part2: {}", solve(&grid, &ins, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_part1() {
        let (grid, ins) = parse_input(INPUT);
        assert_eq!(solve(&grid, &ins, false), 6032);
    }

    #[test]
    fn test_part2() {
        let (grid, ins) = parse_input(INPUT);
        assert_eq!(solve(&grid, &ins, true), 5031);
    }
}
