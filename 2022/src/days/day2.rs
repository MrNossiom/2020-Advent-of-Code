use crate::solver::Solver;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PuzzleInput {
	A,
	B,
	C,
}

impl PuzzleInput {
	fn from_char(s: char) -> Self {
		match s {
			'A' | 'X' => Self::A,
			'B' | 'Y' => Self::B,
			'C' | 'Z' => Self::C,

			_ => unreachable!("invalid day 2 puzzle input"),
		}
	}

	const fn as_move(&self) -> ShiFumMiMove {
		match self {
			Self::A => ShiFumMiMove::Rock,
			Self::B => ShiFumMiMove::Paper,
			Self::C => ShiFumMiMove::Scissors,
		}
	}

	const fn as_plan(&self) -> ShiFumMiResult {
		match self {
			Self::A => ShiFumMiResult::Loss,
			Self::B => ShiFumMiResult::Draw,
			Self::C => ShiFumMiResult::Win,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiFumMiMove {
	Rock,
	Paper,
	Scissors,
}

impl ShiFumMiMove {
	fn my_score(&self, other: &Self) -> u8 {
		self.beats_score(other) + self.move_score()
	}

	fn beats_score(&self, other: &Self) -> u8 {
		if self.win_to() == *other {
			6
		} else if self == other {
			3
		} else {
			0
		}
	}

	const fn move_score(&self) -> u8 {
		match self {
			Self::Rock => 1,
			Self::Paper => 2,
			Self::Scissors => 3,
		}
	}

	const fn lose_to(&self) -> Self {
		match self {
			Self::Rock => Self::Paper,
			Self::Paper => Self::Scissors,
			Self::Scissors => Self::Rock,
		}
	}

	const fn win_to(&self) -> Self {
		match self {
			Self::Rock => Self::Scissors,
			Self::Paper => Self::Rock,
			Self::Scissors => Self::Paper,
		}
	}
}

pub enum ShiFumMiResult {
	Win,
	Loss,
	Draw,
}

pub struct Day2;

impl<'a> Solver<'a> for Day2 {
	type Parsed = Vec<(PuzzleInput, PuzzleInput)>;
	type Output = u32;

	fn parse(input: &'a str) -> Self::Parsed {
		input
			.split('\n')
			.map(|turn| {
				let mut chars = turn.chars();

				// Take the two chars around the space
				let first = PuzzleInput::from_char(chars.next().unwrap());
				let second = PuzzleInput::from_char(chars.nth(1).unwrap());

				(first, second)
			})
			.collect::<Vec<_>>()
	}

	fn part1(data: Self::Parsed) -> Self::Output {
		data.iter()
			.map(|(first, second)| (first.as_move(), second.as_move()))
			.map(|(opponent, prediction)| prediction.my_score(&opponent) as u32)
			.sum()
	}

	fn part2(data: Self::Parsed) -> Self::Output {
		data.iter()
			.map(|(opponent, plan)| {
				let play = match plan.as_plan() {
					ShiFumMiResult::Loss => opponent.as_move().win_to(),
					ShiFumMiResult::Draw => opponent.as_move(),
					ShiFumMiResult::Win => opponent.as_move().lose_to(),
				};

				(opponent.as_move(), play)
			})
			.map(|(opponent, prediction)| prediction.my_score(&opponent) as u32)
			.sum()
	}
}
