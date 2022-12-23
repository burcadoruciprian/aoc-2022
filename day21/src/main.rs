use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MonkeyBusiness {
    value: Option<isize>,
    letft: Option<String>,
    right: Option<String>,
    operator: Option<String>,
}

impl MonkeyBusiness {
    fn new(
        value: Option<isize>,
        letft: Option<String>,
        right: Option<String>,
        operator: Option<String>,
    ) -> Self {
        MonkeyBusiness {
            value,
            letft,
            right,
            operator,
        }
    }

    fn yell(&self, monkeys: &HashMap<String, MonkeyBusiness>) -> isize {
        if self.value.is_some() {
            return self.value.unwrap();
        }
        let lm = monkeys.get(&self.letft.clone().unwrap()).unwrap();
        let rm = monkeys.get(&self.right.clone().unwrap()).unwrap();
        match self.operator.clone().unwrap().as_str() {
            "+" => lm.yell(monkeys) + rm.yell(monkeys),
            "*" => lm.yell(monkeys) * rm.yell(monkeys),
            "/" => lm.yell(monkeys) / rm.yell(monkeys),
            "-" => lm.yell(monkeys) - rm.yell(monkeys),
            _ => panic!("unknown operator"),
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, MonkeyBusiness> {
    input
        .trim()
        .lines()
        .map(|l| {
            let tokens = l.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
            match tokens.len() {
                2 => (
                    tokens[0].trim_end_matches(':').to_string(),
                    MonkeyBusiness::new(Some(tokens[1].parse().unwrap()), None, None, None),
                ),
                4 => (
                    tokens[0].trim_end_matches(':').to_string(),
                    MonkeyBusiness::new(
                        None,
                        Some(tokens[1].clone()),
                        Some(tokens[3].clone()),
                        Some(tokens[2].clone()),
                    ),
                ),
                _ => panic!("malformed input line"),
            }
        })
        .collect::<HashMap<_, _>>()
}

fn part1(monkeys: &HashMap<String, MonkeyBusiness>) -> isize {
    monkeys["root"].yell(monkeys)
}

fn part2(monkeys: &HashMap<String, MonkeyBusiness>) -> isize {
    let mut mks = monkeys.clone();

    mks.entry(String::from("root"))
        .and_modify(|v| v.operator = Some(String::from("-")));
    let mut hv = 300;
    let mut hv_low = 0_isize;
    let mut hv_high = 0_isize;
    loop {
        mks.entry(String::from("humn"))
            .and_modify(|v| v.value = Some(hv));
        let rv = mks["root"].yell(&mks.clone());
        //println!("Root: {} Humn: {}", rv, hv);
        if rv == 0 {
            return hv;
        }

        match rv < 0 {
            true => hv_low = hv,
            false => hv_high = hv,
        }

        if hv_low == hv_high
        {
            panic!("well shiiiit !!!")
        }

        hv = match hv_low == 0 || hv_high == 0 {
            true => hv * 2,
            false => (hv_low + hv_high) / 2 ,
        }
    }
}

fn main() {
    let mks = parse_input(include_str!("input"));
    println!("Part1 {}", part1(&mks));
    println!("Part2 {}", part2(&mks));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1() {
        let mks = parse_input(INPUT);
        assert_eq!(part1(&mks), 152);
    }

    #[test]
    fn test_part2() {
        let mks = parse_input(INPUT);
        assert!([301_isize, 302].contains(&part2(&mks)));
    }
}
