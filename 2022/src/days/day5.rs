use crate::solver::Solver;

/// A move directive with a number of crates, a source and a destination.

#[derive(Debug, Clone)]
pub struct Move {
	pub num_to_move: u8,
	pub from: usize,
	pub to: usize,
}

#[derive(Debug, Clone)]
pub struct Krate(char);

pub struct Day5;

impl<'a> Solver<'a> for Day5 {
	type Parsed = (Vec<Vec<Krate>>, Vec<Move>);
	type Output = String;

	fn parse(input: &'a str) -> Self::Parsed {
		let (layout, moves) = input.split_once("\n\n").unwrap();

		let layout = {
			let mut iter = layout.lines();
			iter.next_back();
			iter
		}
		.map(|line| {
			({
				let mut iter = line.chars();
				iter.next();
				iter
			})
			.step_by(4)
			.map(|c| if c == ' ' { None } else { Some(Krate(c)) })
		})
		.rev();

		let mut stacks: Vec<Vec<Krate>> = vec![];
		for line in layout {
			for (i, krate) in line.enumerate() {
				if let Some(krate) = krate {
					if let Some(stack) = stacks.get_mut(i) {
						stack.push(krate);
					} else {
						stacks.insert(i, vec![krate]);
					}
				}
			}
		}

		let moves = moves
			.split('\n')
			.map(|directive| {
				let mut steps = directive.split(' ');

				steps.next(); // `move`
				let num_to_move = steps.next().unwrap().parse().unwrap();

				steps.next(); // `from`
				let from = steps.next().unwrap().parse().unwrap();

				steps.next(); // `to`
				let to = steps.next().unwrap().parse().unwrap();

				Move {
					num_to_move,
					from,
					to,
				}
			})
			.collect::<Vec<_>>();

		(stacks, moves)
	}

	fn part1((mut stacks, moves): Self::Parsed) -> Self::Output {
		for Move {
			num_to_move,
			from,
			to,
		} in moves
		{
			for _ in 0..num_to_move {
				let krate = stacks[from - 1].pop().unwrap();
				stacks[to - 1].push(krate);
			}
		}

		stacks
			.iter()
			.map(|vec| vec.last().unwrap().0)
			.collect::<String>()
	}

	fn part2((mut stacks, moves): Self::Parsed) -> Self::Output {
		for Move {
			num_to_move,
			from,
			to,
		} in moves
		{
			let mut stack = vec![];

			for _ in 0..num_to_move {
				let krate = stacks[from - 1].pop().unwrap();
				stack.push(krate);
			}
			for _ in 0..num_to_move {
				let krate = stack.pop().unwrap();
				stacks[to - 1].push(krate);
			}
		}

		stacks
			.iter()
			.map(|vec| vec.last().unwrap().0)
			.collect::<String>()
	}
}
