#[derive(Debug, PartialEq)]
enum Safety {
	Unsafe,
	Unknown,
	Flat(i32),
	Increasing(i32),
	Decreasing(i32),
}

fn is_report_safe(report: Vec<i32>) -> bool {
	let safety = report
		.into_iter()
		.fold(Safety::Unknown, |acc, e| match acc {
			Safety::Unsafe => Safety::Unsafe,
			Safety::Unknown => Safety::Flat(e),
			Safety::Flat(old) => match old - e {
				-3..=-1 => Safety::Decreasing(e),
				1..=3 => Safety::Increasing(e),
				_ => Safety::Unsafe,
			},
			Safety::Increasing(old) => match old - e {
				1..=3 => Safety::Increasing(e),
				_ => Safety::Unsafe,
			},
			Safety::Decreasing(old) => match e - old {
				1..=3 => Safety::Decreasing(e),
				_ => Safety::Unsafe,
			},
		});

	safety != Safety::Unsafe
}

fn main() {
	let input = std::fs::read_to_string("../input/2024/day2.txt").unwrap();

	let reports = input
		.lines()
		.map(|report| {
			report
				.split_whitespace()
				.map(|n| n.parse::<i32>().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let silver = reports
		.iter()
		.filter(|report| is_report_safe((**report).clone()))
		.count();

	dbg!(silver);

	let golden = reports
		.iter()
		.filter(|report| {
			(0..report.len())
				.into_iter()
				.map(|i| {
					let mut report = (**report).clone();
					report.remove(i);
					report
				})
				.any(is_report_safe)
		})
		.count();

	dbg!(golden);
}
