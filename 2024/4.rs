const COLS: usize = 140;

// the needle must not overlap with it self
fn count_needle(haystack: Vec<u8>, needle: &[u8]) -> usize {
	let reverse_needle = needle.iter().rev().collect::<Vec<_>>();

	let mut total = 0usize;

	let mut lines_flow_count = [0usize; COLS];
	let mut lines_rev_count = [0usize; COLS];

	for line in haystack.chunks(COLS) {
		let mut flow_count = 0usize;
		let mut rev_count = 0usize;

		let mut index = 0;

		while index < line.len() {
			let chr = line[index];
			let mut should = false;
			// .zip(lines_flow_count.as_mut())
			// .zip(lines_rev_count.as_mut())
			if needle[flow_count..flow_count + 4] == *b"XMAS" {
				should = true;
			}
			if chr == needle[flow_count] {
				if flow_count == needle.len() - 1 {
					flow_count = 0;
					total += 1;
				} else {
					if should {
						dbg!(&needle[flow_count..flow_count + 4]);
					}

					flow_count += 1;
				}
			} else if flow_count != 0 && chr == needle[0] {
				flow_count = 1;
			} else {
				flow_count = 0;
			}
			index += 1;
		}
	}

	total
}

fn main() {
	let input = std::fs::read("../input/2024/day4.txt").unwrap();

	let _golden = dbg!(count_needle(input, b"XMAS"));
}
