pub fn solve(s: &str) -> (usize, usize) {
    let (rows, cols) = parse(s);
    (distance(&rows, &cols, 2), distance(&rows, &cols, 1_000_000))
}

fn parse(s: &str) -> (Vec<u16>, Vec<u16>) {
    let (mut rows, mut cols) = (Vec::new(), Vec::new());
    let mut len: Option<usize> = None;
    for line in s.lines().map(str::trim).filter(|s| !s.is_empty()) {
        let bytes = line.as_bytes();
        if let Some(x) = len {
            if x != bytes.len() {
                panic!();
            }
        } else {
            len = Some(bytes.len());
            rows.resize(bytes.len(), 0);
        }
        for (r, &b) in rows.iter_mut().zip(bytes.iter()) {
            *r += (b == b'#') as u16
        }
        cols.push(
            bytes
                .iter()
                .filter(|&&i| i == b'#')
                .count()
                .try_into()
                .unwrap(),
        )
    }
    (rows, cols)
}

fn distance(rows: &[u16], cols: &[u16], expansion: usize) -> usize {
    distance_in_dimension(rows, expansion) + distance_in_dimension(cols, expansion)
}

fn distance_in_dimension(v: &[u16], expansion: usize) -> usize {
    let (mut stars, mut units_of_distance, mut total_distance) = (0, 0, 0);
    for &n_new_stars in v.iter() {
        // Update the units of distance from the first element until the current element
        // times the total number of stars.
        units_of_distance += stars * {
            if n_new_stars > 0 {
                1
            } else {
                expansion
            }
        };
        // Every new star adds `unit_of_distance` distance in this dimension
        total_distance += units_of_distance * (n_new_stars as usize);
        // Add new stars
        stars += n_new_stars as usize;
    }
    total_distance
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR).0, 374);
    }
}
