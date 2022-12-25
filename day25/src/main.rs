fn snafu_to_dec(snafu: &str) -> isize {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '-' => -1 * 5_isize.pow(i as u32),
            '=' => -2 * 5_isize.pow(i as u32),
            _ => c.to_digit(10).unwrap() as isize * 5_isize.pow(i as u32),
        })
        .sum()
}

fn dec_to_snafu(val: isize) -> String {
    let mut snafu = Vec::new();
    let mut val = val;

    loop {
        match val % 5 {
            0 => snafu.push('0'),
            1 => snafu.push('1'),
            2 => snafu.push('2'),
            3 => {
                snafu.push('=');
                val += 2;
            }
            4 => {
                snafu.push('-');
                val += 1
            }
            _ => unreachable!(),
        };
        val /= 5;

        if val == 0 {
            break;
        }
    }

    String::from_iter(snafu.into_iter().rev())
}

fn main() {
    println!(
        "Part1: {}",
        dec_to_snafu(
            include_str!("input")
                .trim()
                .lines()
                .map(snafu_to_dec)
                .sum::<isize>()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_dec() {
        assert_eq!(snafu_to_dec("1"), 1);
        assert_eq!(snafu_to_dec("1121-1110-1=0"), 314159265);
        assert_eq!(snafu_to_dec("1=-0-2"), 1747);
    }

    #[test]
    fn test_dec_to_snafu() {
        assert_eq!(dec_to_snafu(1), "1");
        assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0");
        assert_eq!(dec_to_snafu(1747), "1=-0-2");
    }
}
