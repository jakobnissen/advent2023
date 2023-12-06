fn parse(s: &str) -> (Vec<(usize, usize)>, (usize, usize)) {
    let mut lines = s
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<usize>().unwrap())
        });
    let shift = |a, b: usize| a * (10_usize.pow(b.ilog10() + 1)) + b;
    let p1: Vec<_> = lines.next().unwrap().zip(lines.next().unwrap()).collect();
    let p2 = p1
        .iter()
        .fold((0, 0), |(a, b), (i, j)| (shift(a, *i), shift(b, *j)));
    (p1, p2)
}

fn solve_parsed(v: &[(usize, usize)]) -> usize {
    v.iter()
        .map(|(time, distance)| solve_race(*time, *distance))
        .product::<usize>()
}

fn solve_race(time: usize, distance: usize) -> usize {
    let d = (time * time - 4 * distance) as f64;
    let min = (((-(time as f64) + d.sqrt()) / -2.0) + 1.0).floor() as usize;
    let max = (((-(time as f64) - d.sqrt()) / -2.0) - 1.0).ceil() as usize;
    max - min + 1
}

pub fn solve(s: &str) -> (usize, usize) {
    let (v, (time, distance)) = parse(s);
    (solve_parsed(&v), solve_race(time, distance))
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (288, 71503));
    }
}
