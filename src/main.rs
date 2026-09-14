// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

fn main () {
    let args = Args::parse();

    match args.problem.parse::<u32>() {
        Ok(a) => run(a),
        Err(e) => eprintln!("\nFailed to parse '{}' as a number: {}\n", args.problem, e),
    }
}

fn run (problem: u32) {
    if let Some((_, run)) = PROBLEMS.iter().find(|(x, _)| *x == problem) {
        run();
    }
    else {
        eprintln!("\nProblem '{}' does not have an implementation\n", problem)
    }
}

#[derive(Parser)]
#[command(name = "euler")]
struct Args {
    problem: String,
}

macro_rules! problems {
    ($(($number:literal, $run:path)),* $(,)?) => {
        const PROBLEMS: &[(u32, fn())] = &[
            $(($number, $run)),*
        ];
    };
}

include!(concat!(env!("OUT_DIR"), "/problem_registry.rs"));
