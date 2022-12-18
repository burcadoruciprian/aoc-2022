use std::{collections::HashSet, cmp};
use itertools::Itertools;

fn sides((x, y, z): (isize, isize, isize)) -> [(isize, isize, isize); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn parse_input(input: &str) -> HashSet<(isize, isize, isize)> {
    input
        .trim()
        .lines()
        .filter_map(|l| l.split(',').map(|x| x.parse().unwrap()).collect_tuple())
        .collect::<HashSet<_>>()
}

fn part1(drops: &HashSet<(isize, isize, isize)>) -> usize {
    drops
        .iter()
        .flat_map(|&p| sides(p))
        .filter(|s| !drops.contains(s))
        .count()
}

fn part2(drops: &HashSet<(isize, isize, isize)>) -> usize {
    let bound = drops.iter().map(|&(x, y, z)| cmp::max(x, cmp::max(y,z))).max().unwrap() + 1;

    let mut visited = HashSet::new();
    let mut stack = vec![(0, 0, 0)];

    while let Some(p) = stack.pop() {
        for (x, y, z) in sides(p) {
            if !drops.contains(&(x, y, z))
                && !visited.contains(&(x, y, z))
                && [x, y, z].iter().all(|&i| (-1..=bound).contains(&i))
            {
                visited.insert((x, y, z));
                stack.push((x, y, z));
            }
        }
    }

    drops
        .iter()
        .flat_map(|&p| sides(p))
        .filter(|s| visited.contains(s))
        .count()
}

fn main() {
    let drops = parse_input(include_str!("input"));
    println!("Part1: {}", part1(&drops));
    println!("Part2: {}", part2(&drops));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        let drops = parse_input(INPUT);
        assert_eq!(part1(&drops), 64);
    }

    #[test]
    fn test_part2() {
        let drops = parse_input(INPUT);
        assert_eq!(part2(&drops), 58);
    }
}
