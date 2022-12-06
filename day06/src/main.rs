use std::collections::HashSet;

fn run(input: &str, window_size: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
        .find(|(_i, v)| {
            let h: HashSet<&char> = HashSet::from_iter(v.iter());
            h.len() == window_size
        })
        .unwrap()
        .0
        + window_size
}

fn main() {
    let input = include_str!("input").trim();
    println!("Part1: {}", run(input, 4));
    println!("Part2: {}", run(input, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    }
}
