fn main() {
	let input = std::fs::read_to_string("../input/2024/day1.txt").unwrap();

	let (mut l, mut r) = input
		.lines()
		.map(|line| line.split_once("   ").unwrap())
		.map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
		.unzip::<_, _, Vec<_>, Vec<_>>();

	l.sort();
	r.sort();

	let _silver: i32 = dbg!(l.iter().zip(r.iter()).map(|(l, r)| (l - r).abs()).sum());
	let _golden: i32 = dbg!(l
		.iter()
		.map(|num| num * r.iter().filter(|el| *el == num).count() as i32)
		.sum());
}
