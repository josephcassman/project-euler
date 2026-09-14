// Copyright 2026 Joseph Cassman
// SPDX-License-Identifier: Apache-2.0

pub mod etc;
pub mod problems;

macro_rules! problems {
	($(($number:literal, $run:path)),* $(,)?) => {
		pub fn run (problem: u32) {
			match problem {
				$($number => $run(),)*
				problem => eprintln!("\nProblem '{}' does not have an implementation\n", problem),
			}
		}
	};
}

include!(concat!(env!("OUT_DIR"), "/problem_registry.rs"));
