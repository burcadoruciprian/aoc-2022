use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Mult(usize),
    Pow2,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisible_by: usize,
    next: (usize, usize),
    inspection: usize,
}

impl Monkey {
    fn from_flat(input: &str) -> Self {
        let last_value = |text: &str| -> usize {
            text.split_ascii_whitespace()
                .last()
                .map(|x| x.parse().unwrap())
                .unwrap()
        };
        let lines: Vec<&str> = input.split('\n').map(|l| l.trim()).collect();
        let items = lines[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let operation = lines[2]
            .split("= ")
            .last()
            .map(|raw_op| {
                if raw_op == "old * old" {
                    Operation::Pow2
                } else if let Some(x) = raw_op.split_once("+ ") {
                    Operation::Add(x.1.parse().unwrap())
                } else if let Some(x) = raw_op.split_once("* ") {
                    Operation::Mult(x.1.parse().unwrap())
                } else {
                    unreachable!()
                }
            })
            .unwrap();

        let divisible_by = last_value(lines[3]);
        let next = (last_value(lines[4]), last_value(lines[5]));

        Monkey {
            items,
            operation,
            divisible_by,
            next,
            inspection: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::from_flat).collect()
}

fn round(monkeys: &mut Vec<Monkey>, worry_relief_fn: impl Fn(usize) -> usize) {
    for i in 0..monkeys.len() {
        let cm = monkeys[i].clone();
        cm.items.iter().for_each(|item| {
            monkeys[i].inspection += 1;
            let mut worry_level = match cm.operation {
                Operation::Add(x) => item + x,
                Operation::Mult(x) => item * x,
                Operation::Pow2 => item * item,
            };

            worry_level = worry_relief_fn(worry_level);

            match worry_level % cm.divisible_by == 0 {
                true => monkeys[cm.next.0].items.push(worry_level),
                false => monkeys[cm.next.1].items.push(worry_level),
            }
        });
        monkeys[i].items.clear();
    }
}

fn part1(monkeys: &Vec<Monkey>) -> usize {
    let mut monkeys = monkeys.clone();

    (0..20).for_each(|_| {
        round(&mut monkeys, |x| x / 3);
    });

    monkeys
        .iter()
        .map(|m| m.inspection)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

fn part2(monkeys: &Vec<Monkey>) -> usize {
    let mut monkeys = monkeys.clone();

    //relief number is calculated using Chinese remainder theorem [https://en.wikipedia.org/wiki/Chinese_remainder_theorem]
    let relief_magic_no: usize = monkeys.iter().map(|m| m.divisible_by).product();

     (0..10_000).for_each(|_| {
        round(&mut monkeys, |x| x % relief_magic_no);
    });
    monkeys
        .iter()
        .map(|m| m.inspection)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

fn main() {
    let input = include_str!("input").trim();
    let monkeys = parse_input(input);
    println!("Part1: {}", part1(&monkeys));
    println!("Part2: {}", part2(&monkeys));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monke 2
    If false: throw to monke 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monke 2
    If false: throw to monke 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monke 1
    If false: throw to monke 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monke 0
    If false: throw to monke 1"#;

    #[test]
    fn test_part1() {
        let monkeys = parse_input(INPUT);
        assert_eq!(part1(&monkeys), 10605);
    }

    #[test]
    fn test_part2() {
        let monkeys = parse_input(INPUT);
        assert_eq!(part2(&monkeys), 2713310158);
    }
}
