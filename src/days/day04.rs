pub fn solve(s: &str) -> (usize, usize) {
    const LEN: usize = 10;
    let mut copies = [1_u32; LEN];
    let mut left_side = [0u8; LEN];
    let mut p1 = 0;
    let mut p2 = 0;
    for line in s.lines().map(str::trim).filter(|s| !s.is_empty()) {
        let (left, right) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        for (i, n) in left.split_ascii_whitespace().enumerate() {
            left_side[i] = n.parse().unwrap()
        }
        let n_overlap = right.split_ascii_whitespace().filter(|n| left_side.contains(&n.parse::<u8>().unwrap())).count();
        if n_overlap > 0 {
            p1 += 2_usize.pow((n_overlap-1) as u32);
        }
        let copy = copies[0];
        copies.rotate_left(1);
        copies[LEN-1] = 1;
        for i in 0..n_overlap {
            copies[i] += copy
        }
        p2 += copy;
    }
    (p1, p2 as usize)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (13, 30));
    }
}
