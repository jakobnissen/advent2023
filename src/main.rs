// TODO:
// Check for data dir's existence

mod days;

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use clap::{self, Parser, Subcommand};

type Boxes = (Box<dyn Display>, Box<dyn Display>);

fn to_boxed<F, A, B>(f: F, s: &str) -> Option<Boxes>
where
    F: Fn(&str) -> (A, B),
    A: Display + 'static,
    B: Display + 'static,
{
    let (a, b) = f(s);
    Some((Box::new(a), Box::new(b)))
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

fn solve_days(days: &[(Day, &str)]) -> Vec<Option<Boxes>> {
    days.iter()
        .map(|(day, s)| match day {
            Day(1) => to_boxed(days::day01::solve, s),
            _ => None,
        })
        .collect()
}

fn print_solution(day: Day, solution: Option<Boxes>) {
    println!("Day {:02}:", day.0);
    if let Some((a, b)) = solution {
        println!("  Part 1: {}", a);
        println!("  Part 2: {}\n", b);
    } else {
        println!("  Unimplemented!\n")
    }
}

#[derive(Subcommand)]
enum Commands {
    Solve {
        data_dir: PathBuf,
        day_strings: Vec<String>,
    },
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
        } => {
            // TODO: Validate data dir's existence
            let mut days: Vec<_> = day_strings
                .iter()
                .map(|i| Day::from_str(i.as_str()))
                .collect();
            days.sort_unstable();
            days.dedup();
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
    }
}
