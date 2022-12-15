use std::collections::HashMap;

use itertools::Itertools;

fn parse_input(input: &str) -> (HashMap<(usize, usize), char>, usize) {
    let mut grid = HashMap::new();
    input.trim().lines().for_each(|l| {
        l.split(" -> ")
            .map(|c| {
                c.split(',')
                    .filter_map(|v| v.parse::<isize>().ok())
                    .collect_tuple::<(isize, isize)>()
                    .unwrap()
            })
            .tuple_windows()
            .for_each(|(start, end)| {
                let mut crt = start;
                grid.insert((crt.0 as usize, crt.1 as usize), '#');

                while crt.0 != end.0 || crt.1 != end.1 {
                    crt.0 += (end.0 as isize - crt.0 as isize).signum();
                    crt.1 += (end.1 as isize - crt.1 as isize).signum();
                    grid.insert((crt.0 as usize, crt.1 as usize), '#');
                }
            });
    });

    let bottom = grid
        .iter()
        .filter(|(_, v)| **v == '#')
        .max_by(|x, y| x.0 .1.cmp(&y.0 .1))
        .unwrap()
        .0
         .1;

    (grid, bottom)
}

fn part1(grid: &HashMap<(usize, usize), char>, bottom: &usize) -> usize {
    let mut grid = grid.clone();
    'outer: loop {
        // new sand
        let (mut sx, mut sy) = (500_usize, 0_usize);

        loop {
            if sy + 1 > *bottom {
                break 'outer;
            }
            //straight down
            if !grid.contains_key(&(sx, sy + 1)) {
                sy += 1;
                continue;
            }

            //down left
            if !grid.contains_key(&(sx - 1, sy + 1)) {
                sx -= 1;
                sy += 1;
                continue;
            }

            //down right
            if !grid.contains_key(&(sx + 1, sy + 1)) {
                sx += 1;
                sy += 1;
                continue;
            }

            grid.insert((sx, sy), 'o');
            break;
        }
    }

    grid.values().filter(|v| **v == 'o').count()
}

fn part2(grid: &HashMap<(usize, usize), char>, bottom: &usize) -> usize {
    let mut grid = grid.clone();

    (0..1000_usize).for_each(|x| {
        grid.insert((x, (bottom + 2)), '#');
    });

    loop {
        // new sand
        let (mut sx, mut sy) = (500_usize, 0_usize);
        if grid.get(&(sx, sy)) == Some(&'o') {
            break;
        }

        loop {
            //straight down
            if !grid.contains_key(&(sx, sy + 1)) {
                sy += 1;
                continue;
            }

            //down left
            if !grid.contains_key(&(sx - 1, sy + 1)) {
                sx -= 1;
                sy += 1;
                continue;
            }

            //down right
            if !grid.contains_key(&(sx + 1, sy + 1)) {
                sx += 1;
                sy += 1;
                continue;
            }

            grid.insert((sx, sy), 'o');
            break;
        }
    }

    grid.values().filter(|v| **v == 'o').count()
}

fn main() {
    let (grid, bottom) = parse_input(include_str!("input"));
    println!("Part1: {}", part1(&grid, &bottom));
    println!("Part12 {}", part2(&grid, &bottom));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        let (grid, bottom) = parse_input(INPUT);
        assert_eq!(part1(&grid, &bottom), 24);
    }

    #[test]
    fn test_part2() {
        let (grid, bottom) = parse_input(INPUT);
        assert_eq!(part2(&grid, &bottom), 93);
    }
}
