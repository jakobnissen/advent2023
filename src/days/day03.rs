// Parse to hashmap of (pos, (n, id)), with multiple pos per integer
// Go through, for each symbol: Get all neighbors

use std::collections::HashSet;

pub fn solve(s: &str) -> (usize, usize) {
    solve_parsed(&parse(s))
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Number { value: u16, id: u16 },
    Star,
    Other,
    None,
}

fn parse(s: &str) -> Vec<Vec<Cell>> {
    let lines: Vec<_> = s.lines().map(str::trim).filter(|s| !s.is_empty()).collect();
    let nrow = lines.len();
    let ncol = lines[0].len();
    // TODO: Check same col
    let mut result = vec![vec![Cell::None; ncol]; nrow];
    let mut id: u16 = 0;
    for (row, line) in lines.iter().enumerate() {
        let mut first = usize::MAX;
        let mut value: u16 = 0;
        for (col, &byte) in line.as_bytes().iter().enumerate() {
            if (0x30..=0x39).contains(&byte) {
                if first == usize::MAX {
                    first = col;
                }
                value = 10 * value + (byte - 0x30) as u16;
                continue;
            } else if first != usize::MAX {
                for i in first..col {
                    result[row][i] = Cell::Number { value, id };
                }
                id += 1;
                value = 0;
                first = usize::MAX;
            }
            if byte == b'.' {
                continue;
            } else if byte == b'*' {
                result[row][col] = Cell::Star;
            } else {
                result[row][col] = Cell::Other;
            }
        }
        if first != usize::MAX {
            for i in first..ncol {
                result[row][i] = Cell::Number { value, id };
            }
            id += 1;
        }
    }
    result
}

#[allow(clippy::needless_range_loop)]
fn solve_parsed(matrix: &[Vec<Cell>]) -> (usize, usize) {
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut seen_p1_numbers: HashSet<u16> = HashSet::new();
    let mut star_neighbors: HashSet<(u16, u16)> = HashSet::new();
    let nrow = matrix.len();
    let ncol = matrix[0].len();
    for (row, v) in matrix.iter().enumerate() {
        for (col, cell) in v.iter().enumerate() {
            let is_star = if let Cell::Star = cell {
                star_neighbors.clear();
                true
            } else {
                false
            };
            match cell {
                Cell::Other | Cell::Star => {
                    for rowi in row.saturating_sub(1)..nrow.min(row + 2) {
                        for coli in col.saturating_sub(1)..ncol.min(col + 2) {
                            if let Cell::Number { value, id } = matrix[rowi][coli] {
                                if seen_p1_numbers.insert(id) {
                                    p1 += value as usize
                                }
                                if is_star {
                                    star_neighbors.insert((value, id));
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
            if is_star && star_neighbors.len() == 2 {
                p2 += star_neighbors
                    .iter()
                    .fold(1, |acc, (n, _)| acc * *n as usize);
            }
        }
    }
    (p1, p2)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (4361, 467835));
    }
}
