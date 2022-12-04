use itertools::Itertools;
use std::collections::HashSet;

fn item_weight(item: char) -> u32 {
    match item.is_ascii_lowercase() {
        true => item as u32 - 96,
        false => item as u32 - 64 + 26,
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_at(l.len() / 2);
            let h1: HashSet<char> = l.chars().collect();
            let h2: HashSet<char> = r.chars().collect();
            h1.intersection(&h2).fold(0, |acc, c| acc + item_weight(*c))
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| {
            c.map(|r| HashSet::from_iter(r.chars()))
                .reduce(|acc: HashSet<char>, v| acc.intersection(&v).cloned().collect())
                .unwrap()
                .iter()
                .fold(0, |acc, v| acc + item_weight(*v))
        })
        .sum()
}

fn main() {
    let input = include_str!("input").trim();
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 70);
    }
}
