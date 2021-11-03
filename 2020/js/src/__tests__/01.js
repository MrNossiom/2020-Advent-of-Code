import { getLinesOfPuzzleInput } from '../../utils/parseInput.js';
import { firstStar, secondStar } from '../01.js';
import ava from 'ava';

const input = (await getLinesOfPuzzleInput('01')).map((number) =>
	parseInt(number, 10)
);

ava('Day 01 - First Star', (t) => {
	t.is(firstStar(input), '1301 + 719');
});

ava('Day 01 - Second Star', (t) => {
	t.is(secondStar(input), '889 + 1079 + 52');
});
