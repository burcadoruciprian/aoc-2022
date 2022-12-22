use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<isize> {
    input.trim().lines().map(|l| l.parse().unwrap()).collect()
}

fn decript(coords: &Vec<isize>, decription_key: isize, mix_count: isize) -> isize {
    let coords = coords
        .iter()
        .map(|v| v * decription_key)
        .collect::<Vec<_>>();
    let mut decifered_pos = (0..coords.len()).collect::<Vec<_>>();

    (0..mix_count).for_each(|_|{
        coords.iter().enumerate().for_each(|(i, c)| {
            let old_pos = decifered_pos.iter().position(|&v| v == i).unwrap();
            decifered_pos.remove(old_pos);
            let len = decifered_pos.len() as isize;
            let new_pos = ((old_pos as isize + *c) % len + len) % len;
            decifered_pos.insert(new_pos as usize, i)
        });
    });

    let orig_zero_pos = coords.iter().position(|&v| v == 0).unwrap();
    let zero_pos = decifered_pos
        .iter()
        .position(|&i| i == orig_zero_pos)
        .unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|v| coords[decifered_pos[(zero_pos + v) % decifered_pos.len()]])
        .sum::<isize>()
}

fn main() {
    let coords = parse_input(include_str!("input"));
    println!("Part1: {}", decript(&coords, 1, 1));
    println!("Part2: {}", decript(&coords, 811589153, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part1() {
        let coords = parse_input(INPUT);
        assert_eq!(decript(&coords, 1,1), 3);
    }

    #[test]
    fn test_part2() {
         let coords = parse_input(INPUT);
        assert_eq!(decript(&coords, 811589153,10), 1623178306);
    }
}
