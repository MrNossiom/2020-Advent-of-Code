#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
	Looking,
	Op1,
	Op2,
}

fn parse_input(input: &[u8], golden: bool) -> u64 {
	let mut index = 0;
	let mut state = State::Looking;
	let mut result = 0u64;

	let mut op1 = 0u64;
	let mut op2 = 0u64;

	let mut can_mul = true;

	while index < input.len() {
		state = match (golden, state, input[index]) {
			(_, State::Looking, b'm') => {
				if index + "mul(_,_)".len() >= input.len() {
					break;
				}

				op1 = 0;
				op2 = 0;

				// index after m
				let i = index + 1;
				if input[i..i + 3] == *b"ul(" && can_mul {
					index += 3;
					State::Op1
				} else {
					State::Looking
				}
			}
			(true, State::Looking, b'd') => {
				if index + "do()mul(_,_)".len() >= input.len() {
					break;
				}
				let i = index + 1;
				if input[i..i + 3] == *b"o()" {
					index += 3;
					can_mul = true;
				} else if input[i..i + 6] == *b"on't()" {
					index += 6;
					can_mul = false;
				}
				State::Looking
			}
			(_, State::Op1, d @ b'0'..=b'9') => {
				op1 = op1 * 10 + (d - b'0') as u64;
				State::Op1
			}
			(_, State::Op1, b',') => State::Op2,
			(_, State::Op2, d @ b'0'..=b'9') => {
				op2 = op2 * 10 + (d - b'0') as u64;
				State::Op2
			}
			(_, State::Op2, b')') => {
				let res = op1 * op2;
				result += res;
				State::Looking
			}
			(_, state, chr) => State::Looking,
		};

		index += 1;
	}

	result
}

fn main() {
	let input = std::fs::read_to_string("../input/2024/day3.txt").unwrap();
	let input = input.as_bytes();

	let silver = dbg!(parse_input(&input, false));
	assert_eq!(silver, 170807108);

	let _golden = dbg!(parse_input(&input, true));
}
