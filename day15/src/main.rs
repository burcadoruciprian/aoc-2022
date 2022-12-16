use ranges::{GenericRange, Ranges};
use std::{
    collections::HashSet,
    ops::{Bound, RangeBounds},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
    range: isize,
}

impl Sensor {
    fn new(x1: isize, y1: isize, x2: isize, y2: isize) -> Self {
        Sensor {
            pos: Pos::new(x1, y1),
            beacon: Pos::new(x2, y2),
            range: (x1 - x2).abs() + (y1 - y2).abs(),
        }
    }
}

fn range_len(range: &GenericRange<isize>) -> isize {
    match (range.start_bound(), range.end_bound()) {
        (Bound::Included(start), Bound::Included(end)) => end - start + 1,
        (Bound::Excluded(start), Bound::Excluded(end)) => end - start - 1,
        (Bound::Included(start), Bound::Excluded(end)) => end - start,
        (Bound::Excluded(start), Bound::Included(end)) => end - start,
        _ => panic!("Range len cannot be deduced"),
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .trim()
        .lines()
        .map(|l| {
            let coords = l
                .split(&[' ', '='][..])
                .filter_map(|c| c.trim_end_matches(&[',', ':'][..]).parse().ok())
                .collect::<Vec<isize>>();
            assert_eq!(coords.len(), 4);
            Sensor::new(coords[0], coords[1], coords[2], coords[3])
        })
        .collect()
}

fn part1(sensors: &Vec<Sensor>, y: isize) -> isize {
    //Ontain ranges where th y line intersects the senors sweep circle (ising manhatan distance ofcourse)
    let mut y_intersect_ranges = Ranges::new();
    sensors.iter().for_each(|s| {
        let hs = s.range - (y - s.pos.y).abs();
        if (hs * 2 + 1) > 0 {
            y_intersect_ranges.insert((s.pos.x - hs)..=(s.pos.x + hs));
        }
    });

    let beacons_in_ranges: HashSet<isize> = sensors
        .iter()
        .filter_map(
            |s| match s.beacon.y == y && y_intersect_ranges.contains(&s.beacon.x) {
                true => Some(s.beacon.x),
                false => None,
            },
        )
        .collect();

    y_intersect_ranges
        .as_slice()
        .iter()
        .fold(0, |acc: isize, r| acc + range_len(r))
        - beacons_in_ranges.len() as isize
}

fn part2(sensors: &Vec<Sensor>, max_y: isize) -> Option<usize> {
    let intersect_ranges_at_line = |y: isize| -> Ranges<isize> {
        let mut y_intersect_ranges = Ranges::new();
        sensors.iter().for_each(|s| {
            let hs = s.range - (y - s.pos.y).abs();
            if (hs * 2 + 1) > 0 {
                y_intersect_ranges.insert((s.pos.x - hs)..=(s.pos.x + hs));
            }
        });

        y_intersect_ranges
    };

    //Look for a line where we have more than one continous
    for y in 0..=max_y {
        let r = intersect_ranges_at_line(y);
        if r.len() > 1 {
            let x = match r.as_slice().first().unwrap().end_bound() {
                Bound::Included(end) => end + 1,
                _ => panic!("Wrong bound"),
            };
            return Some(4000000 * x as usize + y as usize);
        }
    }

    None
}

fn main() {
    let sensors = parse_input(include_str!("input"));
    println!("Part1: {}", part1(&sensors, 2000000));
    println!("Part2: {}", part2(&sensors, 4000000).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn test_part1() {
        let sensors = parse_input(INPUT);
        assert_eq!(part1(&sensors, 10), 26);
    }

    #[test]
    fn test_part2() {
        let sensors = parse_input(INPUT);
        assert_eq!(part2(&sensors, 4000000), Some(56000011));
    }
}
