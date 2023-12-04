// TODO: Redo parsing, maybe with some kind of crate?
// solve_parsed can be more elegant

struct Draw([u32; 3]);

impl Draw {
    fn empty() -> Self {
        Draw([0; 3])
    }
}

struct Game(Vec<Draw>);

impl Game {
    fn is_possible(&self, max: Draw) -> bool {
        self.0
            .iter()
            .all(|draw| draw.0.iter().zip(max.0.iter()).all(|(a, b)| a <= b))
    }

    #[allow(clippy::needless_range_loop)]
    fn max_drawn(&self) -> Draw {
        let mut result = Draw::empty().0;
        for d in self.0.iter() {
            for i in 0..result.len() {
                result[i] = result[i].max(d.0[i])
            }
        }
        Draw(result)
    }
}

const MAX_DRAW: Draw = Draw([12, 13, 14]);

// TODO: Draw twice in one round - validate
fn parse_draw(s: &str) -> Draw {
    let mut result = Draw([0; 3]);
    for cube in s.trim().split(", ") {
        let (a, b) = cube.trim().split_once(' ').unwrap();
        let n = a.parse::<u32>().unwrap();
        match b {
            "red" => result.0[0] = n,
            "green" => result.0[1] = n,
            "blue" => result.0[2] = n,
            _ => unreachable!(),
        };
    }
    result
}

// TODO: Compile time regex
fn parse_game(s: &str) -> Game {
    let (_, rest) = s.split_once(": ").unwrap();
    Game(rest.split("; ").map(parse_draw).collect())
}

fn parse(s: &str) -> Vec<Game> {
    s.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_game)
        .collect()
}

// TODO Slice of as ref slice?
fn solve_parsed(v: &[Game]) -> (usize, usize) {
    v.iter().enumerate().fold((0, 0), |(p1, p2), (i, game)| {
        (
            p1 + (i + 1) * (game.is_possible(MAX_DRAW) as usize),
            p2 + game
                .max_drawn()
                .0
                .iter()
                .map(|i| *i as usize)
                .product::<usize>(),
        )
    })
}

pub fn solve(s: &str) -> (usize, usize) {
    solve_parsed(&parse(s))
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (8, 2286));
    }
}
