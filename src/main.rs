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

#[derive(Parser)]
#[command(name = "euler")]
struct Args {
    problem: String,
}

///
/// This macro is used by the build script build.rs
/// to automate the mapping of problem numbers provided
/// on the command line to corresponding problem code.
///
macro_rules! problems {
    ($(($number:literal, $run:path)),* $(,)?) => {
        fn run (problem: u32) {
            match problem {
                $($number => $run(),)*
                problem => eprintln!("\nProblem '{}' does not have an implementation\n", problem),
            }
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/problem_registry.rs"));
