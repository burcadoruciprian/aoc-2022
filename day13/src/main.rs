use itertools::Itertools;
use nom::{
    branch::alt, character::complete as ch, combinator::map, multi::separated_list0,
    sequence::delimited, Finish, IResult,
};
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    Int(u8),
    List(Vec<Value>),
}

impl Value {
    fn from_str(input: &str) -> Self {
        fn parse(input: &str) -> IResult<&str, Value> {
            alt((
                delimited(
                    ch::char('['),
                    map(separated_list0(ch::char(','), parse), Value::List),
                    ch::char(']'),
                ),
                map(ch::u8, Value::Int),
            ))(input)
        }

        parse(input).finish().unwrap().1
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Int(a), Value::List(b)) => vec![Value::Int(*a)].cmp(b),
            (Value::List(a), Value::Int(b)) => a.cmp(&vec![Value::Int(*b)]),
            (Value::List(a), Value::List(b)) => a.cmp(b),
        }
    }
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|c| c.split('\n').map(Value::from_str).collect_tuple().unwrap())
        .enumerate()
        .filter_map(|(i, (a, b))| match a <= b {
            true => Some(i + 1),
            false => None,
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut values = input
        .trim()
        .split('\n')
        .filter_map(|t| match !t.is_empty() {
            true => Some(Value::from_str(t)),
            false => None,
        })
        .collect::<Vec<Value>>();

    let div1 = Value::from_str("[[2]]");
    let div2 = Value::from_str("[[6]]");
    values.extend_from_slice(&[div1.clone(), div2.clone()]);
    values.sort();

    values
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v == div1 || v == div2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

fn main() {
    let input = include_str!("input");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 140)
    }
}
