use std::collections::HashSet;
use std::fmt;
use std::io::{self, Write as _};
use std::ops;

const WORLD_LEN: usize = 130;
// const WORLD_LEN: usize = 10;
const WORLD_SIZE: usize = WORLD_LEN * WORLD_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IVec2(i32, i32);

impl ops::Add<IVec2> for IVec2 {
	type Output = IVec2;

	fn add(self, other: IVec2) -> Self::Output {
		Self(self.0 + other.0, self.1 + other.1)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
	Empty,
	Obstacle,
	Visited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn turn_right(&mut self) {
		*self = match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}
}

impl Direction {
	fn delta(&self) -> IVec2 {
		match self {
			Self::Up => IVec2(0, -1),
			Self::Right => IVec2(1, 0),
			Self::Down => IVec2(0, 1),
			Self::Left => IVec2(-1, 0),
		}
	}
}

#[derive(Clone)]
struct World([Cell; WORLD_SIZE]);

impl World {
	fn new(base: [Cell; WORLD_SIZE]) -> Self {
		Self(base)
	}

	fn get(&self, IVec2(x, y): IVec2) -> &Cell {
		&self.0[y as usize * WORLD_LEN + x as usize]
	}

	fn get_mut(&mut self, IVec2(x, y): IVec2) -> &mut Cell {
		&mut self.0[y as usize * WORLD_LEN + x as usize]
	}

	fn pos_in_bounds(&self, IVec2(x, y): IVec2) -> bool {
		x >= 0 && x < WORLD_LEN as i32 && y >= 0 && y < WORLD_LEN as i32
	}
}

impl fmt::Display for World {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0..WORLD_LEN {
			for x in 0..WORLD_LEN {
				match self.get(IVec2(x as i32, y as i32)) {
					Cell::Empty => write!(f, ".")?,
					Cell::Obstacle => write!(f, "#")?,
					Cell::Visited => write!(f, "X")?,
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

// W is N²
fn parse_world(input: Vec<u8>) -> (World, IVec2) {
	let mut pos = None;

	let mut line = 0;

	let world = input
		.into_iter()
		.enumerate()
		.filter_map(|(i, byte)| match byte {
			b'.' => Some(Cell::Empty),
			b'#' => Some(Cell::Obstacle),
			b'^' => {
				// account for newlines
				pos = Some(IVec2(i as i32 - (line * (WORLD_LEN + 1) as i32), line));
				Some(Cell::Empty)
			}
			b'\n' => {
				line += 1;
				None
			}
			_ => unreachable!(),
		})
		.collect::<Vec<_>>();

	let world = World::new(world.try_into().unwrap());

	(world, pos.unwrap())
}

fn simulate_world(mut world: World, guard_pos: IVec2) -> (HashSet<IVec2>, bool) {
	let mut visited = HashSet::<IVec2>::new();

	let mut dir_visited = HashSet::<(IVec2, Direction)>::new();

	visited.insert(guard_pos);
	*world.get_mut(guard_pos) = Cell::Visited;

	let mut current_pos = guard_pos.clone();
	let mut direction = Direction::Up;

	let mut next_pos = current_pos + direction.delta();

	let mut is_looping = false;

	while world.pos_in_bounds(next_pos) && !is_looping {
		let cell = world.get(next_pos).clone();
		match cell {
			Cell::Empty | Cell::Visited => {
				if cell == Cell::Visited && dir_visited.contains(&(next_pos, direction)) {
					is_looping = true;
				}

				visited.insert(next_pos);
				dir_visited.insert((next_pos, direction));
				*world.get_mut(next_pos) = Cell::Visited;

				current_pos = next_pos;
				next_pos = current_pos + direction.delta();
			}
			Cell::Obstacle => {
				direction.turn_right();
				next_pos = current_pos + direction.delta();
			}
		}
	}

	(visited, is_looping)
}

fn simulate_blocus(mut world: World, guard_pos: IVec2, visited: HashSet<IVec2>) -> usize {
	let mut nb_of_blocuses = 0;

	for (i, pos) in visited.into_iter().enumerate() {
		if pos == guard_pos {
			continue;
		}

		let mut world = world.clone();
		*world.get_mut(pos) = Cell::Obstacle;

		let (_, is_looping) = simulate_world(world, guard_pos);
		if is_looping {
			nb_of_blocuses += 1;
		}

		// report progress
		print!("\r{i}");
		io::stdout().flush().unwrap();
	}

	nb_of_blocuses
}

fn main() {
	let input = std::fs::read("../input/2024/day6.txt").unwrap();
	// let input = std::fs::read("../input/2024/day6.sample.txt").unwrap();

	let (mut world, guard_pos) = parse_world(input);

	let (visited, _) = simulate_world(world.clone(), guard_pos);
	let nb_of_visited = visited.len();

	let golden = dbg!(simulate_blocus(world, guard_pos, visited));

	println!("silver: {}, golden: {}", nb_of_visited, golden);
}
