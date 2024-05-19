pub fn solve(s: &str) -> (usize, usize) {
    let (start, map) = Map::parse(s);
    let directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let coords: Vec<_> = directions
        .iter()
        .filter_map(|&d| match map.get_next(start, d) {
            GetNextResult::Result(a, b) => Some((a, b)),
            _ => None,
        })
        .collect();
    if coords.len() != 2 {
        panic!("Not exactly two direction from starting position")
    };
    let (mut coord, mut d) = coords[0];
    let mut steps: usize = 1; // we have already taken step 1
    let mut area: i64 = (start.0 * coord.1) as i64 - (coord.0 * start.1) as i64;
    loop {
        steps += 1;
        match map.get_next(coord, d) {
            GetNextResult::Done => {
                area += (coord.0 * start.1) as i64 - (coord.1 * start.0) as i64;
                area = area.abs() / 2;
                let inner = area - ((steps / 2) as i64) + 1;
                return (steps / 2, inner.try_into().unwrap());
            }
            GetNextResult::Result(new_coord, new_d) => {
                area += (coord.0 * new_coord.1) as i64 - (coord.1 * new_coord.0) as i64;
                (coord, d) = (new_coord, new_d);
            }
            _ => panic!(),
        }
    }
}

struct Map {
    v: Vec<Pipe>,
    y: usize,
    x: usize,
}

enum GetNextResult {
    Done,
    OutOfBounds,
    BadDirection,
    Result((usize, usize), Direction),
}

impl Map {
    fn parse(s: &str) -> ((usize, usize), Self) {
        let mut x: Option<usize> = None;
        let mut coord: Option<(usize, usize)> = None;
        let mut vec: Vec<Pipe> = Vec::new();
        for (rownum, line) in s
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .enumerate()
        {
            let bytes = line.as_bytes();
            match x {
                None => x = Some(bytes.len()),
                Some(len) => {
                    if len != bytes.len() {
                        panic!("Unequal row lengths")
                    }
                }
            };
            for (colnum, &char) in bytes.iter().enumerate() {
                let pipe = match &char {
                    b'|' => Pipe::Vertical,
                    b'-' => Pipe::Horizontal,
                    b'.' => Pipe::Ground,
                    b'J' => Pipe::NorthWest,
                    b'F' => Pipe::SouthEast,
                    b'7' => Pipe::SouthWest,
                    b'L' => Pipe::NorthEast,
                    b'S' => {
                        if coord.is_some() {
                            panic!("Two or more starting locations")
                        } else {
                            coord = Some((rownum, colnum));
                            Pipe::Start
                        }
                    }
                    _ => {
                        panic!("Unknown symbol: {}", char::from_u32(char.into()).unwrap())
                    }
                };
                vec.push(pipe);
            }
        }
        let map = match x {
            None => panic!("Must be at least one row"),
            Some(0) => panic!("Each row must have at least one element"),
            Some(x) => {
                let y = vec.len() / x;
                Map { v: vec, x, y }
            }
        };
        if let Some(c) = coord {
            (c, map)
        } else {
            panic!("Could not find starting position")
        }
    }

    fn get_next(&self, from_coord: (usize, usize), direction: Direction) -> GetNextResult {
        let to_coord = match self.update_coord(direction, from_coord) {
            None => return GetNextResult::OutOfBounds,
            Some(x) => x,
        };
        let (y, x) = to_coord;
        let pipe = unsafe { self.v.get_unchecked(self.x * y + x) };
        if matches!(pipe, Pipe::Start) {
            return GetNextResult::Done;
        }
        let new_dir = match new_direction(direction, *pipe) {
            None => return GetNextResult::BadDirection,
            Some(d) => d,
        };
        GetNextResult::Result(to_coord, new_dir)
    }

    fn update_coord(&self, direction: Direction, coord: (usize, usize)) -> Option<(usize, usize)> {
        let (y, x) = coord;
        match direction {
            Direction::East => {
                if x == self.x {
                    None
                } else {
                    Some((y, x + 1))
                }
            }
            Direction::South => {
                if y == self.y {
                    None
                } else {
                    Some((y + 1, x))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some((y, x - 1))
                }
            }
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some((y - 1, x))
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn new_direction(from: Direction, pipe: Pipe) -> Option<Direction> {
    match (from, pipe) {
        (Direction::North, Pipe::Vertical) => Some(Direction::North),
        (Direction::North, Pipe::SouthEast) => Some(Direction::East),
        (Direction::North, Pipe::SouthWest) => Some(Direction::West),
        (Direction::South, Pipe::Vertical) => Some(Direction::South),
        (Direction::South, Pipe::NorthEast) => Some(Direction::East),
        (Direction::South, Pipe::NorthWest) => Some(Direction::West),
        (Direction::West, Pipe::Horizontal) => Some(Direction::West),
        (Direction::West, Pipe::NorthEast) => Some(Direction::North),
        (Direction::West, Pipe::SouthEast) => Some(Direction::South),
        (Direction::East, Pipe::Horizontal) => Some(Direction::East),
        (Direction::East, Pipe::NorthWest) => Some(Direction::North),
        (Direction::East, Pipe::SouthWest) => Some(Direction::South),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    static TEST_STR: &str = "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (8, 1));
    }

    static TEST_STR_2: &str = ".....
    .S-7.
    .|.|.
    .L-J.
    .....";

    #[test]
    fn test_2() {
        assert_eq!(super::solve(TEST_STR_2), (4, 1));
    }

    static TEST_STR_3: &str = "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";

    #[test]
    fn test_3() {
        assert_eq!(super::solve(TEST_STR_3), (23, 4));
    }

    static TEST_STR_4: &str = ".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";

    #[test]
    fn test_4() {
        assert_eq!(super::solve(TEST_STR_4), (70, 8));
    }

    static TEST_STR_5: &str = "..........
    .S------7.
    .|F----7|.
    .||....||.
    .||....||.
    .|L-7F-J|.
    .|||||--|.
    .L--JL--J.
    ..........";

    #[test]
    fn test_5() {
        assert_eq!(super::solve(TEST_STR_5), (22, 4));
    }
}
