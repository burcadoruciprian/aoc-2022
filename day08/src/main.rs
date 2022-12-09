use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    let mut counter = 0_usize;
    for r in 1..grid[0].len() - 1 {
        for c in 1..grid.len() - 1 {
            let crt = grid[r][c];
            //Left
            if !(0..c).rev().any(|i| grid[r][i] >= crt) {
                counter += 1;
                continue;
            }
            //Right
            if !(c + 1..grid[0].len()).any(|i| grid[r][i] >= crt) {
                counter += 1;
                continue;
            }
            //Up
            if !(0..r).rev().any(|i| grid[i][c] >= crt) {
                counter += 1;
                continue;
            }
            //Down
            if !(r + 1..grid.len()).any(|i| grid[i][c] >= crt) {
                counter += 1;
                continue;
            }
        }
    }

    //Add the edges
    counter + 2 * (grid[0].len() + grid.len() - 2)
}

fn part2(grid: &Vec<Vec<u8>>) -> usize {
    let mut highest = 0_usize;
    for r in 1..grid[0].len() - 1 {
        for c in 1..grid.len() - 1 {
            let crt = grid[r][c];

            let left = (0..c)
                .rev()
                .fold_while(0, |acc, i| {
                    if grid[r][i] >= crt {
                        Done(acc + 1)
                    } else {
                        Continue(acc + 1)
                    }
                })
                .into_inner() as usize;

            let right = (c + 1..grid[0].len())
                .fold_while(0, |acc, i| {
                    if grid[r][i] >= crt {
                        Done(acc + 1)
                    } else {
                        Continue(acc + 1)
                    }
                })
                .into_inner() as usize;

            let up = (0..r)
                .rev()
                .fold_while(0, |acc, i| {
                    if grid[i][c] >= crt {
                        Done(acc + 1)
                    } else {
                        Continue(acc + 1)
                    }
                })
                .into_inner() as usize;

            let down = (r + 1..grid.len())
                .fold_while(0, |acc, i| {
                    if grid[i][c] >= crt {
                        Done(acc + 1)
                    } else {
                        Continue(acc + 1)
                    }
                })
                .into_inner() as usize;

            //println!("Score for [{}][{}] = {}", r, c, left * right * up * down);
            highest = std::cmp::max(highest, left * right * up * down);
        }
    }

    highest
}

fn main() {
    let grid = parse_input(include_str!("input"));
    println!("{:?}", part1(&grid));
    println!("{:?}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let grid = parse_input(INPUT);
        assert_eq!(part1(&grid), 21);
    }

    #[test]
    fn test_part2() {
        let grid = parse_input(INPUT);
        assert_eq!(part2(&grid), 8);
    }
}
