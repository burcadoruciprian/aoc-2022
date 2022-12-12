use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> (HashMap<(usize, usize), u8>, (usize, usize), (usize, usize)) {
    let mut grid = HashMap::new();
    let mut start = (0_usize, 0_usize);
    let mut end = (0_usize, 0_usize);
    input.trim().lines().enumerate().for_each(|(y, l)| {
        l.as_bytes().iter().enumerate().for_each(|(x, c)| match c {
            b'S' => {
                start = (x, y);
                grid.insert((x, y), b'a');
            }
            b'E' => {
                end = (x, y);
                grid.insert((x, y), b'z');
            }
            _ => {
                grid.insert((x, y), *c);
            }
        });
    });

    (grid, start, end)
}

fn find_route(
    grid: &HashMap<(usize, usize), u8>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let neighbours =
        |grid: &HashMap<(usize, usize), u8>, crt: (usize, usize)| -> Vec<(usize, usize)> {
            [(0, -1), (-1, 0), (0, 1), (1, 0)]
                .iter()
                .filter_map(|(dx, dy)| {
                    let (nx, ny) = (crt.0 as isize + dx, crt.1 as isize + dy);
                    match nx >= 0 && ny >= 0 && grid.contains_key(&(nx as usize, ny as usize)) {
                        true => Some((nx as usize, ny as usize)),
                        false => None,
                    }
                })
                .collect()
        };

    let mut visited = HashSet::new();
    let mut to_process = VecDeque::new();

    to_process.push_back((start, 0));

    while let Some(((x, y), len)) = to_process.pop_front() {
        if (x, y) == end {
            return Some(len);
        }

        for (nx, ny) in neighbours(grid, (x, y)) {
            let elevation = grid[&(nx, ny)];
            if (grid[&(x, y)] + 1 >= elevation) && !visited.contains(&(nx, ny)) {
                to_process.push_back(((nx, ny), len + 1));
                visited.insert((nx, ny));
            }
        }
    }
    None
}

fn part1(
    grid: &HashMap<(usize, usize), u8>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    find_route(grid, start, end)
}

fn part2(grid: &HashMap<(usize, usize), u8>, end: (usize, usize)) -> Option<usize> {
    grid.into_iter()
        .filter_map(|(k, v)| match v {
            b'a' => find_route(grid, *k, end),
            _ => None,
        })
        .min()
}

fn main() {
    let input = include_str!("input");
    let (grid, start, stop) = parse_input(input);
    println!("Part1: {}", part1(&grid, start, stop).unwrap());
    println!("Part2: {}", part2(&grid, stop).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        let (grid, start, stop) = parse_input(INPUT);
        assert_eq!(part1(&grid, start, stop), Some(31));
    }

    #[test]
    fn test_part2() {
        let (grid, _start, stop) = parse_input(INPUT);
        assert_eq!(part2(&grid, stop), Some(29));
    }
}
