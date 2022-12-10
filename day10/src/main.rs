#[derive(Debug)]
enum OP {
    Noop,
    Addx,
}

fn part1(instructions: &Vec<(OP, isize)>) -> isize {
    let ckeck_cycle = |cycle: isize, x: isize| -> isize {
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => cycle * x,
            _ => 0,
        }
    };

    let mut cycle = 0_isize;
    let mut x = 1;
    let mut strength = 0;

    for (op, n) in instructions {
        match op {
            OP::Noop => {
                cycle += 1;
                strength += ckeck_cycle(cycle, x);
            }
            OP::Addx => {
                cycle += 1;
                strength += ckeck_cycle(cycle, x);
                cycle += 1;
                strength += ckeck_cycle(cycle, x);
                x += n;
            }
        }
    }
    strength
}

fn part2(instructions: &Vec<(OP, isize)>) -> Vec<Vec<char>> {
    let get_pixel = |pos: usize, x: isize| -> char {
        match ((x - 1)..=(x + 1)).contains(&(pos as isize)) {
            true => '#',
            false => '.',
        }
    };
    let crt_row = |cycle: usize| -> usize { ((cycle - 1) / 40) as usize };
    let crt_col = |cycle: usize| -> usize { ((cycle - 1) % 40) as usize };

    let mut crt: Vec<Vec<char>> = vec![vec![]; 6];
    let mut cycle = 0_usize;
    let mut x = 1_isize;

    for (op, n) in instructions {
        match op {
            OP::Noop => {
                cycle += 1;
                crt.get_mut(crt_row(cycle))
                    .unwrap()
                    .push(get_pixel(crt_col(cycle), x));
            }

            OP::Addx => {
                cycle += 1;
                crt.get_mut(crt_row(cycle))
                    .unwrap()
                    .push(get_pixel(crt_col(cycle), x));

                cycle += 1;
                crt.get_mut(crt_row(cycle))
                    .unwrap()
                    .push(get_pixel(crt_col(cycle), x));
                x += n;
            }
        }
    }
    crt
}

fn parse_input(input: &str) -> Vec<(OP, isize)> {
    input
        .trim()
        .lines()
        .map(|l| match l.starts_with("noop") {
            true => (OP::Noop, 0),
            false => (
                OP::Addx,
                l.strip_prefix("addx ").unwrap().parse::<isize>().unwrap(),
            ),
        })
        .collect()
}

fn main() {
    let input = include_str!("input");
    println!("Part1: {}\n", part1(&parse_input(input)));
    println!("Part2: \n");
    let crt = part2(&parse_input(input));
    let text = crt
        .iter()
        .map(|c| String::from_iter(c))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", text);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 13140);
    }

    #[test]
    fn test_part2() {
        let crt = part2(&parse_input(INPUT));
        let text = crt
            .iter()
            .map(|c| String::from_iter(c))
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", text);
    }
}
