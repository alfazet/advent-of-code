use std::{env, fs, time};

mod part1;
mod part2;

const DAY: u8 = 4;
const TITLE: &str = "Ceres Search";

fn usage() {
    println!("Advent of Code 2024, Day {}: {}", DAY, TITLE);
    println!("Usage: day-{:0>2} {{-1|-2}} <input_file>", DAY);
}

fn solve_and_bench(solver: fn(String), input: String) {
    println!();
    let timer = time::Instant::now();
    solver(input);
    println!();
    println!("Elapsed: {} ms", timer.elapsed().as_millis());
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        usage();
    } else {
        let input = fs::read_to_string(&args[2]).expect("read_to_string");
        match args[1].as_str() {
            "-1" => solve_and_bench(part1::solve, input),
            "-2" => solve_and_bench(part2::solve, input),
            _ => usage(),
        };
    }
}
