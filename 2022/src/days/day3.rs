use crate::solver::Solver;
use std::str::Split;

fn char_score(byte: u8) -> u32 {
	match byte {
		b'a'..=b'z' => (byte - b'a' + 1) as u32,
		b'A'..=b'Z' => (byte - b'A' + 26 + 1) as u32,

		_ => unreachable!("invalid day 3 puzzle input"),
	}
}

pub struct Day3;

impl<'a> Solver<'a> for Day3 {
	type Parsed = Split<'a, char>;
	type Output = u32;

	fn parse(input: &'a str) -> Self::Parsed {
		input.split('\n')
	}

	fn part1(data: Self::Parsed) -> Self::Output {
		data.map(|line| line.as_bytes().split_at(line.len() / 2))
			.map(|(one, two)| *one.iter().find(|c| two.contains(c)).unwrap())
			.map(char_score)
			.sum()
	}

	fn part2(data: Self::Parsed) -> Self::Output {
		data.array_chunks::<3>()
			.map(|[one, two, three]| [one.as_bytes(), two.as_bytes(), three.as_bytes()])
			.map(|[one, two, three]| {
				*one.iter()
					.find(|c| two.contains(c) && three.contains(c))
					.unwrap()
			})
			.map(char_score)
			.sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_char_score() {
		assert_eq!(char_score(b'a'), 1);
		assert_eq!(char_score(b'A'), 27);
		assert_eq!(char_score(b'z'), 26);
		assert_eq!(char_score(b'Z'), 52);
	}
}
