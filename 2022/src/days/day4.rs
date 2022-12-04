use crate::solver::Solver;
use std::ops::RangeInclusive;

fn is_range_included_in_the_other(
	range1: &RangeInclusive<u8>,
	range2: &RangeInclusive<u8>,
) -> bool {
	range1.start() >= range2.start() && range1.end() <= range2.end()
}

fn does_range_overlap_with_the_other(
	range1: &RangeInclusive<u8>,
	range2: &RangeInclusive<u8>,
) -> bool {
	range1.start() <= range2.end() && range1.end() >= range2.start()
}

pub struct Day4;

impl<'a> Solver<'a> for Day4 {
	type Parsed = Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>;
	type Output = u32;

	fn parse(input: &'a str) -> Self::Parsed {
		input
			.split('\n')
			.map(|line| {
				let mut ranges = line.split(',').map(|range| {
					let mut bounds = range.split('-').map(|num| num.parse::<u8>().unwrap());

					bounds.next().unwrap()..=bounds.next().unwrap()
				});

				(ranges.next().unwrap(), ranges.next().unwrap())
			})
			.collect()
	}

	fn part1(data: Self::Parsed) -> Self::Output {
		data.iter()
			.map(|(range_one, range_two)| {
				u32::from(
					is_range_included_in_the_other(range_one, range_two)
						|| is_range_included_in_the_other(range_two, range_one),
				)
			})
			.sum()
	}

	fn part2(data: Self::Parsed) -> Self::Output {
		data.iter()
			.map(|(range_one, range_two)| {
				u32::from(
					does_range_overlap_with_the_other(range_one, range_two)
						|| does_range_overlap_with_the_other(range_two, range_one),
				)
			})
			.sum()
	}
}
