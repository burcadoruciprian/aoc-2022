use std::{cmp, collections::{HashSet, HashMap}};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Block {
    x: usize,
    y: usize,
}

impl Block {
    fn new(x: usize, y: usize) -> Self {
        Block { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait Shape {
    fn most(&self, d: Direction) -> usize;
    fn edge(&self, d: Direction) -> Vec<Block>;
    fn shift(&mut self, d: Direction);
    fn blocks(&self) -> Vec<Block>;
}

struct MinusShape {
    blocks: Vec<Block>,
}

impl MinusShape {
    fn new(h: usize) -> Self {
        MinusShape {
            blocks: vec![
                Block::new(2, h),
                Block::new(3, h),
                Block::new(4, h),
                Block::new(5, h),
            ],
        }
    }
}

impl Shape for MinusShape {
    fn most(&self, d: Direction) -> usize {
        match d {
            Direction::Up => self.blocks[0].y,
            Direction::Down => self.blocks[0].y,
            Direction::Left => self.blocks[0].x,
            Direction::Right => self.blocks[3].x,
        }
    }

    fn edge(&self, d: Direction) -> Vec<Block> {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => self.blocks.clone(),
            Direction::Left => vec![self.blocks[0]],
            Direction::Right => vec![self.blocks[3]],
        }
    }

    fn shift(&mut self, d: Direction) {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => self.blocks.iter_mut().for_each(|b| b.y -= 1),
            Direction::Left => self.blocks.iter_mut().for_each(|b| b.x -= 1),
            Direction::Right => self.blocks.iter_mut().for_each(|b| b.x += 1),
        }
    }

    fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }
}

struct BarShape {
    blocks: Vec<Block>,
}

impl BarShape {
    fn new(h: usize) -> Self {
        BarShape {
            blocks: vec![
                Block::new(2, h),
                Block::new(2, h + 1),
                Block::new(2, h + 2),
                Block::new(2, h + 3),
            ],
        }
    }
}

impl Shape for BarShape {
    fn most(&self, d: Direction) -> usize {
        match d {
            Direction::Up => self.blocks[3].y,
            Direction::Down => self.blocks[0].y,
            Direction::Left => self.blocks[0].x,
            Direction::Right => self.blocks[0].x,
        }
    }

    fn edge(&self, d: Direction) -> Vec<Block> {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => vec![self.blocks[0]],
            Direction::Left => self.blocks.clone(),
            Direction::Right => self.blocks.clone(),
        }
    }

    fn shift(&mut self, d: Direction) {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => self.blocks.iter_mut().for_each(|b| b.y -= 1),
            Direction::Left => self.blocks.iter_mut().for_each(|b| b.x -= 1),
            Direction::Right => self.blocks.iter_mut().for_each(|b| b.x += 1),
        }
    }

    fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }
}

struct PlusShape {
    blocks: Vec<Block>,
}

impl PlusShape {
    fn new(h: usize) -> Self {
        PlusShape {
            blocks: vec![
                Block::new(3, h),
                Block::new(3, h + 1),
                Block::new(3, h + 2),
                Block::new(2, h + 1),
                Block::new(4, h + 1),
            ],
        }
    }
}

impl Shape for PlusShape {
    fn most(&self, d: Direction) -> usize {
        match d {
            Direction::Up => self.blocks[2].y,
            Direction::Down => self.blocks[0].y,
            Direction::Left => self.blocks[3].x,
            Direction::Right => self.blocks[4].x,
        }
    }

    fn edge(&self, d: Direction) -> Vec<Block> {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => vec![self.blocks[3], self.blocks[0], self.blocks[4]],
            Direction::Left => vec![self.blocks[0], self.blocks[3], self.blocks[2]],
            Direction::Right => vec![self.blocks[0], self.blocks[4], self.blocks[2]],
        }
    }

    fn shift(&mut self, d: Direction) {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => self.blocks.iter_mut().for_each(|b| b.y -= 1),
            Direction::Left => self.blocks.iter_mut().for_each(|b| b.x -= 1),
            Direction::Right => self.blocks.iter_mut().for_each(|b| b.x += 1),
        }
    }

    fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }
}

struct LShape {
    blocks: Vec<Block>,
}

impl LShape {
    fn new(h: usize) -> Self {
        LShape {
            blocks: vec![
                Block::new(2, h),
                Block::new(3, h),
                Block::new(4, h),
                Block::new(4, h + 1),
                Block::new(4, h + 2),
            ],
        }
    }
}

impl Shape for LShape {
    fn most(&self, d: Direction) -> usize {
        match d {
            Direction::Up => self.blocks[4].y,
            Direction::Down => self.blocks[0].y,
            Direction::Left => self.blocks[0].x,
            Direction::Right => self.blocks[4].x,
        }
    }

    fn edge(&self, d: Direction) -> Vec<Block> {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => vec![self.blocks[0], self.blocks[1], self.blocks[2]],
            Direction::Left => vec![self.blocks[0], self.blocks[3], self.blocks[4]],
            Direction::Right => vec![self.blocks[2], self.blocks[3], self.blocks[4]],
        }
    }

    fn shift(&mut self, d: Direction) {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => self.blocks.iter_mut().for_each(|b| b.y -= 1),
            Direction::Left => self.blocks.iter_mut().for_each(|b| b.x -= 1),
            Direction::Right => self.blocks.iter_mut().for_each(|b| b.x += 1),
        }
    }

    fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }
}

