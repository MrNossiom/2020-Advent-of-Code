import { getBlocksOfPuzzleInput } from '../../utils/parseInput.js';
import { firstStar, secondStar } from '../04.js';
import ava from 'ava';

const input = await getBlocksOfPuzzleInput('04');

ava('Day 04 - First Star', (t) => {
	t.is(firstStar(input), 260);
});

ava('Day 04 - Second Star', (t) => {
	t.is(secondStar(input), 0);
});
