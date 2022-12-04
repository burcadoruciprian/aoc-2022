use itertools::Itertools;
fn parse_input(input: &str) -> Vec<(u32, u32, u32, u32)> {
    input
        .lines()
        .map(|l| {
            l.split(&['-', ','][..])
                .map(|t| t.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part1(pairs: &Vec<(u32, u32, u32, u32)>) -> u32 {
    pairs.iter().fold(0, |acc, v| {
        match (v.0 <= v.2 && v.1 >= v.3) || (v.0 >= v.2 && v.1 <= v.3) {
            true => acc + 1,
            false => acc,
        }
    })
}

fn part2(pairs: &Vec<(u32, u32, u32, u32)>) -> u32 {
    pairs
        .iter()
        .fold(0, |acc, v| match (v.1 < v.2) || (v.3 < v.0) {
            true => acc,
            false => acc + 1,
        })
}

fn main() {
    let input = include_str!("input");
    let pairs = parse_input(input);
    println!("Part1: {}", part1(&pairs));
    println!("Part2: {}", part2(&pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2-4,6-8
2-3,4-5
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 4);
    }
}
