use itertools::Itertools;

fn parse_input(input: &str) -> ([Vec<char>; 9], Vec<(usize, usize, usize)>) {
    let (crt, ins) = input.split_once("\n\n").unwrap();

    let mut crates: [Vec<char>; 9] = Default::default();

    crt.lines().rev().skip(1).for_each(|l| {
        l.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .enumerate()
            .for_each(|(i, v)| {
                if !v.iter().all(|c| c.is_whitespace()) {
                    crates[i].push(v[1]);
                }
            });
    });

    let instructions = ins
        .lines()
        .map(|l| {
            l.split(' ')
                .filter_map(|t| t.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    (crates, instructions)
}

fn run(
    crates: &[Vec<char>; 9],
    instructions: &Vec<(usize, usize, usize)>,
    bulk_move: bool,
) -> String {
    let mut tmp = crates.clone();
    for (no, src, dest) in instructions {
        let to_shift = (0..*no)
            .map(|_| tmp[*src - 1].pop().unwrap())
            .collect::<Vec<char>>();

        if bulk_move {
            to_shift.iter().rev().for_each(|v| tmp[*dest - 1].push(*v));
        } else {
            to_shift.iter().for_each(|v| tmp[*dest - 1].push(*v));
        }
    }

    tmp.iter().filter_map(|c| c.last()).collect::<String>()
}

fn main() {
    let (crates, instructions) = parse_input(include_str!("input"));
    println!("Part1: {}", run(&crates, &instructions, false));
    println!("Part2: {}", run(&crates, &instructions, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let (crates, instructions) = parse_input(INPUT);
        assert_eq!(run(&crates, &instructions, false), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (crates, instructions) = parse_input(INPUT);
        assert_eq!(run(&crates, &instructions, true), "MCD");
    }
}
