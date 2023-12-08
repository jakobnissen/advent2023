// TODO: Add a proper error framework - anyhow?

mod days;

use std::{
    cell::OnceCell,
    fmt::Display,
    io::ErrorKind,
    path::{Path, PathBuf},
    time::Duration,
};

use clap::{self, Parser, Subcommand};
use reqwest::blocking::Client;

type TimedBoxes = (Duration, Box<dyn Display>, Box<dyn Display>);
type BoxedFn = Box<dyn Fn(&str) -> (Box<dyn Display>, Box<dyn Display>)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Day(u8);

// TODO: Proper error
impl Day {
    fn from_str(s: &str) -> Self {
        match s.parse::<u8>() {
            Err(_) => {
                eprintln!("Error: Cannot parse \"{s}\" as integer in 1-25.");
                std::process::exit(1);
            }
            Ok(n) => {
                if !(1..=25).contains(&n) {
                    eprintln!("Error: Day {n} not in 1-25.");
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

// TODO: Handle errors
fn load_days<'a, T>(
    dir: &Path,
    days_and_functions: &'a [(Day, Option<T>)],
) -> Vec<(Day, Option<(&'a T, String)>)> {
    days_and_functions
        .iter()
        .map(|(d, f)| {
            (
                *d,
                f.as_ref().map(|f| {
                    let path = &dir.join(format!("day{:02}.txt", d.0));
                    let string = match std::fs::read_to_string(path) {
                        Ok(s) => s,
                        Err(e) => {
                            if e.kind() == ErrorKind::NotFound {
                                eprintln!("Data file not found at path {:?}", path);
                                std::process::exit(1)
                            } else {
                                eprint!("Error when reading file:\n{}", e);
                                std::process::exit(1);
                            }
                        }
                    };
                    (f, string)
                }),
            )
        })
        .collect()
}

fn wrapper<F, R>(f: F) -> Option<BoxedFn>
where
    R: Display + 'static,
    F: Fn(&str) -> (R, R) + 'static,
{
    Some(Box::new(move |s| {
        let (a, b) = f(s);
        (Box::new(a), Box::new(b))
    }))
}

fn get_solver(day: Day) -> Option<BoxedFn> {
    match day {
        Day(1) => wrapper(days::day01::solve),
        Day(2) => wrapper(days::day02::solve),
        Day(3) => wrapper(days::day03::solve),
        Day(4) => wrapper(days::day04::solve),
        Day(5) => wrapper(days::day05::solve),
        Day(6) => wrapper(days::day06::solve),
        Day(7) => wrapper(days::day07::solve),
        _ => None,
    }
}

fn print_solution(day: Day, solution: Option<TimedBoxes>) {
    print!("Day {:02}", day.0);
    if let Some((duration, a, b)) = solution {
        println!(" [{:.2?}]:\n  Part 1: {}\n  Part 2: {}\n", duration, a, b)
    } else {
        println!(":\n  Unimplemented!\n")
    }
}

fn get_days(day_strings: Option<Vec<String>>, all: bool) -> Vec<Day> {
    // Parse the day strings into a list of days
    if all {
        if day_strings.is_some() {
            eprintln!("If --all days is set, individual days cannot be listed");
            std::process::exit(1);
        }
        (1..=25).map(Day).collect::<Vec<_>>()
    } else if let Some(v) = day_strings {
        parse_days(&v)
    } else {
        eprintln!("No days chosen");
        std::process::exit(1);
    }
}

fn solve(data_dir: &Path, day_strings: Option<Vec<String>>, all: bool) {
    // Parse the day strings into a list of days
    let days = get_days(day_strings, all);

    // Get the functions corresponding to the days, or None if the functions
    // have not been implemented
    let mut days_and_functions: Vec<_> = days.iter().map(|d| (*d, get_solver(*d))).collect();

    // If --all is picked, remove days and functions that are unimplemented,
    // such that it doesn't spam "unimplemented"
    if all {
        days_and_functions.retain(|x| x.1.is_some());
        if days_and_functions.is_empty() {
            eprintln!("No days implemented");
            std::process::exit(0)
        }
    }

    // At this point, we should have exited if there is no work to be done.
    // So, we can check the data directory since we know we need to load data
    assert!(!days_and_functions.is_empty());
    if !data_dir.is_dir() {
        eprintln!(
            "Data directory is not an existing directory: {:#?}",
            data_dir
        );
        std::process::exit(1)
    }

    // Load data for each implemented function. For unimplemented functions,
    // The data is never attempted to be loaded and is just None
    let data = load_days(data_dir, &days_and_functions);

    // For each day, if the solver+data is None, return unimplemented, else
    // run the solver on the data and record the time spent
    let solutions: Vec<Option<TimedBoxes>> = data
        .iter()
        .map(|(_, x)| {
            x.as_ref().map(|(f, data)| {
                let start = std::time::Instant::now();
                let (a, b) = f(data);
                (start.elapsed(), a, b)
            })
        })
        .collect();

    // Print the time taken for the solutions
    for (&day, solution) in days.iter().zip(solutions) {
        print_solution(day, solution)
    }
}

fn download(data_dir: &Path, day_strings: Option<Vec<String>>, all: bool) {
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
    let days = get_days(day_strings, all);
    let client_cell: OnceCell<Client> = OnceCell::new();
    for day in days.iter() {
        let path = data_dir.join(format!("day{:0>2}.txt", day.0));
        if path.exists() {
            println!("Input already exists: Day {:0>2}", day.0);
        } else {
            println!("Downloading day {:0>2}", day.0);
            let client = client_cell.get_or_init(make_client);
            if let Some(data) = download_input(client, *day) {
                std::fs::write(path, data).unwrap()
            } else {
                eprintln!("Day {:0>2} is not released yet!", day.0);
                if !all {
                    std::process::exit(1)
                }
                break;
            }
        }
    }
    // NOTE: If --all is passed, the day list is still untruncated here
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

// If the day is not released yet, return None
fn download_input(client: &Client, day: Day) -> Option<String> {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day.0);
    let resp = client.get(url.as_str()).send().unwrap();
    if !resp.status().is_success() {
        let text = resp.text().unwrap();
        if text.contains("Please don't repeatedly request this endpoint before it unlocks") {
            return None;
        } else {
            eprintln!("Error when processing request:\n{}", text);
            std::process::exit(1);
        }
    }
    Some(resp.text().unwrap())
}

#[derive(Subcommand)]
enum Commands {
    Solve {
        data_dir: PathBuf,
        day_strings: Option<Vec<String>>,
        #[arg(long)]
        all: bool,
    },
    Download {
        data_dir: PathBuf,
        day_strings: Option<Vec<String>>,
        #[arg(long)]
        all: bool,
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
            all,
        } => solve(&data_dir, day_strings, all),
        Commands::Download {
            data_dir,
            day_strings,
            all,
        } => download(&data_dir, day_strings, all),
    }
}