struct SquareShape {
    blocks: Vec<Block>,
}

impl SquareShape {
    fn new(h: usize) -> Self {
        SquareShape {
            blocks: vec![
                Block::new(2, h),
                Block::new(3, h),
                Block::new(2, h + 1),
                Block::new(3, h + 1),
            ],
        }
    }
}

impl Shape for SquareShape {
    fn most(&self, d: Direction) -> usize {
        match d {
            Direction::Up => self.blocks[2].y,
            Direction::Down => self.blocks[0].y,
            Direction::Left => self.blocks[0].x,
            Direction::Right => self.blocks[1].x,
        }
    }

    fn edge(&self, d: Direction) -> Vec<Block> {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => vec![self.blocks[0], self.blocks[1]],
            Direction::Left => vec![self.blocks[0], self.blocks[2]],
            Direction::Right => vec![self.blocks[1], self.blocks[3]],
        }
    }

    fn shift(&mut self, d: Direction) {
        match d {
            Direction::Up => unimplemented!(),
            Direction::Down => self.blocks.iter_mut().for_each(|b| b.y -= 1),
            Direction::Left => self.blocks.iter_mut().for_each(|b| b.x -= 1),
            Direction::Right => self.blocks.iter_mut().for_each(|b| b.x += 1),
        }
    }

    fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }
}

struct Grid {
    max_h: usize,
    blocks: HashSet<Block>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            max_h: 0,
            blocks: HashSet::new(),
        }
    }

    fn new_shape(&self, turn: usize) -> Box<dyn Shape> {
        match turn % 5 {
            1 => Box::new(MinusShape::new(self.max_h + 4)),
            2 => Box::new(PlusShape::new(self.max_h + 4)),
            3 => Box::new(LShape::new(self.max_h + 4)),
            4 => Box::new(BarShape::new(self.max_h + 4)),
            0 => Box::new(SquareShape::new(self.max_h + 4)),
            _ => panic!(),
        }
    }

    fn print(&self, shape: &Box<dyn Shape>) {
        for y in (1..=shape.most(Direction::Up)).rev() {
            print!("|");
            for x in 0..7_usize {
                if shape.blocks().contains(&Block::new(x, y)) {
                    print!("@");
                    continue;
                }

                if self.blocks.contains(&Block::new(x, y)) {
                    print!("#");
                    continue;
                }
                print!(".");
            }
            println!("|");
        }
        println!("+-------+\n\n");
    }

    fn shift(&self, d: Direction, shape: &mut dyn Shape) -> bool {
        let most = shape.most(d);
        let edge = shape.edge(d);
        match d {
            Direction::Up => (),
            Direction::Down => {
                if most == 1_usize
                    || edge
                        .iter()
                        .map(|b| Block::new(b.x, b.y - 1))
                        .any(|b| self.blocks.contains(&b))
                {
                    return false;
                }
            }
            Direction::Left => {
                if most == 0_usize
                    || edge
                        .iter()
                        .map(|b| Block::new(b.x - 1, b.y))
                        .any(|b| self.blocks.contains(&b))
                {
                    return false;
                }
            }
            Direction::Right => {
                if most == 6_usize
                    || edge
                        .iter()
                        .map(|b| Block::new(b.x + 1, b.y))
                        .any(|b| self.blocks.contains(&b))
                {
                    return false;
                }
            }
        }
        shape.shift(d);
        true
    }

    fn run(&mut self, turns: usize, inst: &str) -> usize {
        let instructions = inst.chars().collect::<Vec<char>>();
        let mut turn = 1_usize;
        let mut crt_ins_index = 0_usize;
        let mut cycles: HashMap<usize, Vec<usize>> = HashMap::new();
        while turn <= turns {

            if (self.is_new_floor(&self.max_h))
            {
                println!("New floor: h: {} turn: {}, Ins {}", self.max_h, turn, crt_ins_index);
            }



            let mut shape = self.new_shape(turn);
            let mut settled = false;
            while !settled {
                
                let crt_inst = instructions[crt_ins_index % inst.len()];
                match crt_inst {
                    '>' => self.shift(Direction::Right, shape.as_mut()),
                    '<' => self.shift(Direction::Left, shape.as_mut()),
                    _ => panic!("unknown direction"),
                };
                crt_ins_index += 1;

                settled = !self.shift(Direction::Down, shape.as_mut());
                if settled {
                    self.blocks.extend(shape.blocks());
                    self.max_h = cmp::max(self.max_h, shape.most(Direction::Up));
                }
            }
            turn += 1;
        }

        self.max_h
    }

    fn is_new_floor(&self, h: &usize) -> bool {
        match *h {
            0 => true,
            _ => (0..7).all(|x| self.blocks.contains(&Block::new(x, *h))),
        }
    }
}

fn main() {
    let insttructions = include_str!("input").trim();
    let mut grid = Grid::new();
    println!("Part1: {}", grid.run(2022, insttructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        let mut grid = Grid::new();
        assert_eq!(grid.run(2022, INPUT), 3068);
    }

    #[test]
    fn test_part2() {
        let mut grid = Grid::new();
        assert_eq!(grid.run(1000000000000, INPUT), 1514285714288);
    }
}
