use crate::solver::Solver;
use std::collections::HashSet;

fn detect_start_sequence(packet: &str, width: usize) -> usize {
	let buffer = packet.as_bytes();
	for (counter, chars) in buffer.windows(width).enumerate() {
		let hs = chars.iter().collect::<HashSet<_>>();

		if hs.len() == width {
			return counter + width;
		}
	}

	unreachable!();
}

pub struct Day6;

impl<'a> Solver<'a> for Day6 {
	type Parsed = &'a str;
	type Output = usize;

	fn parse(input: &'a str) -> Self::Parsed {
		input
	}

	fn part1(data: Self::Parsed) -> Self::Output {
		detect_start_sequence(data, 4)
	}

	fn part2(data: Self::Parsed) -> Self::Output {
		detect_start_sequence(data, 14)
	}
}
