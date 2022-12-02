use itertools::Itertools;

fn score_part1(round: (&str, &str)) -> usize {
    match round {
        ("A", "X") => 3 + 1, //draw
        ("A", "Y") => 6 + 2, //win
        ("A", "Z") => 0 + 3, //loose
        ("B", "X") => 0 + 1, //loose
        ("B", "Y") => 3 + 2, //draw
        ("B", "Z") => 6 + 3, //win
        ("C", "X") => 6 + 1, //win
        ("C", "Y") => 0 + 2, //loose
        ("C", "Z") => 3 + 3, //draw
        _ => panic!("Unsupported pattern"),
    }
}

fn score_part2(round: (&str, &str)) -> usize {
    match round {
        ("A", "X") => 0 + 3, //need to loose, play scissors with score 0 + 3
        ("A", "Y") => 3 + 1, //need to draw, play rock with core 3 + 1
        ("A", "Z") => 6 + 2, //need to win, play paper with score 6 + 2
        ("B", "X") => 0 + 1, //need to loose, play rock with score 0 + 1
        ("B", "Y") => 3 + 2, //need to draw, play paper with core 3 + 2
        ("B", "Z") => 6 + 3, //need to win, play scissors with score 6 + 3
        ("C", "X") => 0 + 2, //need to loose, play paper with score 0 + 3
        ("C", "Y") => 3 + 3, //need to draw, play scissors with core 3 + 3
        ("C", "Z") => 6 + 1, //need to win, play rock with score 6 + 1
        _ => panic!("Unsupported pattern"),
    }
}

fn solve(input: &str, score: fn((&str, &str)) -> usize) -> usize {
    let rounds = input
        .trim()
        .split('\n')
        .map(|l| l.split_once(' ').unwrap())
        .collect_vec();

    rounds.iter().fold(0, |acc, r| acc + score(*r))
}

fn main() {
    let input = include_str!("input");
    println!("Part1: {}", solve(input, score_part1));
    println!("Part2: {}", solve(input, score_part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_1() {
        assert_eq!(solve(INPUT, score_part1), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(INPUT, score_part2), 12);
    }
}
