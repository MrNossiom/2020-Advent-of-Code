fn foo(mut stack: Vec<u64>) -> Vec<u64> {
	let next = stack.pop().unwrap();
	if stack.is_empty() {
		vec![next]
	} else {
		foo(stack)
			.into_iter()
			.map(|n| [n * next, n + next])
			.flatten()
			.collect()
	}
}

fn bar(mut stack: Vec<u64>) -> Vec<u64> {
	let next = stack.pop().unwrap();
	if stack.is_empty() {
		vec![next]
	} else {
		bar(stack)
			.into_iter()
			.map(|n| {
				let conc = format!("{n}{next}").parse().unwrap();
				[n * next, n + next, conc]
			})
			.flatten()
			.collect()
	}
}

fn solve_equations(equations: Vec<(u64, Vec<u64>)>) -> u64 {
	let mut count = 0;

	for (target, stack) in equations {
		let possibilites = foo(stack);
		if possibilites.into_iter().any(|num| num == target) {
			count += target;
		}
	}

	count as u64
}

fn solve_equations_two(equations: Vec<(u64, Vec<u64>)>) -> u64 {
	let mut count = 0;

	for (target, stack) in equations {
		if bar(stack).into_iter().any(|num| num == target) {
			count += target;
		}
	}

	count as u64
}

fn main() {
	let input = std::fs::read_to_string("../input/2024/day7.txt").unwrap();
	// let input = std::fs::read_to_string("../input/2024/day7.sample.txt").unwrap();

	let equations = input
		.lines()
		.map(|line| {
			let (ret, ops) = line.split_once(": ").unwrap();
			let ret = ret.parse().unwrap();
			let ops = ops.split_whitespace().map(|n| n.parse().unwrap()).collect();
			(ret, ops)
		})
		.collect::<Vec<(u64, Vec<u64>)>>();

	let _silver = dbg!(solve_equations(equations.clone()));
	let _golden = dbg!(solve_equations_two(equations));
}
