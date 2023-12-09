pub fn solve(s: &str) -> (isize, isize) {
    let mut v: Vec<isize> = Vec::with_capacity(25);
    let numbers = parse(s);
    let (p1, p2) = numbers.fold((0, 0), |(p1, p2), it| {
        v.clear();
        v.extend(it);
        let (line_p1, line_p2) = solve_parsed(&mut v);
        (p1 + line_p1, p2 + line_p2)
    });
    (p1, p2)
}

fn solve_parsed(v: &mut [isize]) -> (isize, isize) {
    let mut offset = 0;
    let len = v.len();
    while (offset..len).any(|i| v[i] != 0) {
        let mut left = v[offset];
        for i in v.iter_mut().skip(offset + 1) {
            let right = *i;
            *i -= left;
            left = right;
        }
        offset += 1;
    }
    let mut rightest: isize = 0;
    let mut leftest: isize = 0;
    for i in (0..offset).rev() {
        for j in (i + 1)..len {
            v[j] += v[j - 1]
        }
        rightest += v[len - 1];
        leftest = v[i] - leftest;
    }
    (rightest, leftest)
}

fn parse(s: &str) -> impl Iterator<Item = impl Iterator<Item = isize> + '_> + '_ {
    s.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
        })
}
