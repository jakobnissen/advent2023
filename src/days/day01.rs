const NUMBERS: &[&str; 10] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_line(bytes: &[u8]) -> (u8, u8) {
    let mut first_p1 = 0xff;
    let mut last_p1 = 0xff;
    let mut first_p2 = 0xff;
    let mut last_p2 = 0xff;
    for (i, byte) in bytes.iter().enumerate() {
        let mut digit = 0xff;
        let mut is_numeric = false;
        if (0x30..0x40).contains(byte) {
            digit = byte - 0x30;
            is_numeric = true;
        } else {
            for (potential_digit, number) in NUMBERS.iter().enumerate() {
                if bytes[i..bytes.len()].starts_with(number.as_bytes()) {
                    digit = potential_digit as u8;
                    break;
                }
            }
        }
        if digit == 0xff {
            continue;
        }
        if is_numeric {
            if first_p1 == 0xff {
                first_p1 = digit;
            }
            last_p1 = digit;
        }
        if first_p2 == 0xff {
            first_p2 = digit;
        }
        last_p2 = digit;
    }
    let p1 = 10 * first_p1 + last_p1;
    let p2 = 10 * first_p2 + last_p2;
    (p1, p2)
}

pub fn solve(s: &str) -> (usize, usize) {
    s.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .fold((0, 0), |(p1, p2), line| {
            let (p1_new, p2_new) = parse_line(line.as_bytes());
            (p1 + usize::from(p1_new), p2 + usize::from(p2_new))
        })
}
