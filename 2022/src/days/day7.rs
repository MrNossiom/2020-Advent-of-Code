use crate::{solver::Solver, util::HashMap};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub enum Command<'a> {
	List(HashMap<&'a str, Option<u32>>),
	ChangeDirectory(&'a str),
}

impl<'a> FromStr for Command<'a> {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		match lines.next().unwrap().split_once(' ') {
			Some(("cd", args)) => Ok(Command::ChangeDirectory(args)),
			None => {
				let mut entries = HashMap::new();

				for line in lines {
					let (size, name) = line.split_once(' ').unwrap();

					let size: Option<u32> = match size {
						"dir" => None,
						_ => Some(size.parse().unwrap()),
					};

					entries.insert(name, size);
				}

				Ok(Command::List(entries))
			}
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum FileType<'a> {
	Directory {
		size: Option<u32>,
		files: HashMap<&'a str, FileType<'a>>,
	},
	File {
		size: u32,
	},
}

impl<'a> FromIterator<Command<'a>> for FileType<'a> {
	fn from_iter<T: IntoIterator<Item = Command<'a>>>(iter: T) -> Self {
		let root = FileType::Directory {
			size: None,
			files: Default::default(),
		};

		for cmd in iter {
			match cmd {
				Command::List(hm) => root.files,
				Command::ChangeDirectory(dir) => {}
			}
		}

		todo!()
	}
}

pub struct Day7;

impl<'a> Solver<'a> for Day7 {
	type Parsed = FileType<'a>;
	type Output = usize;

	fn parse(input: &'a str) -> Self::Parsed {
		let mut raw_commands = input.split("\n$ ");

		// Ignore first command, `cd /`
		raw_commands.next();

		let commands = raw_commands.map(|rcmd| Command::from_str(rcmd).unwrap());

		FileType::from_iter(commands)
	}

	fn part1(data: Self::Parsed) -> Self::Output {
		let base_path = PathBuf::from("/");

		for cmd in data {
			match cmd {
				Command::List(hm) => {}
				Command::ChangeDirectory(dir) => {}
			}
		}

		todo!();
	}

	fn part2(_data: Self::Parsed) -> Self::Output {
		todo!();
	}
}
