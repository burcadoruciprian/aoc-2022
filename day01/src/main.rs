use itertools::Itertools;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split('\n')
                .fold(0, |acc, v| acc + v.parse::<usize>().unwrap_or(0))
        })
        .sorted_by(|a, b| b.cmp(a))
        .collect_vec()
}
fn main() {
    let input = include_str!("input");
    let calories = parse_input(input);
    //calories.sort_by(|a, b| b.cmp(a));
    println!("Part1: {}", calories.first().unwrap());
    println!("Part2: {}", calories.iter().take(3).sum::<usize>());
}
