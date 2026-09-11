// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use project_euler::problems::*;

fn main () {
    let args = Args::parse();

    match args.problem.parse::<u32>() {
        Ok(a) =>
            if a == 22 {
                p0022::run().expect("Problem 22 failed with an error");
            }
            else if let Some((_, run)) = PROBLEMS.iter().find(|(number, _)| *number == a) {
                run();
            }
            else {
                eprintln!("\nProblem '{}' does not have an implementation\n", a)
            }
        Err(e) => eprintln!("\nFailed to parse '{}' as a number: {}\n", args.problem, e),
    }
}

#[derive(Parser)]
#[command(name = "euler")]
struct Args {
    problem: String,
}

const PROBLEMS: [(u32, fn()); 25] = {
    [
        (1, p0001::run),
        (2, p0002::run),
        (3, p0003::run),
        (4, p0004::run),
        (5, p0005::run),
        (6, p0006::run),
        (7, p0007::run),
        (8, p0008::run),
        (9, p0009::run),
        (10, p0010::run),
        (11, p0011::run),
        (12, p0012::run),
        (13, p0013::run),
        (14, p0014::run),
        (15, p0015::run),
        (16, p0016::run),
        (17, p0017::run),
        (18, p0018::run),
        (19, p0019::run),
        (20, p0020::run),
        (21, p0021::run),
        (23, p0023::run),
        (24, p0024::run),
        (25, p0025::run),
        (26, p0026::run),
    ]
};
