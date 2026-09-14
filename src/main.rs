// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

fn main () {
    let args = Args::parse();

    match args.problem.parse::<u32>() {
        Ok(a) => project_euler::run(a),
        Err(e) => eprintln!("\nFailed to parse '{}' as a number: {}\n", args.problem, e),
    }
}

#[derive(Parser)]
#[command(name = "euler")]
struct Args {
    problem: String,
}
