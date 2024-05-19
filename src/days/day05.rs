#[derive(Clone, Copy, Debug)]
struct Span(isize, isize);

impl Span {
    fn offset(self, offset: isize) -> Self {
        Self(self.0 + offset, self.1 + offset)
    }
}

pub fn solve(s: &str) -> (isize, isize) {
    let mut parsed = parse(s);
    let mut outputs: Vec<_> = Vec::new();
    (
        run(&parsed.maps, &mut parsed.p1_seeds, &mut outputs),
        run(&parsed.maps, &mut parsed.p2_seeds, &mut outputs),
    )
}

fn run(maps: &[Vec<(Span, isize)>], inputs: &mut Vec<Span>, outputs: &mut Vec<Span>) -> isize {
    for map in maps.iter() {
        translate(map, inputs, outputs);
        std::mem::swap(inputs, outputs)
    }
    inputs.iter().fold(isize::MAX, |acc, span| acc.min(span.0))
}

fn translate(maps: &[(Span, isize)], inputs: &mut Vec<Span>, outputs: &mut Vec<Span>) {
    outputs.clear();
    while let Some(mut span) = inputs.pop() {
        let mut is_emptied = false;
        for (dst, offset) in maps.iter() {
            match intersection(span, *dst) {
                None => (),
                Some(Intersection::Contained) => {
                    outputs.push(span.offset(*offset));
                    is_emptied = true;
                    break;
                }
                Some(Intersection::Partial { inside, outside }) => {
                    outputs.push(inside.offset(*offset));
                    span = outside;
                }
                Some(Intersection::Middle(a, b, c, d)) => {
                    outputs.push(Span(b, c).offset(*offset));
                    inputs.push(Span(a, b - 1));
                    span = Span(c + 1, d);
                }
            }
        }
        if !is_emptied {
            outputs.push(span)
        }
    }
    collapse(outputs);
}

fn collapse(v: &mut Vec<Span>) {
    if v.len() < 2 {
        return;
    }
    v.sort_unstable_by_key(|i| i.0);
    let (mut last_start, mut last_end) = { (v[0].0, v[0].1) };
    let mut write_index = 0;
    for i in 1..v.len() {
        let r = v[i];
        if r.0 <= last_end + 1 {
            last_end = last_end.max(r.1);
            v[write_index] = Span(last_start, last_end);
        } else {
            write_index += 1;
            v[write_index] = r;
            last_start = r.0;
            last_end = r.1;
        }
    }
    v.truncate(write_index + 1);
}

#[derive(Debug)]
enum Intersection {
    Contained,
    Partial { inside: Span, outside: Span },
    Middle(isize, isize, isize, isize),
}

fn intersection(src: Span, dst: Span) -> Option<Intersection> {
    // ---- (  )  or (  ) -----
    if src.1 < dst.0 || src.0 > dst.1 {
        None
    }
    // (  ---  )
    else if src.0 >= dst.0 && src.1 <= dst.1 {
        Some(Intersection::Contained)
    }
    // ---(---)---
    else if src.0 < dst.0 && src.1 > dst.1 {
        Some(Intersection::Middle(src.0, dst.0, dst.1, src.1))
    }
    // ---(---  )
    else if src.0 < dst.0 && src.1 <= dst.1 {
        Some(Intersection::Partial {
            inside: Span(dst.0, src.1),
            outside: Span(src.0, dst.0 - 1),
        })
    }
    // (  ---)---
    else if src.0 >= dst.0 && src.1 >= dst.1 {
        Some(Intersection::Partial {
            inside: Span(src.0, dst.1),
            outside: Span(dst.1 + 1, src.1),
        })
    } else {
        unreachable!()
    }
}

struct Parsed {
    p1_seeds: Vec<Span>,
    p2_seeds: Vec<Span>,
    maps: Vec<Vec<(Span, isize)>>,
}

fn parse(s: &str) -> Parsed {
    let mut lines = s.lines().map(str::trim).filter(|s| !s.is_empty());
    let seeds: Vec<_> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let p1_seeds: Vec<_> = seeds.iter().map(|n| Span(*n, *n)).collect();
    let p2_seeds: Vec<_> = seeds
        .chunks_exact(2)
        .map(|c| Span(c[0], c[0] + c[1] - 1))
        .collect();
    let mut maps: Vec<_> = Vec::new();
    for line in lines {
        if line.starts_with(|c: char| c.is_ascii_alphabetic()) {
            maps.push(Vec::new());
        } else {
            let mut ns = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<isize>().unwrap());
            let dst = ns.next().unwrap();
            let src = ns.next().unwrap();
            let len = ns.next().unwrap();
            let elem = (Span(src, src + len - 1), dst - src);
            if let Some(v) = maps.last_mut() {
                v.push(elem)
            }
        }
    }
    Parsed {
        p1_seeds,
        p2_seeds,
        maps,
    }
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (35, 46));
    }
}
