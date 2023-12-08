use num;
use std::collections::HashMap;

// This struct identifies a code such as e.g. AKX
// The uppermost bit tells whether it ends with Z
#[derive(Debug, Clone, Copy)]
struct Identifier(u16);

impl Identifier {
    fn from(s: &str, x: u16) -> Self {
        if x > 0x7fff {
            panic!()
        }
        Self((((s.as_bytes().last().unwrap() == &b'Z') as u16) << 15) | x)
    }
}

#[derive(Debug, Clone, Copy)]
struct Pair([Identifier; 2]);

impl Pair {
    fn get(&self, side: bool) -> (Identifier, bool) {
        let i = self.0[side as usize];
        (i, (i.0 >> 15) != 0)
    }
}

struct Map(Vec<Pair>);

impl Map {
    fn from(h: HashMap<&str, (&str, &str)>) -> (Self, Identifier, Vec<Identifier>) {
        // Validate that all (left, right) are keys in the map
        for (left, right) in h.values() {
            if !(h.contains_key(left) && h.contains_key(right)) {
                panic!()
            }
        }

        let mut to_integer: HashMap<&str, u16> = HashMap::new();
        for k in h.keys() {
            let len: u16 = to_integer.len().try_into().unwrap();
            to_integer.insert(*k, len);
        }

        let p1_start = Identifier(*to_integer.iter().find(|(&k, _)| k == "AAA").unwrap().1);
        let p2_starts = to_integer
            .iter()
            .filter(|(k, _)| k.ends_with('A'))
            .map(|(_, v)| Identifier(*v))
            .collect();

        let mut v: Vec<Pair> = vec![Pair([Identifier(0), Identifier(0)]); h.len()];
        for (k, (l, r)) in h.iter() {
            v[to_integer[k] as usize] = Pair([
                Identifier::from(l, *to_integer.get(l).unwrap()),
                Identifier::from(r, *to_integer.get(r).unwrap()),
            ])
        }
        (Self(v), p1_start, p2_starts)
    }

    fn get(&self, i: Identifier) -> Pair {
        unsafe { *self.0.get_unchecked((i.0 & 0x7fff) as usize) }
    }
}

pub fn solve(s: &str) -> (usize, usize) {
    let (sides, map, p1_start, p2_starts) = parse(s);
    let p1 = get_cycle_length(&sides, &map, p1_start);
    let p2_cycle_lengths: Vec<_> = p2_starts
        .iter()
        .map(|&u| get_cycle_length(&sides, &map, u))
        .collect();
    let p2 = p2_cycle_lengths
        .iter()
        .fold(1usize, |acc, &i| num::integer::lcm(acc, i));
    (p1, p2)
}

fn parse(s: &str) -> (Vec<bool>, Map, Identifier, Vec<Identifier>) {
    let mut lines = s.lines().map(str::trim).filter(|s| !s.is_empty());
    let sides: Vec<_> = lines
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|&b| b == b'R')
        .collect();

    // Build a HashMap of the (from) -> (left, right) mappings
    let mut string_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let (from, x) = line.split_once(" = (").unwrap();
        let (left, right) = x.strip_suffix(')').unwrap().split_once(", ").unwrap();
        string_map.insert(from, (left, right));
    }

    let (map, p1_start, p2_starts) = Map::from(string_map);
    (sides, map, p1_start, p2_starts)
}

fn get_cycle_length(sides: &[bool], map: &Map, start: Identifier) -> usize {
    let mut pair = map.get(start);
    for (step, side) in sides.iter().cycle().enumerate() {
        let (i, end) = pair.get(*side);
        if end {
            return step + 1;
        }
        pair = map.get(i);
    }
    unreachable!()
}
