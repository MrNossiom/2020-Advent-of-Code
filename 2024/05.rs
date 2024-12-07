use std::collections::HashMap;

fn check_ordering_in_updates(updates: &Vec<Vec<u16>>, ordering: &HashMap<u16, Vec<u16>>) -> u64 {
	let mut count = 0;

	for update in updates {
		let is_correctly_ordered = update.iter().enumerate().all(|(i, elem)| {
			let Some(matching_pages) = ordering.get(elem) else {
				return true;
			};

			for matching_page in matching_pages {
				let mut j = 0;
				while j < i {
					if update[j] == *matching_page {
						return false;
					}
					j += 1;
				}
			}

			return true;
		});

		if is_correctly_ordered {
			count += u64::from(update[update.len() / 2]);
		}
	}

	count
}

fn main() {
	let input = std::fs::read_to_string("../input/2024/day5.txt").unwrap();
	// let input = std::fs::read_to_string("../input/2024/day5.sample.txt").unwrap();

	let (ordering_input, update_input) = input.split_once("\n\n").unwrap();

	let ordering_rules = ordering_input
		.lines()
		.map(|line| {
			let (a, b) = line.split_once("|").unwrap();
			(a.parse().unwrap(), b.parse().unwrap())
		})
		.fold::<HashMap<u16, Vec<u16>>, _>(HashMap::new(), |mut hm, (a, b)| {
			hm.entry(a).or_default().push(b);
			hm
		});

	let updates = update_input
		.lines()
		.map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
		.collect::<Vec<Vec<u16>>>();

	let _silver = dbg!(check_ordering_in_updates(&updates, &ordering_rules));
}
