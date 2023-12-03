// TODO: If pick day not impl, do not error if file does not exist.
// 1. Parse days
// 2. Fetch functions for each day
// 3. Fetch data for each day
// 4. Compute and print

mod days;

use std::{
    cell::OnceCell,
    fmt::Display,
    path::{Path, PathBuf},
    time::Duration,
};

use clap::{self, Parser, Subcommand};
use reqwest::blocking::Client;

type TimedBoxes = (Duration, Box<dyn Display>, Box<dyn Display>);

fn to_boxed<F, A, B>(f: F, s: &str) -> Option<TimedBoxes>
where
    F: Fn(&str) -> (A, B),
    A: Display + 'static,
    B: Display + 'static,
{
    let start = std::time::Instant::now();
    let (a, b) = f(s);
    let elapsed = start.elapsed();
    Some((elapsed, Box::new(a), Box::new(b)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Day(u8);

// TODO: Proper error
impl Day {
    fn from_str(s: &str) -> Self {
        match s.parse::<u8>() {
            Err(_) => {
                eprint!("Error: Cannot parse \"{s}\" as integer in 1-25.");
                std::process::exit(1);
            }
            Ok(n) => {
                if !(1..=25).contains(&n) {
                    eprint!("Error: Day {n} not in 1-25.");
                    std::process::exit(1);
                }
                Day(n)
            }
        }
    }
}

fn parse_days<T: AsRef<str>>(v: &[T]) -> Vec<Day> {
    let mut days: Vec<_> = v.iter().map(|i| Day::from_str(i.as_ref())).collect();
    days.sort_unstable();
    days.dedup();
    days
}

// TODO: Add proper errors
fn load_day(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

// TODO: Handle errors
fn load_days(dir: &Path, days: &[Day]) -> Vec<(Day, String)> {
    days.iter()
        .map(|&day| (day, load_day(&dir.join(format!("day{:02}.txt", day.0)))))
        .collect()
}

fn solve_days(days: &[(Day, &str)]) -> Vec<Option<TimedBoxes>> {
    days.iter()
        .map(|(day, s)| match day {
            Day(1) => to_boxed(days::day01::solve, s),
            Day(2) => to_boxed(days::day02::solve, s),
            Day(3) => to_boxed(days::day03::solve, s),
            _ => None,
        })
        .collect()
}

fn print_solution(day: Day, solution: Option<TimedBoxes>) {
    print!("Day {:02}", day.0);
    if let Some((duration, a, b)) = solution {
        println!(" [{:.2?}]:", duration);
        println!("  Part 1: {}", a);
        println!("  Part 2: {}\n", b);
    } else {
        println!(":\n  Unimplemented!\n")
    }
}

fn solve<T: AsRef<str>>(data_dir: &Path, day_strings: &[T]) {
    if !data_dir.is_dir() {
        eprint!(
            "Data directory is not an existing file: \"{:#?}\"",
            data_dir
        );
        std::process::exit(1)
    }
    // TODO: Print timings (parsing + solving)
    let days = parse_days(day_strings);
    let data: Vec<(Day, String)> = load_days(&data_dir, &days);
    let solutions = solve_days(
        &data
            .iter()
            .map(|(d, s)| (*d, s.as_str()))
            .collect::<Vec<_>>(),
    );
    for (&day, solution) in days.iter().zip(solutions) {
        print_solution(day, solution)
    }
}

fn download(data_dir: &Path, day_strings: &[String]) {
    // Make dir and verify it exists
    if !data_dir.exists() {
        if data_dir.parent().is_none() {
            eprintln!(
                "Data dir \"{:#?}\" does not exist, and cannot be created because it has no parent",
                data_dir
            );
            std::process::exit(1)
        }
        std::fs::create_dir(data_dir).unwrap() // TODO: Error message
    } else if !data_dir.is_dir() {
        eprintln!(
            "Data dir \"{:#?}\" exists, but is not a directory",
            data_dir
        );
        std::process::exit(1)
    }
    let days = parse_days(&day_strings);
    if days.is_empty() {
        return;
    }
    let client_cell: OnceCell<Client> = OnceCell::new();
    for day in days.iter() {
        let path = data_dir.join(format!("day{:0>2}.txt", day.0));
        if path.exists() {
            println!("Input already exists: Day {:0>2}", day.0);
        } else {
            println!("Downloading day {:0>2}", day.0);
            let client = client_cell.get_or_init(make_client);
            let data = download_input(client, *day);
            std::fs::write(path, data).unwrap() // TODO: Proper error
        }
    }
}

fn make_client() -> Client {
    let mut headers = reqwest::header::HeaderMap::default();
    let session = match std::env::var("ADVENTOFCODE_SESSION") {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Error: Could not load environmental variable ADVENTOFCODE_SESSION: \"{}\"",
                e
            );
            std::process::exit(1);
        }
    };
    let cookie =
        reqwest::header::HeaderValue::from_str(format!("session={}", session).as_str()).unwrap();
    headers.insert("Cookie", cookie);
    Client::builder().default_headers(headers).build().unwrap()
}

fn download_input(client: &Client, day: Day) -> String {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day.0);
    let resp = client.get(url.as_str()).send().unwrap();
    if !resp.status().is_success() {
        eprintln!("Error when processing request:\n{}", resp.text().unwrap());
        std::process::exit(1);
    }
    resp.text().unwrap()
}

#[derive(Subcommand)]
enum Commands {
    Solve {
        data_dir: PathBuf,
        day_strings: Vec<String>,
    },
    Download {
        data_dir: PathBuf,
        day_strings: Vec<String>,
    }, // TODO: Add benchmark (reading, parsing, solving)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Solve {
            data_dir,
            day_strings,
        } => solve(&data_dir, &day_strings),
        Commands::Download {
            data_dir,
            day_strings,
        } => download(&data_dir, &day_strings),
    }
}
