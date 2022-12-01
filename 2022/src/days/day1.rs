use crate::solver::Solver;

pub struct Day1;

impl<'a> Solver<'a> for Day1 {
	type Parsed = Vec<u32>;
	type Output = u32;

	fn parse(input: &'a str) -> Self::Parsed {
		input
			.split("\n\n")
			.map(|block| {
				block
					.split('\n')
					.map(|line| line.parse().unwrap())
					.collect::<Vec<_>>()
			})
			.map(|block| block.iter().sum())
			.collect::<Vec<_>>()
	}

	fn part1(data: Self::Parsed) -> Self::Output {
		// Find the largest element
		data.into_iter().fold(std::u32::MIN, Ord::max)
	}

	fn part2(mut data: Self::Parsed) -> Self::Output {
		data.sort_unstable();

		// Take the 3 last elements and add them together
		data[data.len() - 3..].iter().sum()
	}
}
