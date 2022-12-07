use indextree::{Arena, NodeId};

#[derive(Debug, Clone)]
pub struct Dir {
    name: String,
    size: usize,
}

impl Dir {
    fn new(name: &str, size: usize) -> Self {
        Dir {
            name: name.to_string(),
            size,
        }
    }
}

fn exec_commands(input: &str) -> (NodeId, Arena<Dir>) {
    let arena = &mut Arena::new();
    let mut crt_dir = arena.new_node(Dir::new("/", 0));
    let root = crt_dir;

    input.trim().lines().skip(1).for_each(|l| {
        if l.starts_with("$ cd") {
            let dir_name = l.strip_prefix("$ cd ");

            crt_dir = match dir_name {
                Some("..") => arena.get(crt_dir).unwrap().parent().unwrap(),
                Some(_) => crt_dir
                    .children(arena)
                    .find(|c| arena.get(*c).unwrap().get().name == dir_name.unwrap())
                    .unwrap_or_else(|| {
                        let nc = arena.new_node(Dir::new(dir_name.unwrap(), 0));
                        crt_dir.append(nc, arena);
                        nc
                    }),
                None => panic!("Invalid command"),
            };
        } else if l.chars().take(1).last().unwrap().is_numeric() {
            let size: usize = l.split_once(" ").unwrap().0.parse().unwrap();
            arena.get_mut(crt_dir).unwrap().get_mut().size += size;
        }
    });

    (root, arena.to_owned())
}

fn part1(root: NodeId, arena: &Arena<Dir>) -> usize {
    root.descendants(arena).fold(0, |acc, d| {
        let s = d
            .descendants(arena)
            .fold(0, |acc, dd| acc + arena.get(dd).unwrap().get().size);
        match s <= 100000 {
            true => acc + s,
            false => acc,
        }
    })
}

fn part2(root: NodeId, arena: &Arena<Dir>) -> usize {
    let ocupied_size = root
        .descendants(arena)
        .fold(0, |acc, d| acc + arena.get(d).unwrap().get().size);

    let needed = ocupied_size - 40000000;

    root.descendants(arena)
        .map(|d| {
            d.descendants(arena)
                .fold(0, |acc, dd| acc + arena.get(dd).unwrap().get().size)
        })
        .filter(|s| *s >= needed)
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("input");

    let (root, arena) = exec_commands(input);
    //dbg!(root.debug_pretty_print(&arena));
    println!("Part1: {}", part1(root, &arena));
    println!("Part2: {}", part2(root, &arena));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        let (root, arena) = exec_commands(INPUT);
        assert_eq!(part1(root, &arena), 95437);
    }

    #[test]
    fn test_part2() {
        let (root, arena) = exec_commands(INPUT);
        assert_eq!(part2(root, &arena), 24933642);
    }
}
