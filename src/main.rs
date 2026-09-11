// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use project_euler::problems::*;

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
    ($(($number:literal, $module:ident)),* $(,)?) => {
        const PROBLEMS: &[(u32, fn())] = &[
            $(($number, $module::run)),*
        ];
    };
}

problems!(
    (1, p0001), (2, p0002), (3, p0003), (4, p0004), (5, p0005), (6, p0006), (7, p0007), (8, p0008), (9, p0009), (10, p0010),
    (11, p0011), (12, p0012), (13, p0013), (14, p0014), (15, p0015), (16, p0016), (17, p0017), (18, p0018), (19, p0019),
    (20, p0020),
    (21, p0021),
    (22, p0022),
    (23, p0023),
    (24, p0024),
    (25, p0025),
    (26, p0026),
);
